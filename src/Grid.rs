use sdl2::rect::{Rect};
use sdl2::pixels::Color;
use sdl2::render::{WindowCanvas, Texture};

use crate::Constants;

#[derive(Copy, Clone)]
pub struct Tile  {
    pub rect: Rect,
    pub color: Color,
    pub selected: bool
}
impl Tile {
    pub fn new( x: i32, y: i32, passed:Color ) -> Tile  {
        Tile  {
            rect: Rect::new( x, y,Constants::TILE_SIZE,Constants::TILE_SIZE),
            color: passed,
            selected: false
        }
    }
}

//yanks out the last digit and replaces with a zero
fn flatten( num: i32 ) -> i32 {
    if num < 80   {
        return 0;
    }
    let mut n = num;
    n = n - (n % Constants::TILE_SIZE as i32);
    return n;
}

//finds the index for the tile based on coords
pub fn find_grid_index( x: i32, y:i32, canvas_width: u32, canvas_height: u32 ) -> usize  {

    let xcoord = flatten( x );
    let ycoord = flatten( y );

    let row_index = xcoord / Constants::TILE_SIZE as i32;
    let col_index = ycoord / Constants::TILE_SIZE as i32;

    println!( "row idx = {} col idx = {}", row_index, col_index );

    let r: usize = ((row_index * Constants::NUM_OF_TILES_ROW as i32) + col_index) as usize;
    r
}

pub fn draw_grid(canvas: &mut WindowCanvas, h: u32, w: u32 ) -> Vec<Tile>  {

    let mut tiles = Vec::with_capacity( Constants::NUM_OF_TILES_ROW * Constants::NUM_OF_TILES_COL);
    canvas.clear();
    for i in 0..Constants::NUM_OF_TILES_COL  {
        for j in 0..Constants::NUM_OF_TILES_ROW  {

            let mut color = Color::RGB( 255, 255, 255 );

            //if i == 10 && j == 11 {
            //    color = Color::RGB( 0, 0, 255 );
            //    println!( " correct idx = {}", j + (i*NUM_OF_TILES_ROW) );
            //    println!("X = {} Y = {}", i * TILE_SIZE as usize, j * TILE_SIZE as usize);
            //}

            //println!("{}",j + (i*NUM_OF_TILES_ROW));
            tiles.push(Tile::new(i as i32 * Constants::TILE_SIZE as i32 , j as i32 * Constants::TILE_SIZE as i32, color ));
            canvas.set_draw_color( color );
            canvas.draw_rect( tiles[j + (i*Constants::NUM_OF_TILES_ROW) ].rect );
            canvas.present();
        }
    }
    tiles
}
