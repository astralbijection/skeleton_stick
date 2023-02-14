use std::sync::{Arc, RwLock};

use embedded_graphics::{
    pixelcolor::BinaryColor,
    prelude::{Dimensions, DrawTarget},
};
use embedded_graphics_simulator::SimulatorDisplay;

#[derive(Clone)]
pub struct MultithreadSimDisplay {
    pub arc: Arc<RwLock<SimulatorDisplay<BinaryColor>>>,
}

impl MultithreadSimDisplay {
    pub fn new(arc: Arc<RwLock<SimulatorDisplay<BinaryColor>>>) -> Self {
        Self { arc }
    }
}

impl Dimensions for MultithreadSimDisplay {
    fn bounding_box(&self) -> embedded_graphics::primitives::Rectangle {
        self.arc.read().unwrap().bounding_box()
    }
}

impl DrawTarget for MultithreadSimDisplay {
    type Color = BinaryColor;

    type Error = <SimulatorDisplay<BinaryColor> as DrawTarget>::Error;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = embedded_graphics::Pixel<Self::Color>>,
    {
        self.arc.write().unwrap().draw_iter(pixels)
    }

    fn fill_contiguous<I>(
        &mut self,
        area: &embedded_graphics::primitives::Rectangle,
        colors: I,
    ) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Self::Color>,
    {
        self.arc.write().unwrap().fill_contiguous(area, colors)
    }

    fn fill_solid(
        &mut self,
        area: &embedded_graphics::primitives::Rectangle,
        color: Self::Color,
    ) -> Result<(), Self::Error> {
        self.arc.write().unwrap().fill_solid(area, color)
    }

    fn clear(&mut self, color: Self::Color) -> Result<(), Self::Error> {
        self.arc
            .write()
            .unwrap()
            .fill_solid(&self.bounding_box(), color)
    }
}
