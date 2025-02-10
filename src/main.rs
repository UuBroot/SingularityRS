mod conversion_file;
mod module;
mod files;
mod service;
use service::convert;
fn main() {
    let args: Vec<String> = std::env::args().collect();
    convert(&*args[1].clone(), &*args[2].clone());
}