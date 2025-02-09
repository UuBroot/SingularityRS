pub enum Module {
    FFMPEG,
    IMAGE,
    TEXT
}
impl PartialEq for Module {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Module::FFMPEG, Module::FFMPEG) => true,
            (Module::IMAGE, Module::IMAGE) => true,
            (Module::TEXT, Module::TEXT) => true,
            _ => false,
        }
    }
}