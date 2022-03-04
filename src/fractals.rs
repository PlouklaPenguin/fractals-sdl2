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

    pub fn distance(&self, b: Self) -> f64 {
        ((self.r + b.r).powi(2) + (self.i + b.i).powi(2)).sqrt()
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
        depth: u32,
        _screen_width: i128,
        _screen_height: i128,
        canvas: &mut Canvas<Window>,
    ) -> Result<(), String> {
        //let screen_width = screen_width * 10_i128.pow(depth);
        //println!("{}", screen_width);
        //let screen_height = screen_height * (10_i128.pow(depth));
        canvas.set_draw_color(Color::RGB(255, 255, 255));

        // take distance on y axis (2) and multiply it by 2^x to expand it to an int
        let yrange: i32 = 2 * (2_i32.pow(depth));
        // -2 to 2
        let xrange = 4 * (2_i32.pow(depth));
        println!("xrange: {} yrange: {}", xrange, yrange);

        // For every value in between -2 and 2
        /*
        TODO: Make the for loop such that it only draws points between 0 and 400 / 0 and 300
        This needs to
        */
        for x in -xrange..xrange {
            for y in -yrange..yrange {
                //if x < 0 && x >= -400 && y < 0 && y >= -300 {
                    if is_in_set(Complex::new(
                        (x as f64) / ((2_i128.pow(depth)) as f64),
                        (y as f64) / ((2_i128.pow(depth)) as f64),
                    )) {
                        canvas.draw_point(((x ) as i32, (y ) as i32))?;
                        if x == -2 && y == 0 {
                            println!("oh");
                        }
                        //println!("x = {} y = {}", (x as f64), (y as f64));
                    }
                    //canvas.present();
                }
            //}
        }

        Ok(())
    }
    pub fn is_in_set(constant: Complex) -> bool {
        let mut e = Complex::new(0.0, 0.0) + constant;

        for _i in 0..16 {
            e = mandel(e, constant);
        }
        e.distance(Complex::new(0.0, 0.0)) <= 1.0
    }

    fn mandel(i: Complex, constant: Complex) -> Complex {
        i.square() + constant
    }
}
