use std::sync::LazyLock;

pub static INPUT_FILE: LazyLock<String> =
    LazyLock::new(|| std::env::var("INPUT_FILE").unwrap_or("config.json".to_string()));
