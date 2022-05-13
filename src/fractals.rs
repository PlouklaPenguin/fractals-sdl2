extern crate sdl2;

#[derive(Clone, Copy)]
pub struct Complex {
    r: f64,
    i: f64,
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
    pub fn new(r: f64, i: f64) -> Self {
        Complex { r, i }
    }

    pub fn square(&self) -> Self {
        Complex::new((self.r.powi(2)) - (self.i.powi(2)), 2.0 * self.r * self.i)
    }

    pub fn squared_distance(&self, b: Self) -> f64 {
        (self.r + b.r).powi(2) + (self.i + b.i).powi(2)
    }
}

/*
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
*/

pub mod mandelbrot {

    use sdl2::{pixels::Color, render::Canvas, video::Window};

    use super::Complex;

    ///Draws all points on the screen within the Mandelbrot set, at a specific centre
    pub fn generate_window(
        screen_width: i32,
        screen_height: i32,
        canvas: &mut Canvas<Window>,
        mouse_loc: (i32, i32),
        zoom: u32,
    ) -> Result<(), String> {
        canvas.set_draw_color(Color::RGB(255, 255, 255));

        let zoomed_divisor = 10_i32.pow(zoom);

        for x in (-screen_width/* / 2 */)..(screen_width/* / 2 */) {
            for y in (-screen_height/* / 2 */)..(screen_height/* / 2 */) {
                let centered_x = (x as i32) + mouse_loc.0;
                let centered_y = (y as i32) + mouse_loc.1;

                if (centered_x < screen_width - 10 && centered_x > 10)
                    && (centered_y < screen_height - 10 && centered_y > 10)
                {
                    draw(x, y, canvas, mouse_loc, zoomed_divisor)?;
                }
            }
        }

        Ok(())
    }
    /// Calculates if a complex number is in a set
    pub fn is_in_set(constant: Complex) -> bool {
        let mut e = Complex::new(0.0, 0.0) + constant;

        for _i in 0..32 {
            e = mandel(e, constant);
            let d = e.squared_distance(Complex::new(0.0, 0.0));
            if d > 4.0 {
                return false;
            }
        }
        true
    }

    /// The Mandelbrot Function
    fn mandel(i: Complex, constant: Complex) -> Complex {
        i.square() + constant
    }

    fn draw(
        x: i32,
        y: i32,
        canvas: &mut Canvas<Window>,
        center: (i32, i32),
        zoomed_divisor: i32,
    ) -> Result<(), String> {
        if is_in_set(Complex::new(
            x as f64 / zoomed_divisor as f64,
            y as f64 / zoomed_divisor as f64,
        )) {
            canvas.draw_point((x + center.0, y + center.1))?;
        }
        Ok(())
    }
}
