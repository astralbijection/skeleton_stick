"""
An OLED interface for the pi.

https://www.waveshare.com/1.3inch-oled-hat.htm
"""

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

from skeleton_stick.hid import write_keyboard
from skeleton_stick.storage import PasswordEntry, make_loader



RST_PIN  = 25 
CS_PIN   = 8
DC_PIN   = 24


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
            k: Button(k, pull_up=None, active_state=False)
            for k in Key
        }
    
    def __enter__(self):
        return self
    
    def __exit__(self, type, value, traceback):
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
        """The callback for when passwords are used."""
        with device_path.open('wb') as file:
            write_keyboard(file, entry.password)
    
    serial = spi(device=0, port=0, bus_speed_hz=8000000, transfer_size=4096, gpio_DC=24, gpio_RST=25)  
    device = sh1106(serial, rotate=2) # sh1106  

    while True:
        with canvas(device, dither=True) as draw:
            draw.polygon((55.55592521270453, 23.789394308088617, 72.67547160800935, 20.615969890546587, 76.43979309387001, 38.06268012357415, 59.30609696779982, 41.55814842333336), 'red', 'red')
        sleep(1)

    device.show()

    # with HATButtons() as btns:
        # while True:
            # entries = password_prompt(btns, device, verify=make_loader(password_file))
            # btns.detach_listeners()

            # if not entries:
                # continue

            # password_browser(entries, use_password=use_password)
            # btns.detach_listeners()


R = TypeVar("R")


def password_prompt(btns: HATButtons, device: sh1106, verify: Callable[[str], Optional[R]]) -> Optional[R]:
    """
    Gets a password from the user.
    """

    pw: List[str] = []
    status: Optional[str] = None
    result: Optional[R] = None

    '''
    @btns.when_activated(Key.LEFT)
    def on_left():
        pw.append('L')

    @btns.when_activated(Key.RIGHT)
    def on_right():
        pw.append('R')

    @btns.when_activated(Key.UP)
    def on_up():
        pw.append('U')

    @btns.when_activated(Key.DOWN)
    def on_down():
        pw.append('D')

    @btns.when_activated(Key.K1)
    def on_k1():
        pw.append('1')

    @btns.when_activated(Key.K2)
    def on_k2():
        pw.append('2')

    @btns.when_activated(Key.K3)
    def on_k3():
        nonlocal result, status
        result = verify(''.join(pw))
        if not result:
            status = 'Wrong password'
    '''

    def render():
        with canvas(device) as draw:
            draw.line(((0, 0), (10, 10)))
            # draw.text((0, 0), 'Enter your password using the arrows')
            # draw.text((1, 0), "*" * len(pw))
            # if status:
            #     draw.text((2, 0), status)

    while not result:
        render()
        sleep(1)

    return result


def password_browser(btns: HATButtons, device: sh1106, entries: List[PasswordEntry], use_password: Callable[[PasswordEntry], None]):
    """Browse passwords and use them."""

    @btns.when_activated(Key.UP)
    def on_up():
        pw.append('U')

    @btns.when_activated(Key.DOWN)
    def on_down():
        pw.append('D')

    @btns.when_activated(Key.K1)
    def on_k1():
        pw.append('1')

    @btns.when_activated(Key.K2)
    def on_k2():
        pw.append('2')

    @btns.when_activated(Key.K3)
    def on_k3():
        verify(''.join(pw))

    def render():
        with canvas(device) as draw:
            draw.text(0, 0, 'Enter your password using the arrows')
            draw.text(1, 0, "*" * len(pw))
            if status:
                draw.text(2, 0, status)

    while not done:
        render()
        sleep(1)
