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

    pub fn sq_distance_compl(&self, b: Self) -> f64 {
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
    //use std::num::TryFromIntError;

    use sdl2::{pixels::Color, render::Canvas, video::Window};

    use super::Complex;

    #[derive(PartialEq)]
    pub struct MandelbrotRender {
        screen_width: i32,
        screen_height: i32,
        loc: (i32, i32),
        zoom: f32,
    }

    impl MandelbrotRender {
        pub fn new(screen_width: i32, screen_height: i32, loc: (i32, i32), zoom: f32) -> Self {
            MandelbrotRender {
                screen_width,
                screen_height,
                loc,
                zoom,
            }
        }
    }

    ///Draws all points on the screen within the Mandelbrot set, at a specific centre
    pub fn generate_window(
        screen_width: i32,
        screen_height: i32,
        canvas: &mut Canvas<Window>,
        current_loc: (i32, i32),
        zoom: f32,
        past_mandelbrot: &MandelbrotRender,
    ) -> Result<MandelbrotRender, String> {
        /*
        TODO: Switch to GPU; enable movements by mouse-drag; colours?
         */
        let current = MandelbrotRender::new(screen_width, screen_width, current_loc, zoom);
        if current != *past_mandelbrot {
            canvas.set_draw_color(Color::RGB(255, 255, 255));

            let screen_centre = (screen_width / 2, screen_height / 2);

            let zoomed_divisor = 10_f32.powf(
                zoom, /*.try_into()
                     .map_err(|e: TryFromIntError| e.to_string())?,*/
            );

            let frac_centre = (
                2 * screen_centre.0 + -current_loc.0,
                2 * screen_centre.1 + -current_loc.1,
            );

            println!(
                "past loc: {:?}, current loc: {:?}",
                past_mandelbrot.loc,
                current_loc
            );
            /*let mouse_loc = (
                (mouse_loc.0 - (screen_width / 2)) * -zoom + (screen_width / 2),
                (mouse_loc.1 - (screen_height / 2)) * -zoom + (screen_height / 2),
            );*/

            for x in (-screen_width/* / 2 */)..(screen_width/* / 2 */) {
                for y in (-screen_height/* / 2 */)..(screen_height/* / 2 */) {
                    let centered_x = (x as i32) + frac_centre.0;
                    let centered_y = (y as i32) + frac_centre.1;

                    if (centered_x < screen_width - 10 && centered_x > 10)
                        && (centered_y < screen_height - 10 && centered_y > 10)
                    {
                        draw(x, y, canvas, frac_centre, zoomed_divisor)?;
                    }
                }
            }
        }
        Ok(current)
    }
    /// Calculates if a complex number is in a set
    pub fn is_in_set(constant: Complex) -> bool {
        let mut e = Complex::new(0.0, 0.0) + constant;

        for _i in 0..32 {
            e = mandel(e, constant);
            let d = e.sq_distance_compl(Complex::new(0.0, 0.0));
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
        mouse_loc: (i32, i32),
        zoomed_divisor: f32,
    ) -> Result<(), String> {
        if is_in_set(Complex::new(
            x as f64 / zoomed_divisor as f64,
            y as f64 / zoomed_divisor as f64,
        )) {
            canvas.draw_point((x + mouse_loc.0, y + mouse_loc.1))?;
        }
        Ok(())
    }
}
