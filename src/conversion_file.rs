mod ffmpeg_module;
mod image_module;
mod text_module;
use crate::module::Module;
use crate::conversion_file::ffmpeg_module::ffmpeg_convert;
use crate::conversion_file::image_module::image_convert;
use crate::conversion_file::text_module::text_convert;
use crate::files::get_file_format;

pub struct ConversionFile {
    input_file_path: String,
    output_file_path: String,
    module: Module,
}
impl ConversionFile {
    pub fn new(input_file_path: String, output_file_path: String) -> Self {
        let module = get_module_to_use(get_file_format(&*input_file_path), get_file_format(&*output_file_path)).expect("error getting module to use");
        ConversionFile {
            input_file_path,
            output_file_path,
            module
        }
    }
    pub fn convert(&self) -> Result<(), String> {
        match self.module {
            Module::FFMPEG=>{
                match ffmpeg_convert(self.input_file_path.as_str(), self.output_file_path.as_str()) {
                    Ok(_) => Ok(()),
                    Err(error) => Err(error)
                }
            },
            Module::TEXT=>{
                match text_convert(self.input_file_path.as_str(), self.output_file_path.as_str()) {
                    Ok(..) => Ok(()),
                    Err(error) => Err(error)
                }
            },
            Module::IMAGE=>{
                match image_convert(self.input_file_path.as_str(), self.output_file_path.as_str()) {
                    Ok(..) => Ok(()),
                    Err(error) => Err(error)
                }
            }
        }
    }
}
fn get_module_to_use(input_format: &str, output_format: &str) -> Option<Module> {
    let input_module = match get_module_from_format(input_format) {
        Some(module) => module,
        None => return None,
    };
    let output_module = match get_module_from_format(output_format) {
        Some(module) => module,
        None => return None,
    };
    if input_module == output_module {
        Some(input_module)
    } else {
        let all_input_modules = get_all_modules_from_format(input_format);
        let all_output_modules = get_all_modules_from_format(output_format);

        if all_input_modules
            .iter()
            .filter(|&n| *n == output_module)
            .count()
            > 0
        {
            Some(output_module)
        } else if all_output_modules
            .iter()
            .filter(|&n| *n == input_module)
            .count()
            > 0
        {
            Some(input_module)
        } else {
            None
        }
    }
}
fn get_module_from_format(format: &str) -> Option<Module> {
    if crate::conversion_file::image_module::image_is_supported_format(format) {
        Some(Module::IMAGE)
    } else if crate::conversion_file::ffmpeg_module::ffmpeg_is_supported_format(format) {
        Some(Module::FFMPEG)
    } else if crate::conversion_file::text_module::text_is_supported_format(format) {
        Some(Module::TEXT)
    } else {
        None
    }
}
fn get_all_modules_from_format(format: &str) -> Vec<Module> {
    let mut all_modules: Vec<Module> = Vec::new();
    if crate::conversion_file::ffmpeg_module::ffmpeg_is_supported_format(format) {
        all_modules.push(Module::FFMPEG);
    }
    if crate::conversion_file::image_module::image_is_supported_format(format) {
        all_modules.push(Module::IMAGE);
    }
    if crate::conversion_file::text_module::text_is_supported_format(format) {
        all_modules.push(Module::TEXT);
    }
    all_modules
}