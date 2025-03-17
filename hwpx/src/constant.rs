use regex::Regex;
use std::cell::LazyCell;

pub(crate) const XML_REG_EXP: LazyCell<Regex> =
    LazyCell::new(|| Regex::new(r"Contents/section\d+\.xml").unwrap());
