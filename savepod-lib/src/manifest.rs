use std::io::Error as IOError;

use reqwest::header::ToStrError;
use thiserror::Error;
use url::{ParseError, Url};

#[allow(unused)]
#[derive(Debug, Error)]
pub enum ManifestError {
    #[error("Error while fetching manifest data from remote.")]
    FetchRemote(#[from] reqwest::Error),
    #[error("Error while parsing manifest response raw data.")]
    ParseResponseData(#[from] IOError),
    #[error("Error missing manifest etag header.")]
    MissingHeaderETag,
    #[error("Error parsing manifest response etag header.")]
    InvalidHeaderETag(#[from] ToStrError),
    #[error("Error parsing URL.")]
    UrlParse(#[from] ParseError),
    #[error("Generic error")]
    Generic,
}

#[derive(Debug)]
pub struct Manifest {
    pub source: Url,
    pub etag: String,
    pub content: String,
}

impl Manifest {
    pub fn new(source: Url, etag: String, content: String) -> Result<Manifest, ManifestError> {
        Ok(Self { source, etag, content })
    }
}
