"""
Utilities for converting from characters to HID reports.
"""

_UNSHIFT = "abcdefghijklmnopqrstuvwxyz1234567890" + " -=[]\\" + ";'`,./"
_SHIFT =   "ABCDEFGHIJKLMNOPQRSTUVWXYZ!@#$%^&*()" + " _+{}|"  + ':"~<>?'
_ids = list(range(0x04, 0x28)) + list(range(0x2c, 0x32)) + list(range(0x33, 0x39))
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


def char_to_report(char: str) -> bytes:
    """
    Given a character, returns its HID report.
    """
    return _char_to_report[char]
