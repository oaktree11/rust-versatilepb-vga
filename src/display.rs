//use crate::{
//    framebuffer::Framebuffer, 
 //   output_settings::OutputSettings};
use embedded_graphics::{
    drawable::Pixel,
    geometry::{Point, Size},
    pixelcolor::{BinaryColor, PixelColor},//, Rgb888},
    DrawTarget,
};
use alloc::boxed::Box;
use alloc::{vec, vec::Vec};
use core::convert::TryFrom;
/// Simulator display.
pub struct SimulatorDisplay<C> {
    size: Size,
    pixels: Box<[C]>,
}

impl<C> SimulatorDisplay<C>
where
    C: PixelColor,
{
    /// Creates a new display filled with a color.
    ///
    /// This constructor can be used if `C` doesn't implement `From<BinaryColor>` or another
    /// default color is wanted.
    pub fn with_default_color(size: Size, default_color: C) -> Self {
        let pixel_count = size.width as usize * size.height as usize;
        let pixels = vec![default_color; pixel_count].into_boxed_slice();

        SimulatorDisplay { size, pixels }
    }

    /// Returns the color of the pixel at a point.
    ///
    /// # Panics
    ///
    /// Panics if `point` is outside the display.
    pub fn get_pixel(&self, point: Point) -> C {
        self.point_to_index(point)
            .and_then(|index| self.pixels.get(index).copied())
            .expect("can't get point outside of display")
    }

    fn point_to_index(&self, point: Point) -> Option<usize> {
        if let Ok((x, y)) = <(u32, u32)>::try_from(point) {
            if x < self.size.width && y < self.size.height {
                return Some((x + y * self.size.width) as usize);
            }
        }

        None
    }
}

impl<C> SimulatorDisplay<C>
where
    C: PixelColor + From<BinaryColor>,
{
    /// Creates a new display.
    ///
    /// The display is filled with `C::from(BinaryColor::Off)`.
    pub fn new(size: Size) -> Self {
        Self::with_default_color(size, C::from(BinaryColor::Off))
    }
}


impl<C> DrawTarget<C> for SimulatorDisplay<C>
where
    C: PixelColor,
{
    type Error = core::convert::Infallible;

    fn draw_pixel(&mut self, pixel: Pixel<C>) -> Result<(), Self::Error> {
        let Pixel(point, color) = pixel;

        if let Some(index) = self.point_to_index(point) {
            self.pixels[index] = color;
        }

        Ok(())
    }

    fn size(&self) -> Size {
        self.size
    }
}
