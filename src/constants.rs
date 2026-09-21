use std::sync::LazyLock;

pub static INPUT_FILE: LazyLock<String> =
    LazyLock::new(|| std::env::var("INPUT_FILE").unwrap_or("resources/transactions.csv".to_string()));
