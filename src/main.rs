extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;

static WINDOW_WIDTH: u32 = 800;
static WINDOW_HEIGHT: u32 = 600;

mod fractals;
use fractals::{mandelbrot, Complex};
use std::{
    env,
    num::{ParseFloatError, ParseIntError},
};

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    match args[1].as_str() {
        "gui" => {
            let sdl_context = sdl2::init()?;
            let video_subsystem = sdl_context.video()?;

            let window = video_subsystem
                .window("Mandelbrot", WINDOW_WIDTH, WINDOW_HEIGHT)
                .resizable()
                .build()
                .map_err(|e| e.to_string())?;

            let mut canvas = window
                .into_canvas()
                .present_vsync()
                .build()
                .map_err(|e| e.to_string())?;

            let mut tick = 0;

            let mut event_pump = sdl_context.event_pump()?;



            'running: loop {
                for event in event_pump.poll_iter() {
                    match event {
                        Event::Quit { .. }
                        | Event::KeyDown {
                            keycode: Some(Keycode::Escape),
                            ..
                        } => break 'running,
                        _ => {}
                    }
                }
                canvas.set_draw_color(Color::RGB(0, 0, 0));
                canvas.clear();
                canvas.present();
                canvas.set_draw_color(Color::RGB(255, 255, 255));
                mandelbrot::generate_window(
                    args[2].parse().map_err(|e: ParseIntError| e.to_string())?,
                    800, /*WINDOW_WIDTH as i128*/
                    WINDOW_HEIGHT as i128,
                    &mut canvas,
                )?;
                canvas.present();

                let window = canvas.window_mut();

                let position = window.position();
                let size = window.size();
                let title = format!(
                    "Window - pos({}x{}), size({}x{}): {}",
                    position.0, position.1, size.0, size.1, tick
                );

                window.set_title(&title).map_err(|e| e.to_string())?;

                tick += 1;
            }
        }
        "inset" => {
            let eq = Complex::new(
                args[2]
                    .parse()
                    .map_err(|e: ParseFloatError| e.to_string())?,
                args[3]
                    .parse()
                    .map_err(|e: ParseFloatError| e.to_string())?,
            );
            println!("Result: {}", mandelbrot::is_in_set(eq));
        }
        "square" => {
            let eq = Complex::new(
                args[2]
                    .parse()
                    .map_err(|e: ParseFloatError| e.to_string())?,
                args[3]
                    .parse()
                    .map_err(|e: ParseFloatError| e.to_string())?,
            );
            println!("Result: {}", eq.square());
        }
        _ => (),
    }
    println!("Hello, world!");

    Ok(())
}
