use std::{
    fs::{File, create_dir_all},
    io::Write,
    path::{Path, PathBuf},
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
    dir: PathBuf,
}

impl LocalDataStore {
    pub fn new() -> Result<Self, StorageError> {
        let dirs = ProjectDirs::from("", "", "savepod").ok_or(StorageError::FetchConfig)?;
        Self::with_dir(dirs.data_dir())
    }

    pub fn with_dir<P: AsRef<Path>>(path: P) -> Result<Self, StorageError> {
        let dir = path.as_ref().to_path_buf();
        create_dir_all(&dir).map_err(|_| StorageError::CreateDirs)?;
        Ok(Self { dir })
    }
}

impl Store<Manifest> for LocalDataStore {
    fn store(&self, manifest: Manifest) -> Manifest {
        let mut path = self.dir.clone();
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs, str::FromStr};
    use tempfile::tempdir;
    use url::Url;

    #[test]
    fn test_local_data_store_creates_directory() {
        let temp = tempdir().expect("Failed to create temp dir");
        let target_dir = temp.path().join("my_injected_path");

        assert!(!target_dir.exists());

        let store = LocalDataStore::with_dir(&target_dir).expect("Failed to create store");

        assert!(store.dir.exists());
        assert_eq!(store.dir, target_dir);
    }

    #[test]
    fn test_local_data_store_writes_manifest() {
        let temp = tempdir().unwrap();
        let store = LocalDataStore::with_dir(temp.path()).unwrap();

        let manifest = Manifest {
            content: "some content".to_string(),
            source: Url::from_str("http://example.com").expect("invalid URL"),
            etag: "some tag".to_string(),
        };

        let returned_manifest = store.store(manifest.clone());
        let expected_path = temp.path().join("manifest.yaml");

        assert!(expected_path.exists(), "manifest.yaml was not created");

        let written_content = fs::read_to_string(&expected_path).expect("Failed to read the created manifest file");
        assert_eq!(
            written_content, manifest.content,
            "File contents do not match manifest content"
        );

        assert_eq!(
            returned_manifest.content, manifest.content,
            "Returned manifest does not match input manifest"
        );
    }

    #[test]
    #[should_panic(expected = "couldn't create file: Os { code: 21, kind: IsADirectory, message: \"Is a directory\" }")]
    fn test_store_panics_on_unwritable_file() {
        let temp = tempdir().unwrap();
        let store = LocalDataStore::with_dir(temp.path()).unwrap();

        let target_path = temp.path().join("manifest.yaml");
        fs::create_dir_all(&target_path).unwrap();

        let manifest = Manifest {
            content: "some content".to_string(),
            source: Url::from_str("http://example.com").expect("invalid URL"),
            etag: "some tag".to_string(),
        };

        store.store(manifest);
    }
}
