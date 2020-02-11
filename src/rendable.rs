use cairo::Context;

pub trait Rendable {
    fn render(&self, context: &Context);
}
