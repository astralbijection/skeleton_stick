use std::path::Path;
use std::time::Duration;

use lazy_static::lazy_static;

use itertools::{izip, Itertools};
use tokio::io::{AsyncWrite, AsyncWriteExt};
use tokio::time::sleep;

/// HID report to release keys.
pub const RELEASE: [u8; 8] = [0; 8];

/// An weird report that I'll figure out later.
/// It does some HID magic to initialize the keyboard it seems.
pub const REPORT_DESC: &[u8] = b"\x05\x01\x09\x06\xa1\x01\x05\x07\x19\xe0\x29\xe7\x15\x00\x25\x01\x75\x01\x95\x08\x81\x02\x95\x01\x75\x08\x81\x03\x95\x05\x75\x01\x05\x08\x19\x01\x29\x05\x91\x02\x95\x01\x75\x03\x91\x03\x95\x06\x75\x08\x15\x00\x25\x65\x05\x07\x19\x00\x29\x65\x81\x00\xc0";

/// Supported key IDs.
const KEY_IDS: [u8; 48] = [
    4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28,
    29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 44, 45, 46, 47, 48, 49, 51, 52, 53, 54, 55, 56,
];

const UNSHIFT: &[u8] = b"abcdefghijklmnopqrstuvwxyz1234567890 -=[]\\;'`,./";
const SHIFT: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ!@#$%^&*() _+{}|:\"~<>?";

lazy_static! {
    static ref CHAR_IS_SUPPORTED: [bool ; 128] = {
        let mut map = [false; 128];
        for c in UNSHIFT.into_iter().chain(SHIFT.into_iter()) {
            map[*c as usize] = true;
        }
        map
    };

    static ref CHAR_TO_REPORT: [[u8; 8] ; 128] = {
        let mut map = [[0u8; 8]; 128];
        for (i, u, s) in izip!(KEY_IDS.into_iter(), UNSHIFT.into_iter(), SHIFT.into_iter()) {
            // b0 - modifiers, left-shift = 0x02
            // b1 - ???
            // b2 - keycode
            // b3:7 - padding to 8 bytes
            map[*s as usize] = [2, 0, i, 0, 0, 0, 0, 0];

            // b0 - modifiers, none = 0x00
            // b1 - ???
            // b2 - keycode
            // b3:7 - padding to 8 bytes
            map[*u as usize] = [2, 0, i, 0, 0, 0, 0, 0];
        }
        map
    };
}

/// Converts a given character to the HID report that would send that character
/// on a standard US keyboard.
pub fn char_to_report(c: char) -> Option<[u8; 8]> {
    match CHAR_IS_SUPPORTED.get(c as usize) {
        Some(true) => Some(CHAR_TO_REPORT[c as usize]),
        _ => None,
    }
}

/// Sets up the HID device at the specified path under
/// /sys/kernel/config/usb_gadget.
/// See https://randomnerdtutorials.com/raspberry-pi-zero-usb-keyboard-hid/
/// for what I ripped off.
pub async fn setup_hid(path: impl AsRef<Path>, udc: &str) -> Result<(), tokio::io::Error> {
    use tokio::fs::*;

    let path = path.as_ref();

    write(path.join("idVendor"), "0x1d6b").await?; // Linux Foundation

    write(path.join("idProduct"), "0x0104").await?; // Multifunction Composite Gadget
    write(path.join("bcdDevice"), "0x0200").await?; // v2.0.0
    write(path.join("bcdUSB"), "0x0200").await?; // USB2

    let strings = path.join("strings/0x409");
    create_dir_all(&strings).await?;
    write(strings.join("serialnumber"), "cafebabe").await?;
    write(strings.join("manufacturer"), "IFD3F Technologies").await?;
    write(strings.join("product"), "Skeleton Stick").await?;

    let configs = path.join("configs/c.1");
    let configs_subdir = configs.join("strings/0x409");
    create_dir_all(&configs_subdir).await?;
    write(
        configs_subdir.join("configuration"),
        "Config 1: ECM network",
    )
    .await?;
    write(configs.join("MaxPower"), "250").await?;

    let functions = path.join("functions/hid.usb0");
    create_dir_all(&functions).await?;
    write(functions.join("protocol"), "1").await?;
    write(functions.join("subclass"), "1").await?;
    write(functions.join("report_length"), "8").await?;
    write(functions.join("report_desc"), REPORT_DESC).await?;

    symlink(functions, configs.join("hid.usb0")).await?;

    write(path.join("UDC"), udc).await?;
    Ok(())
}

#[derive(Debug)]
pub enum KeyboardError {
    UnsupportedChar(char),
    IOError(tokio::io::Error),
}

impl From<tokio::io::Error> for KeyboardError {
    fn from(value: tokio::io::Error) -> Self {
        Self::IOError(value)
    }
}

pub async fn write_keyboard(
    mut kb: impl AsyncWrite + std::marker::Unpin,
    text: &str,
    delay: Duration,
) -> Result<(), KeyboardError> {
    let (reports, errs): (Vec<_>, Vec<_>) = text
        .chars()
        .map(|c| char_to_report(c).ok_or(c))
        .partition_result();

    // Raise error if we encountered bad characters
    if let Some(c) = errs.first() {
        return Err(KeyboardError::UnsupportedChar(*c));
    }

    for r in reports {
        kb.write_all(&r).await?;
        sleep(delay).await;
        kb.write_all(&RELEASE).await?;
        sleep(delay).await;
    }

    Ok(())
}
