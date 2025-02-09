mod conversion_file;
mod module;
mod files;
mod service;
use service::convert;
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let input_path: String = args[1].clone();
    let output_path: String = args[2].clone();
    convert(&*input_path, &*output_path);
}