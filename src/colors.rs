use crate::Color;

/// See https://en.wikipedia.org/wiki/Web_colors

// 16 Original "Web" Colors
pub const WHITE:                    Color = Color { r: 0xFF, g: 0xFF, b: 0xFF };
pub const SILVER:                   Color = Color { r: 0xC0, g: 0xC0, b: 0xC0 };
pub const GRAY:                     Color = Color { r: 0x80, g: 0x80, b: 0x80 };
pub const BLACK:                    Color = Color { r: 0x00, g: 0x00, b: 0x00 };
pub const RED:                      Color = Color { r: 0xFF, g: 0x00, b: 0x00 };
pub const MAROON:                   Color = Color { r: 0x80, g: 0x00, b: 0x00 };
pub const YELLOW:                   Color = Color { r: 0xFF, g: 0xFF, b: 0x00 };
pub const OLIVE:                    Color = Color { r: 0x80, g: 0x80, b: 0x00 };
pub const LIME:                     Color = Color { r: 0x00, g: 0xFF, b: 0x00 };
pub const GREEN:                    Color = Color { r: 0x00, g: 0x80, b: 0x00 };
pub const AQUA:                     Color = Color { r: 0x00, g: 0xFF, b: 0xFF };
pub const TEAL:                     Color = Color { r: 0x00, g: 0x80, b: 0x80 };
pub const BLUE:                     Color = Color { r: 0x00, g: 0x00, b: 0xFF };
pub const NAVY:                     Color = Color { r: 0x00, g: 0x00, b: 0x80 };
pub const FUCHSIA:                  Color = Color { r: 0xFF, g: 0x00, b: 0xFF };
pub const PURPLE:                   Color = Color { r: 0x80, g: 0x00, b: 0x80 };

// Extended "X11" Colors
pub const PINK:                     Color = Color { r: 0xFF, g: 0xC0, b: 0xCB };
pub const LIGHT_PINK:               Color = Color { r: 0xFF, g: 0xB6, b: 0xC1 };
pub const HOT_PINK:                 Color = Color { r: 0xFF, g: 0x69, b: 0xB4 };
pub const DEEP_PINK:                Color = Color { r: 0xFF, g: 0x14, b: 0x93 };
pub const PALE_VIOLET_RED:          Color = Color { r: 0xDB, g: 0x70, b: 0x93 };
pub const MEDIUM_VIOLET_RED:        Color = Color { r: 0xC7, g: 0x15, b: 0x85 };

pub const LIGHT_SALMON:             Color = Color { r: 0xFF, g: 0xA0, b: 0x7A };
pub const SALMON:                   Color = Color { r: 0xFA, g: 0x80, b: 0x72 };
pub const DARK_SALMON:              Color = Color { r: 0xE9, g: 0x96, b: 0x7A };
pub const LIGHT_CORAL:              Color = Color { r: 0xF0, g: 0x80, b: 0x80 };
pub const INDIAN_RED:               Color = Color { r: 0xCD, g: 0x5C, b: 0x5C };
pub const CRIMSON:                  Color = Color { r: 0xDC, g: 0x14, b: 0x3C };
pub const FIREBRICK:                Color = Color { r: 0xB2, g: 0x22, b: 0x22 };
pub const DARK_RED:                 Color = Color { r: 0x8B, g: 0x00, b: 0x00 };

pub const ORANGE_RED:               Color = Color { r: 0xFF, g: 0x45, b: 0x00 };
pub const TOMATO:                   Color = Color { r: 0xFF, g: 0x63, b: 0x47 };
pub const CORAL:                    Color = Color { r: 0xFF, g: 0x7f, b: 0x50 }; 
pub const DARK_ORANGE:              Color = Color { r: 0xFF, g: 0x8C, b: 0x00 }; 
pub const ORANGE:                   Color = Color { r: 0xFF, g: 0xA5, b: 0x00 }; 

