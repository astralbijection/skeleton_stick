"""Tests for storage utilities."""
from io import BytesIO

from .storage import PasswordEntry, load, save


def test_load_save():
    """Load and save should result in correct values."""

    key = 'ABABCCDD'
    file = BytesIO()
    pws = [
        PasswordEntry('Disk', 'foo bar spam'),
        PasswordEntry('User account', 'foo bar spam'),
    ]

    save(file, key, pws)
    file.seek(0)
    result = load(file, key)

    assert result == pws
