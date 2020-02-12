use crate::structure::Querable;
use cairo::Context;

pub trait Rendable {
    fn render(&self, context: &Context, querable: &dyn Querable);
}
