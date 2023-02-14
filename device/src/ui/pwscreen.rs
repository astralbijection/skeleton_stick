use embedded_graphics::prelude::DrawTarget;
use embedded_graphics::prelude::DrawTargetExt;
use secstr::{SecStr, SecVec};
use skeleton_stick_lib::password_file::EncryptedPasswordFile;
use skeleton_stick_lib::password_file::PasswordFile;
use tokio::select;

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

pub async fn run(ctx: &mut AppCtx<impl DrawTarget>) -> PasswordFile {
    let mut password: SecStr = "".into();
    let mut message: String = "".into();

    loop {
        let action: Action = ctx.fetch_event().await.into();
        match action {
            Action::AppendChar(c) => {
                password.resize(password.unsecure().len() + 1, c);
            }
            Action::Backspace => {
                password.resize(password.unsecure().len() - 1, 0);
            }
            Action::Clear => password.zero_out(),
            Action::Submit => break,
            Action::NoOp => {}
        }
    }
}
