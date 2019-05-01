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
    let rampup = ((rampup as u16 * color_amplitude as u16) / 64) as u8;
    let rampdown = ((rampdown as u16 * color_amplitude as u16) / 64) as u8;

    // with brightness floor
    let rampup = rampup + brightness_floor;
    let rampdown = rampdown + brightness_floor;

    // Figure out where in the color wheel we are
    match section {
        0 => RGB8 {
            r: rampdown,
            g: rampup,
            b: brightness_floor,
        },
        1 => RGB8 {
            r: rampup,
            g: brightness_floor,
            b: rampdown,
        },
        2 => RGB8 {
            r: brightness_floor,
            g: rampdown,
            b: rampup,
        },
        // An 8 bit value modulo 0x40 can't be more than 2
        _ => unreachable!(),
    }
}
