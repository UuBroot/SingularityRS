use std::fs::{canonicalize, File};
use std::path::Path;

pub fn get_absolute_path(path: &str) -> Option<String> {
    let path = Path::new(path);
    match canonicalize(path) {
        Ok(abs_path) => Some(abs_path.to_string_lossy().into_owned()),
        Err(_) => None,
    }
}
pub fn make_empty_file(path: &str) {
    File::create(path).unwrap();
}
