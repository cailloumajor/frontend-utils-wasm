use clap::{Parser, Subcommand};

mod build_deno;

/// Utility tasks.
#[derive(Parser)]
pub(crate) struct Cli {
    #[command(subcommand)]
    command: Command,
}

impl Cli {
    /// Run the requested command.
    fn run(&self) -> anyhow::Result<()> {
        self.command.run()
    }
}

#[derive(Subcommand)]
enum Command {
    /// Build the Deno library.
    BuildDeno(build_deno::BuildDeno),
}

impl Command {
    fn run(&self) -> anyhow::Result<()> {
        match self {
            Self::BuildDeno(cmd) => cmd.run(),
        }
    }
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    cli.run()
}
