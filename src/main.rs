extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;

static WINDOW_WIDTH: u32 = 800;
static WINDOW_HEIGHT: u32 = 600;

mod fractals;
use fractals::{mandelbrot, Complex};
use std::{env, num::ParseFloatError};

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

            let mut event_pump = sdl_context.event_pump()?;

            canvas.set_draw_color(Color::RGB(0, 0, 0));
            canvas.clear();
            canvas.set_draw_color(Color::RGB(255, 255, 255));
            mandelbrot::generate_window(4, WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32, &mut canvas)?;

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

                canvas.present();
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


