use crate::objects::Color;
use crate::structure::ImageContext;
use cairo::Context;

pub trait Rendable {
    fn render(&self, context: &Context, image_context: &ImageContext);

    fn configure_color(&self, color: &Color, context: &Context, image_context: &ImageContext) {
        match &color {
            Color::Fill => {
                let palette = image_context.palette();
                context.set_source_rgb(
                    f64::from(palette.fill_color.red),
                    f64::from(palette.fill_color.green),
                    f64::from(palette.fill_color.blue),
                );
            }
            Color::Background => {
                let palette = image_context.palette();
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