pub const LIGHT_YELLOW:             Color = Color { r: 0xFF, g: 0xFF, b: 0xE0 };
pub const LEMON_CHIFFON:            Color = Color { r: 0xFF, g: 0xFA, b: 0xCD };
pub const LIGHT_GOLDENROD_YELLOW:   Color = Color { r: 0xFA, g: 0xFA, b: 0xD2 };
pub const PAPAYA_WHIP:              Color = Color { r: 0xFF, g: 0xEF, b: 0xD5 };
pub const MOCCASIN:                 Color = Color { r: 0xFF, g: 0xE4, b: 0xB5 };
pub const PEACH_PUFF:               Color = Color { r: 0xFF, g: 0xDA, b: 0xB9 };
pub const PALE_GOLDENROD:           Color = Color { r: 0xEE, g: 0xE8, b: 0xAA };
pub const KHAKI:                    Color = Color { r: 0xF0, g: 0xE6, b: 0x8C };
pub const DARK_KHAKI:               Color = Color { r: 0xBD, g: 0xB7, b: 0x6B };
pub const GOLD:                     Color = Color { r: 0xFF, g: 0xD7, b: 0x00 };

pub const CORNSILK:                 Color = Color { r: 0xFF, g: 0xF8, b: 0xDC };
pub const BLANCHED_ALMOND:          Color = Color { r: 0xFF, g: 0xEB, b: 0xCD }; 
pub const BISQUE:                   Color = Color { r: 0xFF, g: 0xE4, b: 0xC4 }; 
pub const NAVAJO_WHITE:             Color = Color { r: 0xFF, g: 0xDE, b: 0xAD };
pub const WHEAT:                    Color = Color { r: 0xF5, g: 0xDE, b: 0xB3 };
pub const BURLYWOOD:                Color = Color { r: 0xDE, g: 0xB8, b: 0x87 };
pub const TAN:                      Color = Color { r: 0xD2, g: 0xB4, b: 0x8C };
pub const ROSY_BROWN:               Color = Color { r: 0xBC, g: 0x8F, b: 0x8F };
pub const SANDY_BROWN:              Color = Color { r: 0xF4, g: 0xA4, b: 0x60 };
pub const GOLDENROD:                Color = Color { r: 0xDA, g: 0xA5, b: 0x20 };
pub const DARK_GOLDENROD:           Color = Color { r: 0xB8, g: 0x86, b: 0x0B };
pub const PERU:                     Color = Color { r: 0xCD, g: 0x85, b: 0x3F };
pub const CHOCOLATE:                Color = Color { r: 0xD2, g: 0x69, b: 0x1E };
pub const SADDLE_BROWN:             Color = Color { r: 0x8B, g: 0x45, b: 0x13 };
pub const SIENNA:                   Color = Color { r: 0xA0, g: 0x52, b: 0x2D };
pub const BROWN:                    Color = Color { r: 0xA5, g: 0x2A, b: 0x2A };

pub const DARK_OLIVE_GREEN:         Color = Color { r: 0x55, g: 0x6B, b: 0x2F };
pub const OLIVE_DRAB:               Color = Color { r: 0x6B, g: 0x8E, b: 0x23 };
pub const YELLOW_GREEN:             Color = Color { r: 0x9A, g: 0xCD, b: 0x32 };
pub const LIME_GREEN:               Color = Color { r: 0x32, g: 0xCD, b: 0x32 };
pub const LAWN_GREEN:               Color = Color { r: 0x7C, g: 0xFC, b: 0x00 };
pub const CHARTREUSE:               Color = Color { r: 0x7F, g: 0xFF, b: 0x00 };
pub const GREEN_YELLOW:             Color = Color { r: 0xAD, g: 0xFF, b: 0x2F };
pub const SPRING_GREEN:             Color = Color { r: 0x00, g: 0xFF, b: 0x7F };
pub const MEDIUM_SPRING_GREEN:      Color = Color { r: 0x00, g: 0xFA, b: 0x9A };
pub const LIGHT_GREEN:              Color = Color { r: 0x90, g: 0xEE, b: 0x90 };
pub const PALE_GREEN:               Color = Color { r: 0x98, g: 0xFB, b: 0x98 };
pub const DARK_SEA_GREEN:           Color = Color { r: 0x8F, g: 0xBC, b: 0x8F };
pub const MEDIUM_AQUAMARINE:        Color = Color { r: 0x66, g: 0xCD, b: 0xAA };
pub const MEDIUM_SEA_GREEN:         Color = Color { r: 0x3C, g: 0xB3, b: 0x71 };
pub const SEA_GREEN:                Color = Color { r: 0x2E, g: 0x8B, b: 0x57 };
pub const FOREST_GREEN:             Color = Color { r: 0x22, g: 0x8B, b: 0x22 };
pub const DARK_GREEN:               Color = Color { r: 0x00, g: 0x64, b: 0x00 };

