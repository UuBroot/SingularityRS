mod conversion_file;
mod module;
mod files;
mod service;
use service::convert;
use crate::gui_service::start_gui;

mod gui_service;
fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 1 {
        println!("opening gui");
        start_gui();
    }else if args.len() == 3 {
        convert(&*args[1].clone(), &*args[2].clone());
    }else {
        println!("Usage: {} input_file.mp4 output_file.mp3", args[0]);
    }
}