use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use std::time::Duration;
use sdl2::render::{WindowCanvas, Texture};
use sdl2::EventPump;

use crate::Constants;
use crate::Grid;
use crate::input;
use sdl2::event::EventType::MouseButtonUp;

pub fn run_game_loop(mut event_pump: EventPump, canvas: &mut WindowCanvas, height: u32, width: u32 )  {

    let mut idx = 0;

    let mut current_sel_idx = 0;
    let mut tile_grid = Grid::draw_grid( canvas, height, width );
    let mut mouse_up = true;

    'running: loop {

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        //canvas.clear();

        let state = event_pump.mouse_state();

        for event in event_pump.poll_iter() {
            match event {
                Event::MouseButtonUp { mouse_btn:MouseButton::Left, .. } => { idx = input::on_mouse_release( canvas, tile_grid.as_mut(), width, height, idx, state.x(), state.y() )} |
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                },
                _ => {}
            }
        }

        // The rest of the game loop goes here...

        //canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
    }


}