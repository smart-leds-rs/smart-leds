pub use smart_leds_trait::*;

#[derive(Copy, Clone, Default)]
pub struct Hsv {
    pub hue: u8,
    pub sat: u8,
    pub val: u8,
}

const HSV_SECTION_3: u8 = 0x40;
// Mostly inspired by https://github.com/FastLED/FastLED/blob/a50d96d733b5c6c9b95b6e027fd2e760823feae8/hsv2rgb.cpp#L71
pub fn hsv2rgb(hsv: Hsv) -> RGB8 {
    let value = hsv.val;
    let saturation = hsv.sat;

    // The brightness value is the minimum of r, g & b
    let invsat = 255 - saturation;
    let brightness_floor = ((value as u16 * invsat as u16) / 256) as u8;

    // The maximum amount that will be added to r, g or b on top of brightness_floor
    let color_amplitude = value - brightness_floor;

    // Figure out in which section of the hue wheel we're in and where we are
    // in that section
    let section = hsv.hue / HSV_SECTION_3;
    let offset = hsv.hue % HSV_SECTION_3;
    let rampup = offset;
    let rampdown = HSV_SECTION_3 - 1 - offset;

    // RGB8-amplitude-scaled down versions of rampdown/rampup
    let rampup_amp_adj = ((rampup as u16 * color_amplitude as u16) / 64) as u8;
    let rampdown_amp_adj = ((rampdown as u16 * color_amplitude as u16) / 64) as u8;

    // with brightness floor
    let rampup_adj_with_floor = rampup_amp_adj + brightness_floor;
    let rampdown_adj_with_floor = rampdown_amp_adj+ brightness_floor;

    // Figure out where in the color wheel we are
    match section {
        0 => RGB8 {
            r: rampdown_adj_with_floor,
            g: rampup_adj_with_floor,
            b: brightness_floor,
        },
        1 => RGB8 {
            r: brightness_floor,
            g: rampdown_adj_with_floor,
            b: rampup_adj_with_floor,
        },
        2 => RGB8 {
            r: rampup_adj_with_floor,
            g: brightness_floor,
            b: rampdown_adj_with_floor,
        },
        _ => panic!("Hue value must be between 0 and 192")
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
