//! palette are currently randomly generated.

use palette::rgb::Rgb;
use palette::FromColor;
use palette::Hsl;
use palette::Hsv;
use palette::IntoColor;
use rand::prelude::*;

/// The palette that is used
/// to generate the image.
pub struct Palette {
    pub background_color: Rgb,
    pub fill_color: Rgb,
}

impl Palette {
    /// generate a random color that is not to dark
    /// and to color less.
    pub fn random_color() -> Rgb {
        let mut rng = rand::thread_rng();
        let y: f32 = rng.gen(); // generates a float between 0 and 1
        let hue: f32 = y * 360.0;
        let saturation: f32 = f32::max(rng.gen(), 0.6);
        let value: f32 = f32::max(rng.gen(), 0.6);
        Rgb::from_linear(Hsv::new(hue, saturation, value).into_rgb())
    }

    /// generate a palette from the tint and shade palette
    /// algorithm. choose a dark background and a bright filling color
    pub fn bright_on_dark(input: Rgb) -> Palette {
        let tint_and_shade_palette = TintAndShadePalette::create(input);
        Palette {
            background_color: tint_and_shade_palette.base_shade_30,
            fill_color: tint_and_shade_palette.inverse_saturation_tint_30,
        }
    }

    /// generate a palette from the tint and shade palette
    /// algorithm. choose a bright background and a dark filling color
    pub fn dark_on_bright(input: Rgb) -> Palette {
        let tint_and_shade_palette = TintAndShadePalette::create(input);
        Palette {
            background_color: tint_and_shade_palette.base_tint_30,
            fill_color: tint_and_shade_palette.inverse_saturation_shade_30,
        }
    }
}

/// The Tint and Shade Palette from https://gitlab.com/cameralibre/tint-and-shade
/// Thank you Sam
#[allow(dead_code)]
struct TintAndShadePalette {
    base_color: Rgb,
    base_tint_30: Rgb,
    base_tint_15: Rgb,
    base_shade_15: Rgb,
    base_shade_30: Rgb,
    complement_tint_30: Rgb,
    complement_tint_15: Rgb,
    complement_color: Rgb,
    complement_shade_15: Rgb,
    complement_shade_30: Rgb,
    inverse_saturation_tint_30: Rgb,
    inverse_saturation_tint_15: Rgb,
    inverse_saturation_color: Rgb,
    inverse_saturation_shade_15: Rgb,
    inverse_saturation_shade_30: Rgb,
}

impl TintAndShadePalette {
    /// create a Palette based on one input color
    pub fn create(input: Rgb) -> TintAndShadePalette {
        let hsl: Hsl = Hsl::from_rgb(input.into_linear());
        let h = hsl.hue;
        let s = hsl.saturation;
        let l = hsl.lightness;
        let complement = h + 180.0 % 360.0;
        let tint_15 = l + 0.15;
        let tint_30 = l + 0.30;
        let shade_15 = l - 0.15;
        let shade_30 = l - 0.30;
        let inverse_saturation = 1.0 - s;

        TintAndShadePalette {
            base_color: Rgb::from_linear(Hsl::new(h, s, l).into_rgb()),
            base_tint_15: Rgb::from_linear(Hsl::new(h, s, tint_15).into_rgb()),
            base_tint_30: Rgb::from_linear(Hsl::new(h, s, tint_30).into_rgb()),
            base_shade_15: Rgb::from_linear(Hsl::new(h, s, shade_15).into_rgb()),
            base_shade_30: Rgb::from_linear(Hsl::new(h, s, shade_30).into_rgb()),
            complement_color: Rgb::from_linear(Hsl::new(complement, s, l).into_rgb()),
            complement_tint_15: Rgb::from_linear(Hsl::new(complement, s, tint_15).into_rgb()),
            complement_tint_30: Rgb::from_linear(Hsl::new(complement, s, tint_30).into_rgb()),
            complement_shade_15: Rgb::from_linear(Hsl::new(complement, s, shade_15).into_rgb()),
            complement_shade_30: Rgb::from_linear(Hsl::new(complement, s, shade_30).into_rgb()),
            inverse_saturation_color: Rgb::from_linear(
                Hsl::new(complement, inverse_saturation, l).into_rgb(),
            ),
            inverse_saturation_tint_15: Rgb::from_linear(
                Hsl::new(complement, inverse_saturation, tint_15).into_rgb(),
            ),
            inverse_saturation_tint_30: Rgb::from_linear(
                Hsl::new(complement, inverse_saturation, tint_30).into_rgb(),
            ),
            inverse_saturation_shade_15: Rgb::from_linear(
                Hsl::new(complement, inverse_saturation, shade_15).into_rgb(),
            ),
            inverse_saturation_shade_30: Rgb::from_linear(
                Hsl::new(complement, inverse_saturation, shade_30).into_rgb(),
            ),
        }
    }
}
