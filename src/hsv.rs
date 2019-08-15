pub use smart_leds_trait::*;

#[derive(Copy, Clone, Default)]
pub struct Hsv {
    pub hue: u8,
    pub sat: u8,
    pub val: u8,
}

/// Converts a hsv value into RGB values. Because the hsv values are integers, the precision of the
/// resulting RGB value is limited to +- 4
/// Example:
/// ```
/// use smart_leds::hsv::{hsv2rgb, Hsv};
/// let hsv = Hsv{hue: 89, sat: 230, val: 42};
/// let conv_rgb = hsv2rgb(hsv);
/// // will return RGB { r: 4, g: 41, b: 8},
/// ```
pub fn hsv2rgb(hsv: Hsv) -> RGB8 {
    let v: u16 = hsv.val as u16;
    let s: u16 = hsv.sat as u16;
    let f: u16 = (hsv.hue as u16 * 2 % 85) * 3; // relative interval

    let p: u16 = v * (255 - s) / 255;
    let q: u16 = v * (255 - (s * f) / 255) / 255;
    let t: u16 = v * (255 - (s * (255 - f)) / 255) / 255;
    match hsv.hue {
           0..=42 => RGB{r: v as u8, g: t as u8, b: p as u8},
          43..=84 => RGB{r: q as u8, g: v as u8, b: p as u8},
         85..=127 => RGB{r: p as u8, g: v as u8, b: t as u8},
        128..=169 => RGB{r: p as u8, g: q as u8, b: v as u8},
        170..=212 => RGB{r: t as u8, g: p as u8, b: v as u8},
        213..=254 => RGB{r: v as u8, g: p as u8, b: q as u8},
              255 => RGB{r: v as u8, g: t as u8, b: p as u8},
    }
}

#[cfg(test)]
mod test {

    use super::*;

    fn distance(i: u8, j: u8) -> u8 {
        if i < j {
            j - i
        } else {
            i - j
        }
    }

    #[test]
    fn test_hsv2rgb_1() {
        #[rustfmt::skip]
        let hsv = [
            Hsv{hue:   0, sat: 255, val: 255},
            Hsv{hue:  21, sat: 255, val: 255},
            Hsv{hue:  42, sat: 255, val: 255},
            Hsv{hue:  64, sat: 255, val: 255},
            Hsv{hue:  85, sat: 255, val: 255},
            Hsv{hue: 106, sat: 255, val: 255},
            Hsv{hue: 127, sat: 255, val: 255},
            Hsv{hue: 149, sat: 255, val: 255},
            Hsv{hue: 170, sat: 255, val: 255},
            Hsv{hue: 191, sat: 255, val: 255},
            Hsv{hue: 212, sat: 255, val: 255},
            Hsv{hue: 234, sat: 255, val: 255},
            Hsv{hue: 255, sat: 255, val: 255},
            Hsv{hue: 111, sat: 123, val:  35},
            Hsv{hue:  21, sat:   3, val: 138},
            Hsv{hue:  89, sat: 230, val:  42},
        ];
        #[rustfmt::skip]
        let rgb = [
            RGB { r: 255, g: 0  , b:   0},
            RGB { r: 255, g: 127, b:   0},
            RGB { r: 255, g: 255, b:   0},
            RGB { r: 127, g: 255, b:   0},
            RGB { r:   0, g: 255, b:   0},
            RGB { r:   0, g: 255, b: 127},
            RGB { r:   0, g: 255, b: 255},
            RGB { r:   0, g: 127, b: 255},
            RGB { r:   0, g: 0  , b: 255},
            RGB { r: 127, g: 0  , b: 255},
            RGB { r: 255, g: 0  , b: 255},
            RGB { r: 255, g: 0  , b: 127},
            RGB { r: 255, g: 0  , b:   0},
            RGB { r:  19, g:  35, b:  29},
            RGB { r: 137, g: 137, b: 136},
            RGB { r:   4, g:  41, b:   8},
        ];

        for i in 0..hsv.len() {
            let new_hsv = hsv2rgb(hsv[i]);
            assert!(distance(new_hsv.r, rgb[i].r) < 4);
            assert!(distance(new_hsv.g, rgb[i].g) < 4);
            assert!(distance(new_hsv.b, rgb[i].b) < 4);
        }
    }

    #[test]
    // if sat == 0 then all colors are equal
    fn test_hsv2rgb_2() {
        for i in 0..=255 {
            #[rustfmt::skip]
            let rgb = hsv2rgb(Hsv{hue:  i, sat: 0, val:  42});
            assert! {rgb.r == rgb.b};
            assert! {rgb.b == rgb.g};
        }
    }
}
