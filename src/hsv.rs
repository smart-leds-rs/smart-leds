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
    let value = ((u16::from(hsv.val) * 192) / 256) as u8;
    let saturation = hsv.sat;

    // The brightness value is the minimum of r, g & b
    let invsat = 255 - saturation;
    let brightness_floor = ((u16::from(value) * u16::from(invsat)) / 256) as u8;

    // The maximum amount that will be added to r, g or b on top of brightness_floor
    let color_amplitude = value - brightness_floor;

    // Figure out in which section of the hue wheel we're in and where we are
    // in that section
    let section = hsv.hue / HSV_SECTION_3;
    let offset = hsv.hue % HSV_SECTION_3;
    let rampup = offset;
    let rampdown = HSV_SECTION_3 - 1 - offset;

    // RGB8-amplitude-scaled down versions of rampdown/rampup
    let rampup_amp_adj = ((u16::from(rampup) * u16::from(color_amplitude)) / 64) as u8;
    let rampdown_amp_adj = ((u16::from(rampdown) as u16 * u16::from(color_amplitude)) / 64) as u8;

    // with brightness floor
    let rampup_adj_with_floor = rampup_amp_adj + brightness_floor;
    let rampdown_adj_with_floor = rampdown_amp_adj + brightness_floor;

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
        _ => unreachable!(),
    }
}
