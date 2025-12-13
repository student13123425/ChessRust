use raylib::prelude::*;

pub struct TextureMap {
    pub black_textures: Vec<Texture2D>,
    pub white_textures: Vec<Texture2D>,
}

impl TextureMap {
    pub fn new(rl: &mut RaylibHandle, thread: &RaylibThread) -> Self {
        let mut black_textures = Vec::new();
        let mut white_textures = Vec::new();

        black_textures.push(rl.load_texture(thread, "sprites/KingBlack.png").unwrap());
        black_textures.push(rl.load_texture(thread, "sprites/QweenBlack.png").unwrap());
        black_textures.push(rl.load_texture(thread, "sprites/RookBlack.png").unwrap());
        black_textures.push(rl.load_texture(thread, "sprites/BishopBlack.png").unwrap());
        black_textures.push(rl.load_texture(thread, "sprites/KnightBlack.png").unwrap());
        black_textures.push(rl.load_texture(thread, "sprites/PawnBlack.png").unwrap());

        white_textures.push(rl.load_texture(thread, "sprites/KingwWhite.png").unwrap());
        white_textures.push(rl.load_texture(thread, "sprites/QweenWhite.png").unwrap());
        white_textures.push(rl.load_texture(thread, "sprites/RookWhite.png").unwrap());
        white_textures.push(rl.load_texture(thread, "sprites/BishopWhite.png").unwrap());
        white_textures.push(rl.load_texture(thread, "sprites/KnightWhite.png").unwrap());
        white_textures.push(rl.load_texture(thread, "sprites/PawnWhite.png").unwrap());

        Self {
            black_textures,
            white_textures,
        }
    }
}