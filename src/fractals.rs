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
    pub fn generate_window(
        screen_width: i32,
        screen_height: i32,
        canvas: &mut Canvas<Window>,
        center: (i32, i32),
        zoom: u32,
    ) -> Result<(), String> {
        //let screen_width = screen_width * 10_i128.pow(depth);
        //println!("{}", screen_width);
        //let screen_height = screen_height * (10_i128.pow(depth));
        canvas.set_draw_color(Color::RGB(255, 255, 255));

        // * Don't think these are necessary
        // take distance on y axis (2) and multiply it by 2^x to expand it to an int
        /*         let yrange: i32 = 3 * (10_i32.pow(depth));
        // -2 to 2
        let xrange: i32 = 3 * (10_i32.pow(depth));
        //println!("xrange: {} yrange: {}", xrange, yrange); */

        for x in -screen_width..screen_width {
            for y in -screen_height..screen_height {
                if is_in_set(Complex::new(
                    (x as f64) / ((10_i128.pow(zoom)) as f64),
                    (y as f64) / ((10_i128.pow(zoom)) as f64),
                )) {
                    canvas.draw_point((
                        ((x as i32) + center.0),
                        ((y as i32) + center.1),
                    ))?;
                    /* if x == -2 && y == 0 {
                        println!("oh");
                    } */
                    //println!("x = {} y = {}", (x as f64), (y as f64));
                }
            }
        }

        Ok(())
    }
    pub fn is_in_set(constant: Complex) -> bool {
        let mut e = Complex::new(0.0, 0.0) + constant;

        for _i in 0..256 {
            e = mandel(e, constant);
            let d = e.squared_distance(Complex::new(0.0, 0.0));
            if d > 4.0 {
                return false;
            }
        }
        true
    }

    fn mandel(i: Complex, constant: Complex) -> Complex {
        i.square() + constant
    }
}
