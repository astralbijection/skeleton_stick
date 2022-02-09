"""
Utilities for converting from characters to HID reports.
"""

from io import BytesIO
from pathlib import Path
from time import sleep
from typing import Optional

RELEASE = b'\0' * 8
"""HID report to release keys."""

_UNSHIFT = (
    "abcdefghijklmnopqrstuvwxyz1234567890"
    " -=[]\\"
    ";'`,./"
)
_SHIFT = (
    "ABCDEFGHIJKLMNOPQRSTUVWXYZ!@#$%^&*()"
    " _+{}|"
    ':"~<>?'
)
_ids = (
    list(range(0x04, 0x28)) +
    list(range(0x2c, 0x32)) +
    list(range(0x33, 0x39))
)
_char_to_report = {
    **{
        # b0 - modifiers, left-shift = 0x02
        # b1 - ???
        # b2 - keycode
        # b3:7 - padding to 8 bytes
        c: b'\2\0' + bytes([i]) + (b'\0' * 5) for c, i in zip(_SHIFT, _ids)
    },
    **{
        # b0 - modifiers, none = 0x00
        # b1 - ???
        # b2 - keycode
        # b3:7 - padding to 8 bytes
        c: b'\0\0' + bytes([i]) + (b'\0' * 5) for c, i in zip(_UNSHIFT, _ids)
    },
}

REPORT_DESC = (
    b'\x05\x01\x09\x06\xa1\x01\x05\x07\x19\xe0\x29\xe7\x15\x00\x25\x01\x75\x01'
    b'\x95\x08\x81\x02\x95\x01\x75\x08\x81\x03\x95\x05\x75\x01\x05\x08\x19\x01'
    b'\x29\x05\x91\x02\x95\x01\x75\x03\x91\x03\x95\x06\x75\x08\x15\x00\x25\x65'
    b'\x05\x07\x19\x00\x29\x65\x81\x00\xc0'
)
"""
An weird report that I'll figure out later.
It does some HID magic to initialize the keyboard it seems.
"""


def get_udc() -> Optional[str]:
    """Returns the UDC value."""
    try:
        udc_path = next(Path('/sys/class/udc').iterdir())
    except FileNotFoundError:
        return None
    except StopIteration:
        return None
    return udc_path.name


def setup_hid(path: Path, udc: str, ignore_busy: bool = True):
    """
    Sets up the HID device at the specified path under
    /sys/kernel/config/usb_gadget.

    See https://randomnerdtutorials.com/raspberry-pi-zero-usb-keyboard-hid/
    for what I ripped off.
    """
    path.mkdir(parents=True, exist_ok=True)
    (path / "idVendor").write_text('0x1d6b')  # Linux Foundation
    (path / "idProduct").write_text('0x0104')  # Multifunction Composite Gadget
    (path / "bcdDevice").write_text('0x0100')  # v1.0.0
    (path / "bcdUSB").write_text('0x0200')  # USB2

    strings = path / "strings" / "0x409"
    strings.mkdir(parents=True, exist_ok=True)
    (strings / "serialnumber").write_text("cafebabe")
    (strings / "manufacturer").write_text("Astrid Yu")
    (strings / "product").write_text("Skeleton Stick")

    configs = path / "configs" / "c.1"
    configs_subdir = configs / "strings" / "0x409"
    configs_subdir.mkdir(parents=True, exist_ok=True)
    (configs_subdir / "configuration").write_text("Config 1: ECM network")
    (configs / "MaxPower").write_text("250")

    functions = path / "functions" / "hid.usb0"
    functions.mkdir(parents=True, exist_ok=True)
    (functions / "protocol").write_text("1")
    (functions / "subclass").write_text("1")
    (functions / "report_length").write_text("8")
    (functions / "report_desc").write_bytes(REPORT_DESC)

    (configs / "hid.usb0").symlink_to(functions)

    try:
        (path / "UDC").write_text(udc)
    except OSError as err:
        if not (ignore_busy and err.errno == 16):
            raise


def write_keyboard(file: BytesIO, text: str, delay: float = 0.01) -> None:
    """Given a file, writes the given text to the file as keyboard reports."""
    for char in text:
        file.write(char_to_report(char))
        sleep(delay)
        file.write(RELEASE)
        sleep(delay)


def char_to_report(char: str) -> bytes:
    """
    Given a character, returns its HID report.
    """
    return _char_to_report[char]
