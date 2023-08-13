use clap::{Parser, Subcommand};
use std::path::PathBuf;

/// Command-line interface to the Freight toolchain.
#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[arg(short, long)]
    /// Show all messages from compiler output.
    verbose: bool,

    #[arg(short, long)]
    /// Only show error messages from compiler output.
    quiet: bool,
}

#[derive(Debug, Subcommand)]
/// Commands for the Freight toolchain CLI.
enum Commands {
    /// Compile a code path from your project and write its binary to a file on disk.
    Build {
        /// A code path to compile, like 'foo::Bar.baz()'.
        path: String,
        /// A file path on disk to write the binary to.
        file: PathBuf,
    },

    /// Compile a code path from your project and run it immediately.
    Run {
        /// A code path to compile, like 'foo::Bar.baz()'.
        path: String,
    },

    /// Publish a package to the Freight registry.
    Publish {
        /// Name of the package to publish.
        package: String,

        /// Version of the package to publish. This must be unique, or you will
        /// receive an error from the registry.
        version: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Build { path, file } => freight::build(path, file),
        Commands::Run { path } => freight::run(path),
        Commands::Publish { package, version } => freight::publish(package, version),
    }
}
