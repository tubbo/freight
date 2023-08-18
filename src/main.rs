use clap::{error::Result, Error, Parser, Subcommand};
use freight::project;
use std::{env::current_dir, path::PathBuf};

/// Freight - The package-oriented programming language.
///
/// This is the command-line interface to the Freight toolchain, enabling usage
/// of the compiler, build tooling, and packaging infrastructure.
///
/// For more information, visit https://freight-lang.org/
#[derive(Debug, Parser)]
#[command(author, version, long_about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Root directory of the Freight project. (default: current directory)
    #[arg(short, long)]
    root: Option<PathBuf>,
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
    Ship {
        /// Name of the package to publish.
        package: String,

        /// Version of the package to publish. This must be unique, or you will
        /// receive an error from the registry.
        version: String,
    },
}

fn main() -> Result<(), Error> {
    let cli = Cli::parse();
    let project = match &cli.root {
        Some(root) => project::init(&root),
        None => project::init(&current_dir()?),
    };

    match &cli.command {
        Commands::Build { path, file } => project.build(path, file),
        Commands::Run { path } => project.run(path),
        Commands::Ship { package, version } => project.ship(package, version),
    }
}
