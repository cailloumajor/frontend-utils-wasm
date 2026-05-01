use std::fs::{self, File};
use std::io::{self, BufReader};
use std::path::PathBuf;
use std::process::Stdio;

use anyhow::{Context as _, anyhow};
use askama::Template;
use base64::engine::general_purpose::STANDARD_NO_PAD;
use base64::write::EncoderWriter;
use cargo_metadata::Message;
use clap::Args;
use duct::cmd;
use escargot::CargoBuild;
use flate2::Compression;
use flate2::write::GzEncoder;
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

        eprintln!("\nCreating the output directory if needed");
        fs::create_dir_all(&self.out_dir).context("Failed to create the output directory")?;

        let stem = built_wasm_path
            .file_stem()
            .context("Invalid built WASM file name")?;

        let js_glue_filename = format!("{stem}_bg.js");
        let js_glue_path = self.out_dir.join(&js_glue_filename);

        let js_entry_path = self.out_dir.join(stem).with_extension("js");

        let ts_filename = format!("{stem}.d.ts");
        let ts_path = self.out_dir.join(&ts_filename);

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
        let base64_encoder = EncoderWriter::new(Vec::new(), &STANDARD_NO_PAD);
        let mut gz_encoder = GzEncoder::new(base64_encoder, Compression::best());
        io::copy(&mut optimized_reader, &mut gz_encoder)
            .context("Failed to copy optimized WASM to Base64 encoder")?;
        let wasm_encoded = gz_encoder
            .finish()
            .context("Failed to finish gzip encoder")?
            .finish()
            .context("Failed to finish Base64 encoder")?;

        eprintln!("\nWriting the JS entry file to the output directory");
        let base64_lines = wasm_encoded
            .chunks(BASE64_COLUMN_WIDTH)
            .map(|chunk| str::from_utf8(chunk).expect("Base64 should be valid UTF-8"))
            .collect::<Vec<_>>();
        let mut js_entry_file =
            File::create(&js_entry_path).context("Failed to create the JS entry file")?;
        let js_entry = JsEntry {
            types_file: &ts_filename,
            glue_file: &js_glue_filename,
            base64_lines: &base64_lines,
        };
        js_entry
            .write_into(&mut js_entry_file)
            .context("Failed to write JS entry file")?;

        eprintln!("\nWriting the types definitions file to the output");
        let ts = bindgen_out
            .ts()
            .context("Missing generated TypeScript definitions")?;
        fs::write(&ts_path, ts).context("Failed to write the types definitions file")?;

        eprintln!("\nFormatting written files");
        cmd!(
            "biome",
            "format",
            "--write",
            js_glue_path,
            js_entry_path,
            ts_path
        )
        .run()
        .context("Failed to format written files")?;

        Ok(())
    }
}

/// The JS entry file template definition.
#[derive(Template)]
#[template(path = "entry.js", escape = "none")]
struct JsEntry<'a> {
    types_file: &'a str,
    glue_file: &'a str,
    base64_lines: &'a [&'a str],
}
