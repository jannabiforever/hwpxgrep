use std::{path::PathBuf, sync::LazyLock};

pub static HWPXG_CACHE_FILE: LazyLock<PathBuf> = LazyLock::new(|| {
    std::env::var("HWPXG_CACHE_FILE")
        .map(PathBuf::from)
        .unwrap()
});
