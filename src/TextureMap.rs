struct TextureMap {
    black_texures:Vec<TextureMap>,
    white_texures:Vec<TextureMap>,
}

impl TextureMap {
    pub fn new() -> Self {
        Self {
            black_texures:Vec::new(),
            white_texures:Vec::new(),
        }
    }
}