use std::io::Read;
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
use secstr::SecBox;
use secstr::SecStr;
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

        fn append_char(c: u8, shift: bool) -> Action {
            if shift {
                Action::AppendChar(c.to_ascii_uppercase())
            } else {
                Action::AppendChar(c)
            }
        }

        match ev {
            Event::KeyDown(k) => match k {
                Key::Up => append_char(b'u', shift),
                Key::Down => append_char(b'd', shift),
                Key::Left => append_char(b'l', shift),
                Key::Right => append_char(b'r', shift),
                Key::Center => append_char(b'c', shift),

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
    password: SecStr,
    last_message_bb: Rectangle,
}

impl<'a, D> PasswordScreen<'a, D>
where
    D: DrawTarget<Color = BinaryColor>,
{
    pub fn new(ctx: &'a mut AppCtx<D>) -> Self {
        Self {
            ctx,
            password: "".into(),
            last_message_bb: Rectangle::zero(),
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
        let mut text_bb = SecBox::new(Box::new(Rectangle::zero()));

        loop {
            let stars: String = iter::repeat('*')
                .take(self.password.unsecure().len())
                .collect();
            let star_text = Text::new(
                &stars,
                Point::new(5, 30),
                MonoTextStyle::new(&FONT_4X6, BinaryColor::On),
            );
            self.ctx.drawable.fill_solid(&text_bb.unsecure(), BinaryColor::Off);
            star_text.draw(&mut self.ctx.drawable);
            *text_bb.unsecure_mut() = star_text.bounding_box();

            let action: Action = self.ctx.fetch_event().await.into();
            match action {
                Action::AppendChar(c) => {
                    self.password.resize(self.password.unsecure().len() + 1, c);
                }
                Action::Backspace => {
                    self.password.resize(self.password.unsecure().len() - 1, 0);
                }
                Action::Clear => {
                    self.password.resize(0, 0);
                }
                Action::Submit => break,
                Action::NoOp => {}
            }
            // eprintln!("{action:?} {}", String::from_utf8_lossy(&self.password.unsecure()));
        }
    }

    fn write_message(&mut self, message: &str) {
        self.ctx
            .drawable
            .fill_solid(&self.last_message_bb, BinaryColor::Off);
        let text = Text::new(
            message,
            Point::new(6, 20),
            MonoTextStyle::new(&FONT_6X9, BinaryColor::On),
        );
        text.draw(&mut self.ctx.drawable);
        self.last_message_bb = text.bounding_box();
    }

    async fn unlock(&mut self) -> Option<PasswordFile> {
        None // TODO
    }
}
