"""
An OLED interface for the pi.

https://www.waveshare.com/1.3inch-oled-hat.htm
"""

from xmlrpc.client import Boolean
from spidev import SpiDev
import os
from enum import IntEnum
from pathlib import Path
from time import sleep
from typing import Callable, Dict, List, Optional, TypeVar

from gpiozero import Button, OutputDevice
from luma.core.interface.serial import spi
from luma.core.render import canvas
from luma.oled.device import sh1106


from skeleton_stick.hid import get_udc, setup_hid, write_keyboard
from skeleton_stick.storage import PasswordEntry, make_loader


RST_PIN = 25
DC_PIN = 24


class Key(IntEnum):
    """
    A key on the HAT.
    """
    UP = 6
    DOWN = 19
    LEFT = 5
    RIGHT = 26
    CENTER = 13
    K1 = 21
    K2 = 20
    K3 = 16


class HATButtons:
    """Wrapper for the buttons on a HAT."""

    def __init__(self):
        self.buttons: Dict[Key, Button] = {
            k: Button(k, pull_up=True)
            for k in Key
        }

    def __enter__(self):
        return self

    def __exit__(self, _, value, traceback):
        self.close()

    def detach_listeners(self) -> None:
        """
        Detaches all event listeners on the buttons.
        """
        for _, btn in self.buttons.items():
            btn.when_activated = None
            btn.when_deactivated = None
            btn.when_held = None

    def close(self) -> None:
        """
        Closes all buttons.
        """
        for _, btn in self.buttons.items():
            btn.close()

    def when_activated(self, key: Key) -> Callable[[Callable[[], None]], Callable[[], None]]:
        """
        A decorator for attaching event handlers.
        """
        def decorator(handler: Callable[[], None]) -> Callable[[], None]:
            self.buttons[key].when_activated = handler
            return handler
        return decorator


def start_oled(password_file: Path):
    """Start the OLED interface."""

    device_path = Path(os.environ.get("HID_GADGET", "/dev/hidg0"))

    def use_password(entry: PasswordEntry) -> None:
        udc = get_udc()
        print("UDC:", udc)
        setup_hid(
            Path('/sys/kernel/config/usb_gadget/skeleton_stick'),
            udc=udc
        )
        with device_path.open('wb') as file:
            write_keyboard(file, entry.password)

    serial = spi(device=0, port=0, bus_speed_hz=8000000,
                 transfer_size=4096, gpio_DC=DC_PIN, gpio_RST=RST_PIN)
    device = sh1106(serial, rotate=2)  # sh1106

    with HATButtons() as btns:
        while True:
            btns.detach_listeners()
            entries = password_prompt(
                btns, device, verify=make_loader(password_file))

            if not entries:
                continue

            btns.detach_listeners()
            password_browser(btns, device, entries, use_password=use_password)


R = TypeVar("R")


def password_prompt(btns: HATButtons, device: sh1106, verify: Callable[[str], Optional[R]]) -> Optional[R]:
    """
    Gets a password from the user.
    """

    buf: List[str] = []
    status: Optional[str] = None
    result: Optional[R] = None

    @btns.when_activated(Key.LEFT)
    def on_left():
        buf.append('L')
        render()

    @btns.when_activated(Key.RIGHT)
    def on_right():
        buf.append('R')
        render()

    @btns.when_activated(Key.UP)
    def on_up():
        buf.append('U')
        render()

    @btns.when_activated(Key.DOWN)
    def on_down():
        buf.append('D')
        render()

    @btns.when_activated(Key.CENTER)
    def on_center():
        buf.append('C')
        render()

    @btns.when_activated(Key.K1)
    def on_backspace():
        if len(buf) > 0:
            buf.pop()  # Backspace
            render()

    @btns.when_activated(Key.K2)
    def on_extra():
        nonlocal buf
        buf = []  # Clear
        render()

    @btns.when_activated(Key.K3)
    def on_verify():
        nonlocal result, status
        status = "Verifying..."
        render()
        result = verify(''.join(buf))
        status = 'Success!' if result else 'Wrong password'

    def render():
        with canvas(device) as draw:
            draw.text((0, 0), 'Decrypt', fill='white')
            draw.text((0, 10), "*" * len(buf), fill='white')
            if status:
                draw.text((0, 30), status, fill='white')

    render()
    while not result:
        sleep(0.5)

    return result


def password_browser(btns: HATButtons, device: sh1106, entries: List[PasswordEntry], use_password: Callable[[PasswordEntry], None]):
    """Browse passwords and use them."""

    selected_index: int = 0
    scroll_offset: int = 0
    sending: bool = False
    error: bool = False
    done: bool = False

    @btns.when_activated(Key.UP)
    def on_up():
        nonlocal error, selected_index
        selected_index = (selected_index - 1) % len(entries)
        error = False
        render()

    @btns.when_activated(Key.DOWN)
    def on_down():
        nonlocal error, selected_index
        selected_index = (selected_index + 1) % len(entries)
        error = False
        render()

    @btns.when_activated(Key.K3)
    @btns.when_activated(Key.CENTER)
    def on_center():
        nonlocal error, sending
        sending = True
        error = False
        render()
        try:
            use_password(entries[selected_index])
        except Exception as err:
            print(err)
            error = True
        sending = False
        render()

    @btns.when_activated(Key.K1)
    def on_exit():
        nonlocal done
        done = True

    def render():
        nonlocal sending, selected_index, scroll_offset
        rows = 6
        row_height = 10

        # Move scroll offset until selected index is visible
        scroll_offset = min(
            max(scroll_offset, selected_index - rows - 1),
            selected_index)

        with canvas(device) as draw:
            for i in range(rows):
                entry_i = scroll_offset + i
                if entry_i >= len(entries):
                    break  # no more entries to draw

                entry = entries[entry_i]
                if i == selected_index:
                    if sending:
                        selector = '*'
                    elif error:
                        selector = '!'
                    else:
                        selector = '>'
                else:
                    selector = ' '
                draw.text((0, row_height * i), selector + entry.name, fill='white')

    render()

    while not done:
        sleep(0.5)
