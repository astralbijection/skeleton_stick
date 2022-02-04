"""Tests for HID utilities."""
import pytest
from .convert import char_to_report


@pytest.mark.parametrize(
    "test_input,expected",
    [
        ("f", b'\0\0\x09\0\0\0\0\0'),
        ("/", b'\0\0\x38\0\0\0\0\0'),
        ("'", b'\0\0\x34\0\0\0\0\0'),
        (";", b'\0\0\x33\0\0\0\0\0'),
        ("_", b'\2\0\x2D\0\0\0\0\0'),
        ("*", b'\2\0\x25\0\0\0\0\0'),
        ("|", b'\2\0\x31\0\0\0\0\0'),
        ("P", b'\2\0\x13\0\0\0\0\0'),
        ("A", b'\2\0\x04\0\0\0\0\0'),
    ]
)
def test_char_to_report(test_input, expected):
    """The char to report converter should be correct."""
    assert char_to_report(test_input) == expected
