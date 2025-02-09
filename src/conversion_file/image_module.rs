use image::ImageFormat;
pub fn image_convert(input: &str, output: &str) -> Result<String, String> {
    let img = match image::open(input) {
        Ok(img) => img,
        Err(e) => return Err(e.to_string()),
    };
    match img.save(output) {
        Ok(_) => Ok(output.to_string()),
        Err(e) => Err(e.to_string()),
    }
    .expect("Error saving image file");
    Ok(String::from(output))
}
pub fn image_is_supported_format(name: &str) -> bool {
    for f in ImageFormat::all() {
        if f.extensions_str()[0] == name {
            return true;
        }
    }
    false
}