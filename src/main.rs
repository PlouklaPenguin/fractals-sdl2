extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::{
    env,
    num::{ParseFloatError, ParseIntError},
    time,
};

mod fractals;
use fractals::{mandelbrot, Complex};

static WINDOW_WIDTH: u32 = 800;
static WINDOW_HEIGHT: u32 = 600;

static SIXTEEN_MILIS: time::Duration = time::Duration::new(0, 33000000);

fn sleep(time: time::Duration) {
    let now = time::Instant::now();
    while now.elapsed() < time {}
}

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

            /* canvas.set_draw_color(Color::RGB(0, 0, 0));
            canvas.clear();
            canvas.set_draw_color(Color::RGB(255, 255, 255));
            mandelbrot::generate_window(
                args[2].parse().map_err(|e: ParseIntError| e.to_string())?,
                800, /*WINDOW_WIDTH as i128*/
                WINDOW_HEIGHT as i128,
                &mut canvas,
            )?; */

            let mut event_pump = sdl_context.event_pump()?;

            'running: loop {
                let now = time::Instant::now();

                for event in event_pump.poll_iter() {
                    match event {
                        Event::MouseButtonDown { x, y, .. } => {
                            canvas.draw_point((x, y))?;
                        }
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
                canvas.set_draw_color(Color::RGB(255, 255, 255));
                let window = canvas.window_mut();

                let size = window.size();

                mandelbrot::generate_window(
                    args[2].parse().map_err(|e: ParseIntError| e.to_string())?,
                    size.0,
                    size.1,
                    &mut canvas,
                )?;
                canvas.present();

                /* let title = format!(
                    "Window - pos({}x{}), size({}x{}): {}",
                    position.0, position.1, size.0, size.1, tick
                );

                window.set_title(&title).map_err(|e| e.to_string())?; */

                if now.elapsed() < SIXTEEN_MILIS {
                    sleep(SIXTEEN_MILIS - now.elapsed());
                }
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