pub const CYAN:                     Color = Color { r: 0x00, g: 0xFF, b: 0xFF };
pub const LIGHT_CYAN:               Color = Color { r: 0xE0, g: 0xFF, b: 0xFF };
pub const PALE_TURQUOISE:           Color = Color { r: 0xAF, g: 0xEE, b: 0xEE };
pub const AQUAMARINE:               Color = Color { r: 0x7F, g: 0xFF, b: 0xD4 };
pub const TURQUOISE:                Color = Color { r: 0x40, g: 0xE0, b: 0xD0 };
pub const MEDIUM_TURQUOISE:         Color = Color { r: 0x48, g: 0xD1, b: 0xCC };
pub const DARK_TURQUOISE:           Color = Color { r: 0x00, g: 0xCE, b: 0xD1 };
pub const LIGHT_SEA_GREEN:          Color = Color { r: 0x20, g: 0xB2, b: 0xAA };
pub const CADET_BLUE:               Color = Color { r: 0x5F, g: 0x9E, b: 0xA0 };
pub const DARK_CYAN:                Color = Color { r: 0x00, g: 0x8B, b: 0x8B };

pub const LIGHT_STEEL_BLUE:         Color = Color { r: 0xB0, g: 0xC4, b: 0xDE };
pub const POWDER_BLUE:              Color = Color { r: 0xB0, g: 0xE0, b: 0xE6 };
pub const LIGHT_BLUE:               Color = Color { r: 0xAD, g: 0xD8, b: 0xE6 };
pub const SKY_BLUE:                 Color = Color { r: 0x87, g: 0xCE, b: 0xEB };
pub const LIGHT_SKY_BLUE:           Color = Color { r: 0x87, g: 0xCE, b: 0xFA };
pub const DEEP_SKY_BLUE:            Color = Color { r: 0x00, g: 0xBF, b: 0xFF };
pub const DODGER_BLUE:              Color = Color { r: 0x1E, g: 0x90, b: 0xFF };
pub const CORNFLOWER_BLUE:          Color = Color { r: 0x64, g: 0x95, b: 0xED };
pub const STEEL_BLUE:               Color = Color { r: 0x46, g: 0x82, b: 0xB4 };
pub const ROYAL_BLUE:               Color = Color { r: 0x41, g: 0x69, b: 0xE1 };
pub const MEDIUM_BLUE:              Color = Color { r: 0x00, g: 0x00, b: 0xCD };
pub const DARK_BLUE:                Color = Color { r: 0x00, g: 0x00, b: 0x8B };
pub const MIDNIGHT_BLUE:            Color = Color { r: 0x19, g: 0x19, b: 0x70 };

