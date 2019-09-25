
use sdl2::event::Event;
use sdl2::mouse::MouseButton;

use crate::Grid;
use crate::Constants;
use crate::Grid::Tile;
use sdl2::render::WindowCanvas;
use sdl2::event::EventType;
use sdl2::EventPump;

pub fn on_mouse_release( canvas: &mut WindowCanvas, tile_grid: &mut Vec<Tile>, width: u32, height: u32, last_idx: usize, x: i32, y: i32 ) -> usize  {

            //let state = event.mouse_state();
    let current_sel_idx = Grid::find_grid_index(x, y, width, height);

            if tile_grid[current_sel_idx].selected == false {
                canvas.set_draw_color( Constants::get_background_line_color() );
                canvas.draw_rect( tile_grid[last_idx].rect );
                tile_grid[last_idx].selected = false;

                canvas.set_draw_color(Constants::get_highlight_color());
                canvas.draw_rect(tile_grid[current_sel_idx].rect);
                tile_grid[current_sel_idx].selected = true;
                println!("X = {:?}, Y = {:?}", x, y);
                canvas.present();
                return current_sel_idx;

            }  else  {

                return last_idx;

            }

        }