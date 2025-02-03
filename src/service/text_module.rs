use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};
use serde_json;
use serde_json::Value;
use serde_yaml;

static SUPPORTED_FORMATS: [&str; 5] = [ "json","yaml","yml","xml","csv"];

pub fn text_convert(input: &str , output: &str , input_format: &str, output_format: &str) -> Result<String, String> {
    let data = match read_file(input, input_format) {
        Ok(f) => f,
        Err(e) => return Err( format!("Error while reading file: {}", e) )
    };
    match write_file(output, data, output_format) {
        Ok(f) => f,
        Err(e) => return Err( format!("Error while writing file: {}", e) )
    }
    Ok("converted successfully using text".to_string())
}
pub fn text_is_supported_format(name:&str) -> bool {
    SUPPORTED_FORMATS.contains(&name)
}
fn read_file(path: &str, format: &str) -> Result<HashMap<String, Value>, String> {
    let mut file = match File::open(path) {
        Ok(f) => f,
        Err(e) => return Err(format!("Error opening file: {}", e)),
    };

    let mut content = String::new();
    match file.read_to_string(&mut content) {
        Ok(_) => (),
        Err(e) => return Err(format!("Error reading file: {}", e)),
    }

    match format.to_lowercase().as_str() {
        "json" => {
            match serde_json::from_str(&content) {
                Ok(map) => Ok(map),
                Err(e) => Err(format!("Error parsing JSON: {}", e)),
            }
        },
        "yaml" | "yml" => {
            match serde_yaml::from_str(&content) {
                Ok(map) => Ok(map),
                Err(e) => Err(format!("Error parsing YAML: {}", e)),
            }
        },
        _ => Err(format!("Unsupported file type: {}. Only 'json', 'yaml', or 'yml' are supported.", format)),
    }
}
fn write_file(path: &str, data: HashMap<String, Value>, format: &str) -> Result<(), String> {
    let mut file = match File::create(path) {
        Ok(f) => f,
        Err(e) => return Err(format!("Error creating file: {}", e)),
    };
    match format {
        "json" => {
            // Convert HashMap to JSON string
            let json_string: String = match serde_json::to_string_pretty(&data) {
                Ok(json) => json,
                Err(e) => return Err(format!("Error converting to JSON: {}", e)),
            };
            match file.write_all(json_string.as_bytes()) {
                Ok(_) => Ok(()),
                Err(e) => Err(format!("Error writing to file: {}", e)),
            }
        }
        "yaml" | "yml" => {
            let yaml_string: String = match serde_yaml::to_string(&data) {
                Ok(yaml) => yaml,
                Err(e) => return Err(format!("Error converting to YAML: {}", e)),
            };
            match file.write_all(yaml_string.as_bytes()) {
                Ok(_) => Ok(()),
                Err(e) => Err(format!("Error writing to file: {}", e)),
            }
        }
        _ => {
            eprintln!("Error writing file!");
            Err(format!("Unsupported file type: {}", format))
        }
    }
}