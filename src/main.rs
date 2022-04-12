extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::{env, num::ParseFloatError, thread, time};

mod fractals;
use fractals::{mandelbrot, Complex};

static WINDOW_WIDTH: u32 = 800;
static WINDOW_HEIGHT: u32 = 600;

static SIXTEEN_MILIS: time::Duration = time::Duration::new(0, 16000000);

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

            let mut mouse_loc = ((WINDOW_WIDTH / 2) as i32, (WINDOW_HEIGHT / 2) as i32);

            let zoom_inc: f32 = args[2]
                .parse()
                .map_err(|e: ParseFloatError| e.to_string())?;
            let mut zoom = 2.5_f32;

            canvas.set_draw_color(Color::RGB(0, 0, 0));
            canvas.clear();
            canvas.set_draw_color(Color::RGB(255, 255, 255));

            let mut og_loc = (WINDOW_WIDTH as i32 / 2, WINDOW_HEIGHT as i32 / 2);

            mandelbrot::generate_window(
                WINDOW_WIDTH as i32,
                WINDOW_HEIGHT as i32,
                &mut canvas,
                mouse_loc,
                og_loc,
                zoom,
            )?;

            let mut event_pump = sdl_context.event_pump()?;

            let mut mouse_down = false;

            'running: loop {
                let now = time::Instant::now();

                let window = canvas.window_mut();
                let size = window.size();

                for event in event_pump.poll_iter() {
                    match event {
                        Event::MouseButtonDown { x, y, .. } => {
                            //mouse_loc = (x, y);
                            //println!("x: {}, y: {}", mouse_loc.0, mouse_loc.1);
                            //canvas.set_draw_color(Color::RGB(0, 0, 0));
                            //canvas.clear();

                            /* mandelbrot::generate_window(
                                size.0 as i32,
                                size.1 as i32,
                                &mut canvas,
                                mouse_loc,
                                zoom,
                            )?; */

                            mouse_down = true;
                            og_loc = (x, y);
                        }
                        Event::MouseButtonUp { x, y, .. } => {
                            let current_loc = (x, y);

                            mandelbrot::generate_window(
                                size.0 as i32,
                                size.1 as i32,
                                &mut canvas,
                                mouse_loc,
                                og_loc,
                                zoom,
                            )?;
                        }
                        Event::Quit { .. }
                        | Event::KeyDown {
                            keycode: Some(Keycode::Escape),
                            ..
                        } => break 'running,
                        Event::KeyDown { keycode, .. } => match keycode {
                            Some(Keycode::Equals) => {
                                zoom += zoom_inc;
                                canvas.set_draw_color(Color::RGB(0, 0, 0));
                                canvas.clear();

                                mandelbrot::generate_window(
                                    size.0 as i32,
                                    size.1 as i32,
                                    &mut canvas,
                                    mouse_loc,
                                    og_loc,
                                    zoom,
                                )?;
                            }
                            Some(Keycode::Minus) => {
                                zoom -= zoom_inc;

                                canvas.set_draw_color(Color::RGB(0, 0, 0));
                                canvas.clear();

                                mandelbrot::generate_window(
                                    size.0 as i32,
                                    size.1 as i32,
                                    &mut canvas,
                                    mouse_loc,
                                    og_loc,
                                    zoom,
                                )?;
                            }
                            _ => (),
                        },
                        _ => {}
                    }
                }

                canvas.present();

                if now.elapsed() < SIXTEEN_MILIS {
                    thread::sleep(SIXTEEN_MILIS - now.elapsed());
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
