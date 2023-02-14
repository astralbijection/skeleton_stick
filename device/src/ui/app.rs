use std::sync::Arc;

use deadqueue::unlimited;
use embedded_graphics::prelude::DrawTarget;

use super::{
    event::{Event, Key},
    pwscreen,
};

#[derive(Debug)]
pub struct AppCtx<D: DrawTarget> {
    pub drawable: D,
    pub raw_events: Arc<unlimited::Queue<Event>>,
    shift: bool,
}

impl<D> AppCtx<D>
where
    D: DrawTarget,
{
    pub fn create_with_display(drawable: D) -> Self {
        let raw_events = Arc::new(unlimited::Queue::new());

        Self {
            drawable,
            raw_events,
            shift: false,
        }
    }

    pub async fn run(&mut self) {
        pwscreen::run(self).await;
    }

    pub async fn fetch_event(&mut self) -> (Event, bool) {
        let ev = self.raw_events.pop().await;
        match &ev {
            Event::KeyDown(Key::Shift) => {
                self.shift = true;
            }
            Event::KeyUp(Key::Shift) => {
                self.shift = false;
            }
            _ => {}
        }
        (ev, self.shift)
    }
}
