const SUPPORTED_FORMATS: [&str; 16] = ["png", "jpeg", "jpg", "gif", "tiff", "tif", "bmp", "webp", "avif", "hdr", "ico", "tga", "dds", "exr", "qoi", "pnm"];
pub fn image_convert(input: &str , output: &str ) -> Result<String, String> {
    let img = match image::open(input) {
        Ok(img) => img,
        Err(e) => return Err(e.to_string())
    };
    match img.save(output) {
        Ok(_) => Ok(output.to_string()),
        Err(e) => Err(e.to_string())
    }.expect("Error saving image file");
    Ok(String::from(output))
}
pub fn image_is_supported_format(name:&str) -> bool {
    SUPPORTED_FORMATS.contains(&name)
}
