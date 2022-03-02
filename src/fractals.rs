extern crate sdl2;

#[derive(Clone, Copy)]
pub struct Complex {
    r: f32,
    i: f32,
}

impl core::fmt::Display for Complex {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "({} + {}i)", self.r, self.i)
    }
}

impl std::ops::Add for Complex {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            r: self.r + other.r,
            i: self.i + other.i,
        }
    }
}

impl Complex {
    pub fn new(r: f32, i: f32) -> Complex {
        Complex { r, i }
    }

    pub fn square(&self) -> Complex {
        Complex {
            r: (self.r.powi(2)) - (self.i.powi(2)),
            i: 2.0 * self.r * self.i,
        }
    }
}

/*
    TODO: Ask Eric why this doesn't work and what the err means
impl Canvas<Window> {
    pub fn draw_big_point(&self, point: P, size: u8) where P: Into<Point> {

    }
}*/
mod custom_draw {
    use sdl2::{rect::Point, rect::Rect, render::Canvas, video::Window};

    fn draw_big_point<P: Into<Point>>(
        canvas: &mut Canvas<Window>,
        point: P,
        size: u32,
    ) -> Result<(), String> {
        let point = point.into();

        canvas.draw_rect(Rect::new(point.x, point.y, size, size))?;
        Ok(())
    }
}

mod math {
    use super::Complex;

    pub fn distance(a: Complex, b: Complex) -> f32 {
        ((a.r + b.r).powi(2) + (a.i + b.i).powi(2)).sqrt()
    }
}

pub mod mandelbrot {
    use sdl2::{pixels::Color, render::Canvas, video::Window};

    use super::{math, Complex};

    pub fn generate_window(
        depth: i32,
        screen_width: i32,
        screen_height: i32,
        canvas: &mut Canvas<Window>,
    ) -> Result<(), String> {
        let screen_width = screen_width * (10 ^ depth);
        let screen_height = screen_height * (10 ^ depth);
        canvas.set_draw_color(Color::RGB(255, 255, 255));

        for x in (screen_width / -2)..(screen_width / 2 - 1) {
            for y in (screen_width / -2)..(screen_height / 2 - 1) {
                if is_in_set(super::Complex {
                    r: (x as f32) / ((10 ^ depth) as f32),
                    i: (y as f32) / ((10 ^ depth) as f32),
                }) {
                    canvas.draw_point(((x + 400) as i32, (y + 300) as i32))?;
                    println!(
                        "x = {} y = {}",
                        (x as f32) / ((10 ^ depth) as f32),
                        (y as f32) / ((10 ^ depth) as f32)
                    );
                }
                //canvas.present();
            }
        }

        Ok(())
    }
    pub fn is_in_set(constant: Complex) -> bool {
        let mut e = Complex { r: 0.0, i: 0.0 } + constant;

        for _i in 0..16 {
            e = mandel(e, constant);
        }
        math::distance(e, Complex { r: 0.0, i: 0.0 }) <= 1.0
    }

    fn mandel(i: Complex, constant: Complex) -> Complex {
        i.square() + constant
    }
}
