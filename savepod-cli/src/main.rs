use savepod_lib::{
    manifest::Manifest,
    remote::{Fetch, Remote},
    storage::{LocalDataStore, Store},
};

const URL: &str = "https://raw.githubusercontent.com/mtkennerly/ludusavi-manifest/master/data/manifest.yaml";

fn main() {
    let _manifest: Manifest = Remote::new()
        .fetch(URL)
        .and_then(|manifest| LocalDataStore::new().store(manifest))
        .unwrap_or_else(|err| {
            eprintln!("Fatal error: {}", err);
            std::process::exit(1);
        });
}
