use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{WindowCanvas, Texture};

mod GameLoop;
mod Grid;
mod Constants;
mod input;

fn main() -> Result<(), String> {

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let display_mode = sdl_context.video().unwrap().current_display_mode(0).unwrap();

    let height = display_mode.h as u32;
    let width = display_mode.w as u32;

    let window = video_subsystem.window("HELLO", width, height)
        .position_centered()
        .build()
        .expect("could not initialize video subsystem");

    let mut canvas = window.into_canvas().build()
        .expect("could not make a canvas");

    canvas.set_draw_color(Constants::get_background_color());
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump()?;

    GameLoop::run_game_loop( event_pump, &mut canvas, height, width );

    Ok(())
}