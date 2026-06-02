use std::{
    fs::{File, create_dir_all},
    io::Write,
    path::PathBuf,
};

use directories::ProjectDirs;

use crate::manifest::{Manifest, ManifestError};

pub trait Store<T, E> {
    fn store(&self, stc: T) -> Result<T, E>;
}

pub struct LocalDataStore {}

impl LocalDataStore {
    pub fn new() -> LocalDataStore {
        LocalDataStore {}
    }
}

impl Default for LocalDataStore {
    fn default() -> Self {
        Self::new()
    }
}

impl Store<Manifest, ManifestError> for LocalDataStore {
    fn store(&self, manifest: Manifest) -> Result<Manifest, ManifestError> {
        let dirs = ProjectDirs::from("", "", "savepod").ok_or(ManifestError::Generic)?;
        create_dir_all(dirs.data_dir()).map_err(|_| ManifestError::Generic)?;

        let mut path = PathBuf::new();
        path.push(dirs.data_dir());
        path.push("manifest.yaml");

        let mut file = match File::create(&path) {
            Err(why) => panic!("couldn't create file: {:?}", why),
            Ok(file) => file,
        };
        match file.write_all(manifest.content.as_bytes()) {
            Err(why) => panic!("couldn't write file: {:?}", why),
            Ok(_) => println!("successfully wrote to file"),
        }
        Ok(manifest)
    }
}
