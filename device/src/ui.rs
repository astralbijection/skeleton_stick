use embedded_graphics::prelude::DrawTarget;

pub struct App<D: DrawTarget> {
    drawable: D,
}

impl<D> App<D>
where
    D: DrawTarget,
{
    pub fn create_with_display(drawable: D) -> Self {
        Self { drawable }
    }

    pub fn handle_event(&self, event: Event) {
        todo!()
    }

    pub fn draw_target(&self) -> &D {
        &self.drawable
    }
}

pub enum Event {
    KeyDown(Key),
    KeyUp(Key),
}

pub enum Key {
    Up,
    Down,
    Left,
    Right,
    Center,

    Back,
    Enter,
    Shift,
}
