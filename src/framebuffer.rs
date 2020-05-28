use crate::{
    display::SimulatorDisplay, 
    output_settings::OutputSettings};
use embedded_graphics::{
    drawable::{Drawable, Pixel},
    geometry::{//Dimensions, 
    Point, Size},
    pixelcolor::{PixelColor, Rgb888, RgbColor},
    primitives::{Primitive, Rectangle},
    style::{PrimitiveStyle, Styled},
    DrawTarget,
};
 use alloc::boxed::Box;
 use core::convert::TryFrom;
use crate::Vga;
#[macro_use]
use crate::print;

/// Rgb888 framebuffer
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Framebuffer {
    size: Size,
    pub(crate) data: Box<[u8]>,
    pub(crate) output_settings: OutputSettings,
}

impl Framebuffer {
    /// Creates a new framebuffer filled with `background_color`.
    pub fn new<C>(display: &SimulatorDisplay<C>, output_settings: &OutputSettings) -> Self
    where
        C: PixelColor + Into<Rgb888>,
    {
        let size = output_settings.framebuffer_size(display);
        print!("new framebuffer size  x {} y {}\n",size.width,size.height);
        // Create an empty pixel buffer.
        let pixel_count = size.width as usize * size.height as usize;
        print!("size  pixelcount{}\n",pixel_count);
        let data = vec![0; pixel_count * 4].into_boxed_slice();

        let mut framebuffer = Self {
            size,
            data,
            output_settings: output_settings.clone(),
        };

        // Fill pixel buffer with background color.
        let background_color = output_settings.theme.convert(Rgb888::BLACK);
        framebuffer.clear(background_color).unwrap();

        // Update buffer.
        //framebuffer.update(display,v);

        framebuffer
    }

    /// Updates the framebuffer from a `SimulatorDisplay`.
    pub fn update<C>(&mut self, display: &SimulatorDisplay<C>)
    where
        C: PixelColor + Into<Rgb888>,
    {
        let Size { width, height } = display.size();

        let pixel_pitch = (self.output_settings.scale + self.output_settings.pixel_spacing) as i32;
        //let pixel_size = Size::new(self.output_settings.scale, self.output_settings.scale);
        let ptr: *const u8 = self.data.as_ptr();
        print!("framebuffer update\n");
        unsafe {

            Vga::setPointer(ptr);
        }
        for y in 0..height as i32 {
            for x in 0..width as i32 {
                let color = display.get_pixel(Point::new(x, y)).into();
                let p = Point::new(x * pixel_pitch, y * pixel_pitch);
                let p1 = Point::new(p.x +1, p.y+1);

                Rectangle::new(p, p1)
                    .into_styled(PrimitiveStyle::with_fill(
                        self.output_settings.theme.convert(color),
                    ))
                    .draw(self)
                    .ok();
            }
        }
    }

    fn get_pixel_mut(&mut self, point: Point) -> Option<&mut [u8]> {
        if let Ok((x, y)) = <(u32, u32)>::try_from(point) {
            if x < self.size.width && y < self.size.height {
                let start_index = (x + y * self.size.width) as usize * 4;
                return self.data.get_mut(start_index..start_index + 4);
            }
        }

        None
    }

    
}

impl DrawTarget<Rgb888> for Framebuffer {
    type Error = core::convert::Infallible;

    fn draw_pixel(&mut self, pixel: Pixel<Rgb888>) -> Result<(), Self::Error> {
        let Pixel(point, color) = pixel;

        if let Some(pixel) = self.get_pixel_mut(point) {
            // swapped b and r and added in the 0
            pixel.copy_from_slice(&[0,color.b(), color.g(), color.r()]);
        }

        Ok(())
    }

    fn draw_rectangle(
        &mut self,
        item: &Styled<Rectangle, PrimitiveStyle<Rgb888>>,
    ) -> Result<(), Self::Error> {
        if item.style.stroke_color.is_some() && item.style.stroke_width != 0 {
            return self.draw_iter(item);
        }

        if let Some(fill_color) = item.style.fill_color {
            let color = &[0,fill_color.r(), fill_color.g(), fill_color.b()];
            let p = item.primitive.top_left;
            if let Some(pixel) = self.get_pixel_mut(p) {
                      pixel.copy_from_slice(color);

                   }
           // for p in item.primitive.bounding_box().points() {
            //    if let Some(pixel) = self.get_pixel_mut(p) {
             //       pixel.copy_from_slice(color);
             //   }
           // }
        }

        Ok(())
    }

    fn size(&self) -> Size {
        self.size
    }
}

