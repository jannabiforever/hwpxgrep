use regex::Regex;
use std::sync::LazyLock;

pub(crate) static XML_REG_EXP: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"Contents/section\d+\.xml").unwrap());
