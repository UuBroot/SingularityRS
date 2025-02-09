use crate::files::{get_absolute_path, make_empty_file};
use crate::conversion_file::ConversionFile;
use std::fs::remove_file;
pub fn convert(input: &str, output: &str) {
    //gets absolute path for input
    let abs_input = match get_absolute_path(input) {
        Some(abs_path) => abs_path,
        None => {
            eprintln!("{} doesn't exist", input);
            return;
        }
    };
    //creates a temporary file to get the absolute path without errors
    make_empty_file(output);
    //gets absolute path for output
    let abs_output = match get_absolute_path(output) {
        Some(abs_path) => abs_path,
        None => {
            eprintln!("{} folder doesn't exist", output);
            remove_file(output).expect("Error deleting leftover file");//deletes the empty file that was created previously
            return;
        }
    };
    println!("converting {} to {}", input, output);
    let conversion_file = ConversionFile::new(
        (&*abs_input).parse().unwrap(),
        (&*abs_output).parse().unwrap(),
    );
    conversion_file.convert().expect("Error converting file");
}