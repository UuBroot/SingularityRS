mod files;
mod ffmpeg_module;

use std::path::Path;
use crate::service::files::{get_absolute_path, make_empty_file};
use crate::service::ffmpeg_module::{ffmpeg_convert, ffmpeg_is_supported_format};
pub fn convert(input: &str, output: &str) {
    println!("converting {} to {}", input, output);

    let input_format_split: Vec<_> = input
        .split('/')
        .collect();
    let input_format = input_format_split
        .last()
        .unwrap()
        .split('.')
        .last()
        .unwrap();

    let output_format_split: Vec<_> = output
        .split('/')
        .collect();
    let output_format = output_format_split
        .last()
        .unwrap()
        .split('.')
        .last()
        .unwrap();

    println!("input format: {}, output format: {}", input_format, output_format);

    //gets absolute path for input
    let abs_input = match get_absolute_path(input){
        Some(abs_path)=>{
            abs_path
        },
        None => {
            println!("File does not exist or cannot be resolved.");
            "".parse().unwrap()
        }
    };

    //creates a temporary file to get the absolute path without errors
    make_empty_file(output);
    //gets absolute path for output
    let abs_output = match get_absolute_path(output){
        Some(abs_path)=>{
            abs_path
        },
        None => {
            println!("File does not exist or cannot be resolved.");
            "".parse().unwrap()
        }
    };

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

    if ffmpeg_is_supported_format(input_format) && ffmpeg_is_supported_format(output_format){
        match ffmpeg_convert(&*abs_input, &*abs_output) {
            Ok(message) => println!("{}", message),
            Err(error) => println!("Error: {}", error),
        }
    }else{
        println!("format is not supported yet");
    }

}