pub const LAVENDER:                 Color = Color { r: 0xE6, g: 0xE6, b: 0xFA };
pub const THISTLE:                  Color = Color { r: 0xD8, g: 0xBF, b: 0xD8 };
pub const PLUM:                     Color = Color { r: 0xDD, g: 0xA0, b: 0xDD };
pub const VIOLET:                   Color = Color { r: 0xEE, g: 0x82, b: 0xEE };
pub const ORCHID:                   Color = Color { r: 0xDA, g: 0x70, b: 0xD6 };
pub const MAGENTA:                  Color = Color { r: 0xFF, g: 0x00, b: 0xFF };
pub const MEDIUM_ORCHID:            Color = Color { r: 0xBA, g: 0x55, b: 0xD3 };
pub const MEDIUM_PURPLE:            Color = Color { r: 0x93, g: 0x70, b: 0xDB };
pub const BLUE_VIOLET:              Color = Color { r: 0x8A, g: 0x2B, b: 0xE2 };
pub const DARK_VIOLET:              Color = Color { r: 0x94, g: 0x00, b: 0xD3 };
pub const DARK_ORCHID:              Color = Color { r: 0x99, g: 0x32, b: 0xCC };
pub const DARK_MAGENTA:             Color = Color { r: 0x8B, g: 0x00, b: 0x8B };
pub const INDIGO:                   Color = Color { r: 0x4B, g: 0x00, b: 0x82 };
pub const DARK_SLATE_BLUE:          Color = Color { r: 0x4B, g: 0x3D, b: 0x8B };
pub const SLATE_BLUE:               Color = Color { r: 0x6A, g: 0x5A, b: 0xCD };
pub const MEDIUM_SLATE_BLUE:        Color = Color { r: 0x7B, g: 0x68, b: 0xEE };

pub const SNOW:                     Color = Color { r: 0xFF, g: 0xFA, b: 0xFA };
pub const HONEYDEW:                 Color = Color { r: 0xF0, g: 0xFF, b: 0xF0 };
pub const MINT_CREAM:               Color = Color { r: 0xF5, g: 0xFF, b: 0xFA };
pub const AZURE:                    Color = Color { r: 0xF0, g: 0xFF, b: 0xFF };
pub const ALICE_BLUE:               Color = Color { r: 0xF0, g: 0xF8, b: 0xFF };
pub const GHOST_WHITE:              Color = Color { r: 0xF8, g: 0xF8, b: 0xFF };
pub const WHITE_SMOKE:              Color = Color { r: 0xF5, g: 0xF5, b: 0xF5 };
pub const SEASHELL:                 Color = Color { r: 0xFF, g: 0xF5, b: 0xEE };
pub const BEIGE:                    Color = Color { r: 0xF5, g: 0xF5, b: 0xDC };
pub const OLD_LACE:                 Color = Color { r: 0xFD, g: 0xF5, b: 0xE6 };
pub const FLORAL_WHITE:             Color = Color { r: 0xFF, g: 0xFA, b: 0xF0 };
pub const IVORY:                    Color = Color { r: 0xFF, g: 0xFF, b: 0xF0 };
pub const ANTINQUE_WHITE:           Color = Color { r: 0xFA, g: 0xEB, b: 0xD7 };
pub const LINEN:                    Color = Color { r: 0xFA, g: 0xF0, b: 0xE6 };
pub const LAVENDER_BLUSH:           Color = Color { r: 0xFF, g: 0xF0, b: 0xF5 };
pub const MISTY_ROSE:               Color = Color { r: 0xFF, g: 0xE4, b: 0xE1 };

pub const GAINSBORO:                Color = Color { r: 0xDC, g: 0xDC, b: 0xDC };
pub const LIGHT_GRAY:               Color = Color { r: 0xD3, g: 0xD3, b: 0xD3 };
pub const DARK_GRAY:                Color = Color { r: 0xA9, g: 0xA9, b: 0xA9 };
pub const DIM_GRAY:                 Color = Color { r: 0x69, g: 0x69, b: 0x69 };
pub const LIGHT_SLATE_GRAY:         Color = Color { r: 0x77, g: 0x88, b: 0x99 };
pub const SLATE_GRAY:               Color = Color { r: 0x70, g: 0x80, b: 0x90 };
pub const DARK_SLATE_GRAY:          Color = Color { r: 0x2F, g: 0x4F, b: 0x4F };
