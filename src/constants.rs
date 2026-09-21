use std::sync::LazyLock;

pub static RESOURCE_FOLDER: LazyLock<String> =
    LazyLock::new(|| std::env::var("RESOURCE_FOLDER").unwrap_or("resources".to_string()));
