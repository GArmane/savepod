use std::sync::LazyLock;

pub static VERSION: LazyLock<&'static str> =
    LazyLock::new(|| option_env!("SAVEPOD_VERSION").unwrap_or(env!("CARGO_PKG_VERSION")));
