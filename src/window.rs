use crate::{vga, display::SimulatorDisplay, framebuffer::Framebuffer, output_settings::OutputSettings};
use embedded_graphics::{
    geometry::{Point, Size},
    pixelcolor::{PixelColor, Rgb888},
    DrawTarget,
};




/// Simulator window
#[allow(dead_code)]
pub struct Window {
    framebuffer: Option<Framebuffer>,
    output_settings: OutputSettings,
}

impl Window {
    /// Creates a new simulator window.
    pub fn new(output_settings: &OutputSettings) -> Self {
        Self {
            framebuffer: None,
            output_settings: output_settings.clone(),
        }
    }

    /// Updates the window.
    pub fn update<C>(&mut self, display: &SimulatorDisplay<C>)
    where
        C: PixelColor + Into<Rgb888>,
    {

        if self.framebuffer.is_none() {
            self.framebuffer = Some(Framebuffer::new(display, &self.output_settings));
        }

        let framebuffer = self.framebuffer.as_mut().unwrap();

        framebuffer.update(display);
    }

    /// Shows a static display.
    ///
    /// This methods updates the window once and loops until the simulator window
    /// is closed.
    pub fn show_static<C>(&mut self, display: &SimulatorDisplay<C>)
    where
        C: PixelColor + Into<Rgb888>,
    {
        //self.update(&display);


    }


}

