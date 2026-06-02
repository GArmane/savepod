use std::{io::Read, str::FromStr};

use url::Url;

use crate::manifest::{Manifest, ManifestError};

pub trait Fetch<T, E> {
    fn fetch(&self, url: &str) -> Result<T, E>;
}

pub struct Remote {}

impl Remote {
    pub fn new() -> Remote {
        Remote {}
    }
}

impl Default for Remote {
    fn default() -> Self {
        Self::new()
    }
}

impl Fetch<Manifest, ManifestError> for Remote {
    fn fetch(&self, url: &str) -> Result<Manifest, ManifestError> {
        let mut res = reqwest::blocking::get(url)?;

        let mut body = String::new();
        res.read_to_string(&mut body)?;

        let etag = res
            .headers()
            .get(reqwest::header::ETAG)
            .ok_or(ManifestError::MissingHeaderETag)
            .and_then(|header| header.to_str().map_err(ManifestError::InvalidHeaderETag))
            .map(|value| value.to_string())?;

        let url = Url::from_str(url)?;
        Manifest::new(url, etag, body)
    }
}

#[cfg(test)]
mod tests {
    mod fetch_manifest {
        use mockito;
        use rstest::*;

        use crate::{
            manifest::ManifestError,
            remote::{Fetch, Remote},
        };

        #[fixture]
        fn resource_url() -> &'static str {
            "/manifest.yaml"
        }

        #[fixture]
        fn etag() -> &'static str {
            "1234"
        }

        #[fixture]
        fn body() -> &'static str {
            "manifest content"
        }

        #[rstest]
        fn test_success(resource_url: &str, etag: &str, body: &str) {
            let mut server = mockito::Server::new();
            let mock = server
                .mock("GET", resource_url)
                .with_status(200)
                .with_header("ETag", etag)
                .with_body(body)
                .create();

            let remote = Remote::new();
            let result = remote.fetch(&format!("{}{}", server.url(), resource_url));

            mock.assert();

            let manifest = result.expect("Fetch failed unexpectedly");
            assert_eq!(manifest.etag, etag);
            assert_eq!(manifest.content, body);
        }

        #[rstest]
        fn test_missing_header(resource_url: &str, body: &str) {
            let mut server = mockito::Server::new();
            let mock = server
                .mock("GET", resource_url)
                .with_status(200)
                .with_body(body)
                .create();

            let remote = Remote::new();
            let result = remote.fetch(&format!("{}{}", server.url(), resource_url));

            mock.assert();
            assert!(result.is_err());
            assert!(matches!(result, Err(ManifestError::MissingHeaderETag)));
        }
    }
}
