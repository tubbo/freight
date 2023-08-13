use std::path::PathBuf;

/// Build the given code path to a binary file on disk.
#[allow(clippy::ptr_arg)]
pub fn build(path: &String, file: &PathBuf) {
    println!("built '{}' to {}", path, file.display());
}

/// Run a code path for the current project.
pub fn run(path: &String) {
    println!("running '{}'", path);
}

/// Publish a package to the Freight registry.
pub fn publish(name: &String, version: &String) {
    println!("published package '{}' at {}", name, version);
}
