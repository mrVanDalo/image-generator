use palette::rgb::Rgb;
use palette::FromColor;
use palette::Hsl;
use palette::Hsv;
use palette::IntoColor;
use rand::prelude::*;


pub struct Palette {
    pub background_color: ColorType,
    pub fill_color: ColorType,
}

impl Default for Palette {
    fn default() -> Self {
        Palette::dark_on_bright(Palette::random_color())
    }
}

impl Palette {
    pub fn random_color() -> Rgb {
        let mut rng = rand::thread_rng();
        let y: f32 = rng.gen(); // generates a float between 0 and 1
        let hue: f32 = y * 360.0;
        let saturation: f32 = f32::max(rng.gen(), 0.6);
        let value: f32 = f32::max(rng.gen(), 0.6);
        Rgb::from_linear(Hsv::new(hue, saturation, value).into_rgb())
    }
    #[allow(dead_code)]
    pub fn bright_on_dark(input: Rgb) -> Palette {
        let tint_and_shade_palette = TintAndShadePalette::create(input);
        Palette {
            background_color: tint_and_shade_palette.base_shade_30,
            fill_color: tint_and_shade_palette.inverse_saturation_tint_30,
        }
    }
    #[allow(dead_code)]
    pub fn dark_on_bright(input: Rgb) -> Palette {
        let tint_and_shade_palette = TintAndShadePalette::create(input);
        Palette {
            background_color: tint_and_shade_palette.base_tint_30,
            fill_color: tint_and_shade_palette.inverse_saturation_shade_30,
        }
    }
}

type ColorType = Rgb;

#[allow(dead_code)]
struct TintAndShadePalette {
    base_color: ColorType,
    base_tint_30: ColorType,
    base_tint_15: ColorType,
    base_shade_15: ColorType,
    base_shade_30: ColorType,
    complement_tint_30: ColorType,
    complement_tint_15: ColorType,
    complement_color: ColorType,
    complement_shade_15: ColorType,
    complement_shade_30: ColorType,
    inverse_saturation_tint_30: ColorType,
    inverse_saturation_tint_15: ColorType,
    inverse_saturation_color: ColorType,
    inverse_saturation_shade_15: ColorType,
    inverse_saturation_shade_30: ColorType,
}

impl TintAndShadePalette {
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
