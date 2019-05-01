#![no_std]

pub mod colors;
pub mod hsv;

pub use smart_leds_trait::*;

/// An iterator that provides brightness reduction
pub struct Brightness<I> {
    iter: I,
    brightness: u8,
}

impl<'a, I> Iterator for Brightness<I>
where
    I: Iterator<Item = RGB8>,
{
    type Item = RGB8;

    fn next(&mut self) -> Option<RGB8> {
        self.iter.next().map(|a| RGB8 {
            r: (a.r as u16 * (self.brightness as u16 + 1) / 256) as u8,
            g: (a.g as u16 * (self.brightness as u16 + 1) / 256) as u8,
            b: (a.b as u16 * (self.brightness as u16 + 1) / 256) as u8,
        })
    }
}

/// Pass your iterator into this function to get reduced brightness
pub fn brightness<I>(iter: I, brightness: u8) -> Brightness<I>
where
    I: Iterator<Item = RGB8>,
{
    Brightness { iter, brightness }
}

/// An iterator that provides gamma correction.
/// Makes the colour distribution non-linear, to match your eyes' perception
/// In other words, makes orange look orange.
/// If using in combination with a brightness reduction, apply the gamma
/// correction first, then the brightness reduction
/// ie: brightness(gamma(data.iter().cloned()), 32)
pub struct Gamma<I> {
    iter: I,
}

impl<'a, I> Iterator for Gamma<I>
where
    I: Iterator<Item = RGB8>,
{
    type Item = RGB8;

    fn next(&mut self) -> Option<RGB8> {
        // This table remaps linear input values
        // (the numbers we’d like to use; e.g. 127 = half brightness)
        // to nonlinear gamma-corrected output values
        // (numbers producing the desired effect on the LED;
        // e.g. 36 = half brightness).
        const GAMMA8: [u8; 256] = [
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 3, 3, 3, 3, 3, 3, 3, 4, 4,
            4, 4, 4, 5, 5, 5, 5, 6, 6, 6, 6, 7, 7, 7, 7, 8, 8, 8, 9, 9, 9, 10, 10, 10, 11, 11, 11,
            12, 12, 13, 13, 13, 14, 14, 15, 15, 16, 16, 17, 17, 18, 18, 19, 19, 20, 20, 21, 21, 22,
            22, 23, 24, 24, 25, 25, 26, 27, 27, 28, 29, 29, 30, 31, 32, 32, 33, 34, 35, 35, 36, 37,
            38, 39, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 50, 51, 52, 54, 55, 56, 57, 58,
            59, 60, 61, 62, 63, 64, 66, 67, 68, 69, 70, 72, 73, 74, 75, 77, 78, 79, 81, 82, 83, 85,
            86, 87, 89, 90, 92, 93, 95, 96, 98, 99, 101, 102, 104, 105, 107, 109, 110, 112, 114,
            115, 117, 119, 120, 122, 124, 126, 127, 129, 131, 133, 135, 137, 138, 140, 142, 144,
            146, 148, 150, 152, 154, 156, 158, 160, 162, 164, 167, 169, 171, 173, 175, 177, 180,
            182, 184, 186, 189, 191, 193, 196, 198, 200, 203, 205, 208, 210, 213, 215, 218, 220,
            223, 225, 228, 231, 233, 236, 239, 241, 244, 247, 249, 252, 255,
        ];
        self.iter.next().map(|a| RGB8 {
            r: GAMMA8[a.r as usize],
            g: GAMMA8[a.g as usize],
            b: GAMMA8[a.b as usize],
        })
    }
}

/// Pass your iterator into this function to get corrected gamma
pub fn gamma<I>(iter: I) -> Gamma<I>
where
    I: Iterator<Item = RGB8>,
{
    Gamma { iter }
}
