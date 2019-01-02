#![no_std]

pub mod colors;

pub use smart_leds_trait::*;

/// An iterator that provides brightness reduction
pub struct Brightness<I> {
    iter: I,
    brightness: u8,
}

impl<'a, I> Iterator for Brightness<I>
where
    I: Iterator<Item = Color>,
{
    type Item = Color;

    fn next(&mut self) -> Option<Color> {
        self.iter.next().map(|a| Color {
            r: (a.r as u32 * self.brightness as u32 / 256) as u8,
            g: (a.g as u32 * self.brightness as u32 / 256) as u8,
            b: (a.b as u32 * self.brightness as u32 / 256) as u8,
        })
    }
}

/// Pass your iterator into this function to get reduced brightness
pub fn brightness<I>(iter: I, brightness: u8) -> Brightness<I>
where
    I: Iterator<Item = Color>,
{
    Brightness { iter, brightness }
}
