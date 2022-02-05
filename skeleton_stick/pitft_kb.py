"""
A driver for translating PiTFT Joystick keys into keystrokes.

The PiTFT is from Adafruit, here:
https://www.adafruit.com/product/4506
"""
import keyboard
from gpiozero import Button


_up = 17
_down = 22
_left = 27
_right = 23
_center = 4
_five = 5
_six = 6

_gpio_to_key = {
    _up: 'up',
    _down: 'down',
    _left: 'left',
    _right: 'right',
    _center: 'space',
    _five: 'backspace',
    _six: 'enter',
}

# See https://learn.adafruit.com/adafruit-1-3-color-tft-bonnet-for-raspberry-pi/pinouts


class PiTFTDriver:
    """Driver."""

    def __init__(self):
        def mk_button(pin, keystroke):
            btn = Button(pin, None, False)
            btn.when_activated = lambda: keyboard.press(keystroke)
            btn.when_deactivated = lambda: keyboard.release(keystroke)
            return btn

        self.buttons = [mk_button(pin, keystroke)
                        for pin, keystroke in _gpio_to_key.items()]

    def close(self):
        """Closes this driver."""
        for b in self.buttons:
            b.close()
