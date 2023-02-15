use std::iter;

use embedded_graphics::mono_font::ascii::FONT_6X9;
use embedded_graphics::mono_font::iso_8859_14::FONT_4X6;
use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::prelude::Dimensions;
use embedded_graphics::prelude::DrawTarget;
use embedded_graphics::prelude::Point;
use embedded_graphics::primitives::Rectangle;
use embedded_graphics::text::Text;
use embedded_graphics::Drawable;
use skeleton_stick_lib::password_file::PasswordFile;

use super::{
    app::AppCtx,
    event::{Event, Key},
};

#[derive(Debug, PartialEq, Eq, Clone)]
enum Action {
    AppendChar(u8),
    Backspace,
    Clear,
    Submit,
    NoOp,
}

impl From<(Event, bool)> for Action {
    fn from(value: (Event, bool)) -> Self {
        let (ev, shift) = value;

        let append_char = |c: u8| -> Action {
            if shift {
                Action::AppendChar(c.to_ascii_uppercase())
            } else {
                Action::AppendChar(c)
            }
        };

        match ev {
            Event::KeyDown(k) => match k {
                Key::Up => append_char(b'u'),
                Key::Down => append_char(b'd'),
                Key::Left => append_char(b'l'),
                Key::Right => append_char(b'r'),
                Key::Center => append_char(b'c'),

                Key::Enter => Action::Submit,
                Key::Back => {
                    if shift {
                        Action::Clear
                    } else {
                        Action::Backspace
                    }
                }
                _ => Action::NoOp,
            },
            _ => Action::NoOp,
        }
    }
}

pub struct PasswordScreen<'a, D>
where
    D: DrawTarget<Color = BinaryColor>,
{
    ctx: &'a mut AppCtx<D>,
    password: Vec<u8>,
    last_message_box: Rectangle,
}

impl<'a, D> PasswordScreen<'a, D>
where
    D: DrawTarget<Color = BinaryColor>,
{
    pub fn new(ctx: &'a mut AppCtx<D>) -> Self {
        Self {
            ctx,
            password: Vec::new(),
            last_message_box: Rectangle::zero(),
        }
    }

    pub async fn run(mut self) -> PasswordFile
    where
        D: DrawTarget<Color = BinaryColor>,
    {
        Text::new(
            "Enter your password",
            Point::new(5, 5),
            MonoTextStyle::new(&FONT_6X9, BinaryColor::On),
        )
        .draw(&mut self.ctx.drawable);

        loop {
            self.ask_pw().await;

            self.write_message("Unlocking...");

            if let Some(pwf) = self.unlock().await {
                return pwf;
            }

            self.write_message("Wrong password!");
        }
    }

    async fn ask_pw(&mut self) {
        let mut text_bb = Rectangle::zero();

        loop {
            let stars: String = iter::repeat('*').take(self.password.len()).collect();
            let star_text = Text::new(
                &stars,
                Point::new(5, 30),
                MonoTextStyle::new(&FONT_4X6, BinaryColor::On),
            );
            self.ctx.drawable.fill_solid(&text_bb, BinaryColor::Off);
            star_text.draw(&mut self.ctx.drawable);
            text_bb = star_text.bounding_box();

            let action: Action = self.ctx.fetch_event().await.into();
            eprintln!("{action:?}");
            match action {
                Action::AppendChar(c) => {
                    self.password.push(c);
                }
                Action::Backspace => {
                    self.password.pop();
                }
                Action::Clear => {
                    self.password.clear();
                }
                Action::Submit => break,
                Action::NoOp => {}
            }
        }
    }

    fn write_message(&mut self, message: &str) {
        self.ctx
            .drawable
            .fill_solid(&self.last_message_box, BinaryColor::Off);
        let text = Text::new(
            message,
            Point::new(6, 20),
            MonoTextStyle::new(&FONT_6X9, BinaryColor::On),
        );
        text.draw(&mut self.ctx.drawable);
        self.last_message_box = text.bounding_box();
    }

    async fn unlock(&mut self) -> Option<PasswordFile> {
        None // TODO
    }
}
