use clap::{error::Result, Error};
use glob::glob;
use std::{fs::read_to_string, path::PathBuf};

/// The local codebase that is being compiled.
///
/// A `Project` contains references to every `.freight` file in the codebase,
/// and can perform tasks on that codebase such as building, running, and
/// managing packages.
pub struct Project {
    /// All file paths in the local codebase.
    files: Vec<PathBuf>,
}

impl Project {
    /// Collect all source code together from the paths in `Project.files` and
    /// output them as a single String. Used by the compiler to gather the code
    /// written locally for the first pass at parsing.
    pub fn source(&self) -> String {
        let mut code = String::new();

        for path in &self.files {
            match read_to_string(path) {
                Ok(source) => code.push_str(&source),
                Err(error) => eprintln!("{:?}", error),
            }
        }

        return code;
    }

    /// Build the given code path to a binary file on disk.
    #[allow(clippy::ptr_arg)]
    pub fn build(&self, path: &String, file: &PathBuf) -> Result<(), Error> {
        println!("built '{}' to {}", path, file.display());
        Ok(())
    }

    /// Run a code path for the current project.
    pub fn run(&self, path: &String) -> Result<(), Error> {
        println!("running '{}'", path);
        Ok(())
    }

    /// Publish a package to the Freight registry.
    pub fn ship(&self, name: &String, version: &String) -> Result<(), Error> {
        println!("published package '{}' at {}", name, version);
        Ok(())
    }
}

/// Initialize a new `Project` from the given root directory. This is typically
/// the current working directory as provided by the OS environment, but can be
/// configured in the CLI using the `--root` option. A project consists of all
/// `.freight` files in its root (recursively), so this function will glob over
/// those paths to return the full set of file paths for this project.
pub fn init(root: &PathBuf) -> Project {
    let path = root.join("**").join("*.freight");
    let pattern = path.to_str().unwrap_or_default();
    let files = glob(pattern)
        .unwrap()
        .filter_map(|entry| entry.ok())
        .collect();

    Project { files }
}

#[test]
fn test_init() {
    let cwd = std::env::current_dir().unwrap();
    let project = init(&cwd);

    assert_ne!(project.files.len(), 0);
}

#[test]
fn test_build() {
    let project = init(&PathBuf::new());
    let path = "foo::bar()".to_string();
    let file = PathBuf::new();

    assert!(project.build(&path, &file).is_ok())
}

#[test]
fn test_run() {
    let project = init(&PathBuf::new());
    let path = "foo::bar()".to_string();

    assert!(project.run(&path).is_ok())
}
