use std::{
    fs::{File, create_dir_all},
    io::Write,
    path::PathBuf,
};

use directories::ProjectDirs;
use thiserror::Error;

use crate::manifest::Manifest;

#[allow(unused)]
#[derive(Debug, Error)]
pub enum StorageError {
    #[error("Error while fetching project config directory.")]
    FetchConfig,
    #[error("Error while creating project directories.")]
    CreateDirs,
}

pub trait Store<T> {
    fn store(&self, obj: T) -> T;
}

pub struct LocalDataStore {
    dirs: ProjectDirs,
}

impl LocalDataStore {
    pub fn new() -> Result<LocalDataStore, StorageError> {
        let dirs = ProjectDirs::from("", "", "savepod").ok_or(StorageError::FetchConfig)?;
        create_dir_all(dirs.data_dir()).map_err(|_| StorageError::CreateDirs)?;
        Ok(LocalDataStore { dirs })
    }
}

impl Store<Manifest> for LocalDataStore {
    fn store(&self, manifest: Manifest) -> Manifest {
        let mut path = PathBuf::new();
        path.push(self.dirs.data_dir());
        path.push("manifest.yaml");

        let mut file = match File::create(&path) {
            Err(why) => panic!("couldn't create file: {:?}", why),
            Ok(file) => file,
        };
        match file.write_all(manifest.content.as_bytes()) {
            Err(why) => panic!("couldn't write file: {:?}", why),
            Ok(_) => println!("successfully wrote to file"),
        }
        manifest
    }
}
