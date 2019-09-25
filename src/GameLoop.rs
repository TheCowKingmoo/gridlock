use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use std::time::Duration;
use sdl2::render::{WindowCanvas, Texture};
use sdl2::EventPump;

use crate::Constants;
use crate::Grid;

pub fn run_game_loop(mut event_pump: EventPump, canvas: &mut WindowCanvas, height: u32, width: u32 )  {

    let mut current_sel_idx = 0;
    let mut tile_grid = Grid::draw_grid( canvas, height, width );

    'running: loop {

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        //canvas.clear();

        let mut state;
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                },
                _ => {}
            }
        }

        if event_pump.mouse_state().is_mouse_button_pressed(MouseButton::Left)  {
            state = event_pump.mouse_state();
            let idx = Grid::find_grid_index( state.x(), state.y(), width, height );

            if current_sel_idx != idx  {
                tile_grid[idx].selected = true;
                canvas.set_draw_color(Constants::get_highlight_color());
                canvas.draw_rect( tile_grid[idx].rect );
                current_sel_idx = idx;
                canvas.present();
            }
            println!("X = {:?}, Y = {:?}", state.x(), state.y());
        }
        // The rest of the game loop goes here...

        //canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
    }


}