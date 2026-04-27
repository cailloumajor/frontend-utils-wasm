use std::fs::{self, File};
use std::io::{self, BufReader, Write};
use std::path::PathBuf;
use std::process::Stdio;

use anyhow::{Context, anyhow};
use base64::engine::general_purpose::STANDARD_NO_PAD;
use base64::write::EncoderWriter;
use cargo_metadata::Message;
use clap::Args;
use duct::cmd;
use escargot::CargoBuild;
use indoc::writedoc;
use wasm_bindgen_cli_support::Bindgen;

/// The target triplet to build for.
const RUST_TARGET: &str = "wasm32-unknown-unknown";

/// The column width for wrapped base64 string.
const BASE64_COLUMN_WIDTH: usize = 120;

#[derive(Args)]
pub(crate) struct BuildDeno {
    /// The output directory.
    #[arg(long, default_value = "pkg")]
    out_dir: PathBuf,
}

impl BuildDeno {
    pub(crate) fn run(&self) -> anyhow::Result<()> {
        let wasmopt_version = cmd!("wasm-opt", "--version")
            .read()
            .context("`wasm-opt` tool not found")?;
        eprintln!("\nFound {wasmopt_version}");

        eprintln!("\nBuilding the Rust library for `{RUST_TARGET}` target");

        let mut cargo_child = CargoBuild::new()
            .release()
            .target(RUST_TARGET)
            .into_command()
            .stdout(Stdio::piped())
            .spawn()
            .context("Failed to run docker build")?;

        let stdout = cargo_child
            .stdout
            .take()
            .expect("child stdout should have been captured");

        let mut built_wasm_path = None;

        for item in Message::parse_stream(BufReader::new(stdout)) {
            let message = item.context("Failed to parse cargo message")?;
            match message {
                Message::CompilerArtifact(artifact) => {
                    for artifact_file in artifact
                        .filenames
                        .into_iter()
                        .filter(|f| f.extension().is_some_and(|ext| ext == "wasm"))
                    {
                        let old = built_wasm_path.replace(artifact_file);
                        if old.is_some() {
                            return Err(anyhow!("Build returned more than one WASM file"));
                        }
                    }
                }
                Message::CompilerMessage(msg) => {
                    eprintln!("{msg}");
                }
                Message::TextLine(text) => {
                    eprintln!("{text}");
                }
                _ => {}
            }
        }

        let built_wasm_path = built_wasm_path.context("Build did not generate any WASM file")?;

        eprintln!("\nRunning wasm-bindgen");
        let mut bindgen_out = Bindgen::new()
            .input_path(&built_wasm_path)
            .bundler(true)
            .context("Failed to select wasm-bindgen output mode")?
            .typescript(true)
            .generate_output()
            .context("Failed to generate wasm-bindgen output")?;

        let stem = built_wasm_path
            .file_stem()
            .context("Invalid built WASM file name")?;

        eprintln!("\nCreating the output directory if needed");
        fs::create_dir_all(&self.out_dir).context("Failed to create the output directory")?;

        let js_glue_path = self.out_dir.join(format!("{stem}_bg.js"));
        let js_glue_filename = js_glue_path
            .file_name()
            .expect("JS glue file path should have a file name");
        let js_path = self.out_dir.join(stem).with_extension("js");
        let ts_path = js_path.with_extension("d.ts");
        let ts_filename = ts_path
            .file_name()
            .expect("TS file path should have a file name");

        eprintln!("\nWriting the JS glue file to the output directory");
        let js_glue = bindgen_out
            .start()
            .context("Missing JS glue in generated output")?;
        fs::write(&js_glue_path, js_glue).context("Failed to write JS glue file")?;

        eprintln!("\nOptimizing generated WASM and encoding to Base64");
        let mut optimized_reader = cmd!("wasm-opt", "-Oz", "-o", "/dev/stdout")
            .stdin_bytes(bindgen_out.wasm_mut().emit_wasm())
            .reader()
            .context("Failed to optimize WASM")?;
        let mut encoder = EncoderWriter::new(Vec::new(), &STANDARD_NO_PAD);
        io::copy(&mut optimized_reader, &mut encoder)
            .context("Failed to copy optimized WASM to Base64 encoder")?;
        let wasm_encoded = encoder
            .finish()
            .context("Failed to finish Base64 encoder")?;
        let wasm_base64_lines = wasm_encoded
            .chunks(BASE64_COLUMN_WIDTH)
            .map(|bytes_chunk| {
                str::from_utf8(bytes_chunk).expect("Base64 bytes should be valid UTF-8")
            })
            .collect::<Vec<_>>();
        let wasm_b64_formatted = wasm_base64_lines.join("\\\n");

        eprintln!("\nWriting the JS entry file to the output directory");
        let mut js_file = File::create(&js_path).context("Failed to create the JS entry file")?;
        writedoc!(
            &mut js_file,
            r#"
                // @ts-self-types="./{0}"

                import * as imports from "./{1}"
                import {{ __wbg_set_wasm }} from "./{1}"

                // deno-fmt-ignore
                const bytes = Uint8Array.fromBase64("\
                {wasm_b64_formatted}\
                ")

                const wasmModule = new WebAssembly.Module(bytes)
                const wasmInstance = new WebAssembly.Instance(wasmModule, {{ "./{1}": imports }})

                __wbg_set_wasm(wasmInstance.exports)
                wasmInstance.exports.__wbindgen_start()

                export * from "./{1}"
            "#,
            ts_filename.display(),
            js_glue_filename.display(),
        )
        .context("Failed to write to the JS entry file")?;

        eprintln!("\nWriting the types definitions file to the output");
        let ts = bindgen_out
            .ts()
            .context("Missing generated TypeScript definitions")?;
        fs::write(&ts_path, ts).context("Failed to write the types definitions file")?;

        eprintln!("\nFormatting written files");
        cmd!("deno", "fmt", js_glue_path, js_path, ts_path)
            .run()
            .context("Failed to format written files")?;

        Ok(())
    }
}
