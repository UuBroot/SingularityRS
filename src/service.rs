mod files;
mod ffmpeg_module;

use std::path::Path;
use crate::service::files::get_absolute_path;

pub fn convert(input: &str, output: &str) {
    println!("converting {} to {}", input, output);

    //gets absolute path for input
    let abs_input = match get_absolute_path(input){
        Some(abs_path)=>{
            println!("Absolute path: {}", abs_path);
            abs_path
        },
        None => {
            println!("File does not exist or cannot be resolved.");
            "".parse().unwrap()
        }
    };
    println!("{}", abs_input);

    //gets absolute path for output
    let abs_output = match get_absolute_path(output){
        Some(abs_path)=>{
            println!("Absolute path: {}", abs_path);
            abs_path
        },
        None => {
            println!("File does not exist or cannot be resolved.");
            "".parse().unwrap()
        }
    };
    println!("{}", abs_output);

    let mut output_folder_split: Vec<_> = abs_output.split('/').collect();
    output_folder_split.pop();//removes the filename from the output
    let output_folder = output_folder_split.join("/");

    //Checks if input file exists
    if !Path::new(input).exists() {
        println!("{} doesn't exist", input);
        return;
    }

    //Checks if the output folder exists
    if !Path::new(&output_folder).exists() {
        println!("{} folder doesn't exist", output);
        return;
    }


}