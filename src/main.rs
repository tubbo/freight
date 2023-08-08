use std::path::PathBuf;
use clap::{Parser,Subcommand};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Compile a code path from your project and write its binary to a file on disk.
    Build {
        /// A code path to compile, like 'foo::Bar.baz()'
        path: String,
        /// A file path on disk to write the binary to
        file: PathBuf,
    },

    /// Compile a code path from your project and run it immediately.
    Run {
        /// A code path to compile, like 'foo::Bar.baz()'
        path: String,
    },

    /// Start a new REPL session.
    Console {},
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Build { path, file } => {
            // let program = compiler::compile(path);
            // let mut output = std::fs::File::create(file);

            // output.write_all(program);
            println!("built {} at {}", path, file.display());
        }
        Commands::Run { path } => {
            println!("running {}...", path)

            // let program = compiler::compile(path);

            // vm::run(program);
        }
        Commands::Console => {
            // TODO: repl
        }
    }
}
