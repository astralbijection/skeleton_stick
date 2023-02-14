use embedded_graphics::{pixelcolor::BinaryColor, prelude::Size};
use embedded_graphics_simulator::{
    BinaryColorTheme, OutputSettingsBuilder, SimulatorDisplay, Window,
};

use crate::ui::App;

pub fn run() {
    let display = SimulatorDisplay::<BinaryColor>::new(Size::new(128, 64));
    let app = App::create_with_display(display);

    let output_settings = OutputSettingsBuilder::new()
        .theme(BinaryColorTheme::OledBlue)
        .build();
    let mut window = Window::new("Skeleton Stick Simulator", &output_settings);

    while true {
        window.update(&app.draw_target());
        for simev in window.events() {
            match simev {
                embedded_graphics_simulator::SimulatorEvent::KeyUp {
                    keycode,
                    keymod,
                    repeat,
                } => todo!(),
                embedded_graphics_simulator::SimulatorEvent::KeyDown {
                    keycode,
                    keymod,
                    repeat,
                } => todo!(),
                embedded_graphics_simulator::SimulatorEvent::Quit => todo!(),
                _ => {} // no-op
            }
        }
    }
}
