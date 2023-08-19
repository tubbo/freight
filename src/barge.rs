use serde::{Deserialize, Serialize};
use std::fs::{copy, create_dir_all, write};
use std::path::PathBuf;
use typed_html::{dom::DOMTree, html};

/// Barge is the Freight package registry, implemented as a statically-generated
/// website that includes both HTML pages as well as `.zip` and `.json` files
/// that the Freight CLI can work with. Barges can be deployed anywhere with a
/// web server, making repository deployment simple on GitHub Pages and other
/// free services.
#[derive(Debug, Deserialize, Serialize)]
pub struct Barge {
    /// Path to the barge repository on disk.
    path: PathBuf,

    /// Packages that are served by the barge repository.
    packages: Vec<Package>,
}

/// A package on the Barge.
#[derive(Debug, Deserialize, Serialize)]
struct Package {
    /// Name of the package
    name: String,

    /// Version of the package
    version: String,

    /// Current location of the package. This is typically in a data dir
    /// somewhere.
    file: PathBuf,
}

impl Barge {
    /// Open a new Barge at the corresponding path, including any existing
    /// data from the directory.
    pub fn open(path: &PathBuf) -> Self {
        let packages: Vec<Package> = vec![];

        Barge {
            path: path.clone(),
            packages,
        }
    }

    /// Add a package with the corresponding version to the registry.
    pub fn load(&mut self, name: String, version: &String, file: &PathBuf) {
        self.packages.push(Package {
            name: name.clone(),
            version: version.clone(),
            file: file.clone(),
        });
    }

    /// Copy all packages from their source paths and generate the index
    /// files for the repository.
    pub fn sail(&self) -> std::io::Result<()> {
        let index = &self.path.clone().join("index.json");
        let homepage = &self.path.clone().join("index.html");

        create_dir_all(&self.path)?;

        for package in &self.packages {
            let filename = format!("{}-v{}.zip", package.name, package.version);
            let published = &self.path.clone().join(filename);

            copy(&package.file, published)?;
        }

        write(index, &self.to_json()?)?;
        write(homepage, &self.to_html())?;
        Ok(())
    }

    /// This barge's index as JSON for use by the toolchain.
    fn to_json(&self) -> serde_json::Result<String> {
        serde_json::to_string(&self)
    }

    /// This barge's index as HTML for easy navigation.
    fn to_html(&self) -> String {
        let doc: DOMTree<String> = html!(
            <html>
                <head>
                    <title>"Barge"</title>
                </head>
                <body>
                    <h1>"Barge"</h1>
                </body>
            </html>
        );

        doc.to_string()
    }
}

/// Open the `Barge` at the given path, creating one if it doesn't already
/// exist. If an `index.json` is discovered within the directory, it will
/// be parsed as a JSON index for the barge, and the packages field will
/// be populated accordingly.
pub fn open(path: &PathBuf) -> Barge {
    Barge::open(path)
}
