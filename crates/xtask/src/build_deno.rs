use std::fs;
use std::io::BufReader;
use std::path::PathBuf;
use std::process::Stdio;

use anyhow::{Context, anyhow};
use cargo_metadata::Message;
use clap::Args;
use duct::cmd;
use escargot::CargoBuild;
use wasm_bindgen_cli_support::Bindgen;

/// The target triplet to build for.
const RUST_TARGET: &str = "wasm32-unknown-unknown";

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
        eprintln!("Found {wasmopt_version}");

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
            .deno(true)
            .context("Failed to enable wasm-bindgen Deno build")?
            .typescript(true)
            .generate_output()
            .context("Failed to generate wasm-bindgen output")?;

        let stem = built_wasm_path
            .file_stem()
            .context("Invalid built WASM file name")?;

        eprintln!("\nCreating the output directory if needed");
        fs::create_dir_all(&self.out_dir).context("Failed to create the output directory")?;

        eprintln!("\nWriting the WASM file to the output directory");
        let wasm_path = self.out_dir.join(format!("{stem}_bg.wasm"));
        bindgen_out
            .wasm_mut()
            .emit_wasm_file(&wasm_path)
            .context("Failed to write WASM file to the output directory")?;

        let js_path = self.out_dir.join(stem).with_extension("js");
        let ts_path = js_path.with_extension("d.ts");

        eprintln!("\nWriting the types definitions file to the output");
        let ts = bindgen_out
            .ts()
            .context("Missing generated TypeScript definitions")?;
        fs::write(&ts_path, ts).context("Failed to write the types definitions file")?;

        eprintln!("\nWriting the JS file to the output directory");
        fs::write(&js_path, bindgen_out.js().as_bytes())
            .context("Failed to write JS content to the JS file")?;

        eprintln!("Formatting written files");
        cmd!("deno", "fmt", js_path, ts_path)
            .run()
            .context("Failed to format written files")?;

        eprintln!("\nOptimizing generated WASM");
        let opt_path = wasm_path.with_extension("wasm-opt.wasm");
        cmd!("wasm-opt", "-Oz", "-o", &opt_path, &wasm_path)
            .run()
            .context("Failed to optimize WASM")?;
        fs::rename(opt_path, wasm_path)
            .context("Failed to overwrite WASM with optimized version")?;

        Ok(())
    }
}
