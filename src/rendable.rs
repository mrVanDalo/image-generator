use crate::objects::Color;
use crate::structure::Querable;
use cairo::Context;

pub trait Rendable {
    fn render(&self, context: &Context, querable: &dyn Querable);

    fn configure_color(&self, color: &Color, context: &Context, querable: &dyn Querable) {
        match &color {
            Color::Fill => {
                let palette = querable.palette();
                context.set_source_rgb(
                    f64::from(palette.fill_color.red),
                    f64::from(palette.fill_color.green),
                    f64::from(palette.fill_color.blue),
                );
            }
            Color::Background => {
                let palette = querable.palette();
                context.set_source_rgb(
                    f64::from(palette.background_color.red),
                    f64::from(palette.background_color.green),
                    f64::from(palette.background_color.blue),
                );
            }
        }
    }

    #[inline(always)]
    fn stroke_and_preserve_line_width(&self, context: &Context) {
        // todo: just add this function to context
        context.save();
        context.identity_matrix();
        context.stroke();
        context.restore();
    }
}
