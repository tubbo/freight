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
#[command(author, version, long_about, max_term_width = 80)]
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

    /// Compile and immediately run code in your project.
    Run {
        /// A code path to compile, like 'foo::Bar.baz()'.
        path: String,
    },

    /// Publish this project's packages.
    ///
    /// This command publishes the exportable packages in your project, first
    /// by compiling said packages into distributable tarballs, and then
    /// generating both an index for the package repository as well as a set of
    /// documentation pages for your project's packages. This artifact can then
    /// be uploaded to any static hosting service, such as GitHub Pages, for
    /// publishing both your project's codebase and its documentation.
    Ship {
        /// Version of the package(s) to publish.
        version: String,

        /// Optional short list of packages to publish.
        #[arg(short, long)]
        packages: Option<Vec<String>>,

        /// Directory to output files to.
        #[arg(short, long, default_value = "./docs")]
        output: PathBuf,
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
        Commands::Ship {
            version,
            packages,
            output,
        } => project.ship(version, packages, output),
    }
}
