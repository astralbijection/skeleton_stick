pub mod multithread_util;

use std::{
    borrow::BorrowMut,
    collections::HashMap,
    process::abort,
    sync::{Arc, RwLock},
    thread,
};

use deadqueue::unlimited;
use embedded_graphics::prelude::Size;
use embedded_graphics_simulator::{
    sdl2::Keycode, BinaryColorTheme, OutputSettingsBuilder, SimulatorDisplay, Window,
};

use crate::ui::{
    app::AppCtx,
    event::{Event, Key},
};

use self::multithread_util::MultithreadSimDisplay;

#[tokio::main]
pub async fn run() {
    ctrlc::set_handler(move || {
        eprintln!("received interrupt, exiting");
        abort();
    })
    .expect("Error setting keyboard interrupt handler");

    let display = MultithreadSimDisplay::new(Arc::new(RwLock::new(SimulatorDisplay::new(
        Size::new(128, 64),
    ))));

    let mut app = AppCtx::create_with_display(display);

    {
        let raw_events = app.raw_events.clone();
        let display = app.drawable.clone();
        thread::spawn(move || {
            window_thread(raw_events, display);
        });
    }

    app.borrow_mut().run().await;
}

fn window_thread(event_queue: Arc<unlimited::Queue<Event>>, display: MultithreadSimDisplay) {
    let keymap: HashMap<Keycode, Key> = HashMap::from([
        (Keycode::Up, Key::Up),
        (Keycode::Down, Key::Down),
        (Keycode::Left, Key::Left),
        (Keycode::Right, Key::Right),
        (Keycode::Backspace, Key::Back),
        (Keycode::Return, Key::Enter),
        (Keycode::Space, Key::Shift),
    ]);

    let output_settings = OutputSettingsBuilder::new()
        .theme(BinaryColorTheme::OledBlue)
        .build();

    let mut window = Window::new("Skeleton Stick Simulator", &output_settings);

    eprintln!("Keymap: {:#?}", &keymap);

    loop {
        {
            let read = display.arc.read().unwrap();
            window.update(&read);
        }

        for simev in window.events() {
            match simev {
                embedded_graphics_simulator::SimulatorEvent::KeyUp {
                    keycode,
                    repeat: false,
                    ..
                } => {
                    if let Some(k) = keymap.get(&keycode) {
                        event_queue.push(Event::KeyUp(*k))
                    }
                }
                embedded_graphics_simulator::SimulatorEvent::KeyDown {
                    keycode,
                    repeat: false,
                    ..
                } => {
                    if let Some(k) = keymap.get(&keycode) {
                        event_queue.push(Event::KeyDown(*k))
                    }
                }
                embedded_graphics_simulator::SimulatorEvent::Quit => break,
                _ => {} // no-op
            }
        }
    }
}
