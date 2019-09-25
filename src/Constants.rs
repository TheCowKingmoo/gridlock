use sdl2::pixels::Color;

pub const TILE_SIZE: u32 = 80;
pub const WINDOW_WIDTH: u32 = 1920;
pub const WINDOW_HEIGHT: u32 = 1080;
pub const NUM_OF_TILES_ROW: usize = (WINDOW_HEIGHT / TILE_SIZE) as usize;
pub const NUM_OF_TILES_COL: usize = (WINDOW_WIDTH / TILE_SIZE) as usize;

pub fn get_background_color() -> Color  {
    return Color::RGB(0, 0, 0);
}
pub fn get_highlight_color() -> Color  {
    return Color::RGB(255, 0, 0);
}
