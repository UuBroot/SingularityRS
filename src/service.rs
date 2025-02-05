mod ffmpeg_module;
mod files;
mod image_module;
mod text_module;

use crate::service::ffmpeg_module::{ffmpeg_convert, ffmpeg_is_supported_format};
use crate::service::files::{get_absolute_path, make_empty_file};
use crate::service::image_module::{image_convert, image_is_supported_format};
use crate::service::text_module::{text_convert, text_is_supported_format};
use std::path::Path;

pub fn convert(input: &str, output: &str) {
    println!("converting {} to {}", input, output);

    let input_format_split: Vec<_> = input.split('/').collect();
    let input_format = input_format_split
        .last()
        .unwrap()
        .split('.')
        .last()
        .unwrap();

    let output_format_split: Vec<_> = output.split('/').collect();
    let output_format = output_format_split
        .last()
        .unwrap()
        .split('.')
        .last()
        .unwrap();

    //gets the module to use
    let module_to_use = match get_module_to_use(input_format, output_format) {
        Some(module) => module,
        None => {
            eprintln!("File not supported");
            return;
        }
    };

    //gets absolute path for input
    let abs_input = match get_absolute_path(input) {
        Some(abs_path) => abs_path,
        None => {
            println!("File does not exist or cannot be resolved.");
            "".parse().unwrap()
        }
    };

    //creates a temporary file to get the absolute path without errors
    make_empty_file(output);
    //gets absolute path for output
    let abs_output = match get_absolute_path(output) {
        Some(abs_path) => abs_path,
        None => {
            println!("File does not exist or cannot be resolved.");
            "".parse().unwrap()
        }
    };

    let mut output_folder_split: Vec<_> = abs_output.split('/').collect();
    output_folder_split.pop(); //removes the filename from the output
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

    match module_to_use.as_str() {
        "ffmpeg" => match ffmpeg_convert(&*abs_input, &*abs_output) {
            Ok(message) => println!("{}", message),
            Err(error) => println!("Error: {}", error),
        },
        "text" => match text_convert(&*abs_input, &*abs_output, &input_format, &output_format) {
            Ok(message) => println!("{}", message),
            Err(error) => println!("Error: {}", error),
        },
        "image" => match image_convert(&*abs_input, &*abs_output) {
            Ok(message) => println!("{}", message),
            Err(error) => println!("Error: {}", error),
        },
        _ => {
            eprintln!("format not supported");
        }
    }
}
fn get_module_to_use(input_format: &str, output_format: &str) -> Option<String> {
    let input_module = match get_module_from_format(input_format) {
        Some(module) => module,
        None => return None,
    };
    let output_module = match get_module_from_format(output_format) {
        Some(module) => module,
        None => return None,
    };
    if input_module == output_module {
        Some(input_module.to_string())
    } else {
        let all_input_modules = get_all_modules_from_format(input_format);
        let all_output_modules = get_all_modules_from_format(output_format);

        if all_input_modules
            .iter()
            .filter(|&n| *n == output_module)
            .count()
            > 0
        {
            Some(output_module.to_string())
        } else if all_output_modules
            .iter()
            .filter(|&n| *n == input_module)
            .count()
            > 0
        {
            Some(input_module.to_string())
        } else {
            None
        }
    }
}
fn get_module_from_format(format: &str) -> Option<String> {
    if image_is_supported_format(format) {
        Some("image".to_string())
    } else if ffmpeg_is_supported_format(format) {
        Some("ffmpeg".to_string())
    } else if text_is_supported_format(format) {
        Some("text".to_string())
    } else {
        None
    }
}
fn get_all_modules_from_format(format: &str) -> Vec<String> {
    let mut all_modules: Vec<String> = Vec::new();
    if ffmpeg_is_supported_format(format) {
        all_modules.push("ffmpeg".to_string());
    }
    if text_is_supported_format(format) {
        all_modules.push("text".to_string());
    }
    all_modules
}
