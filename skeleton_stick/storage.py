"""
Utilities for interacting with the password storage.

The password data can be stored on a file, or as the leading bits
of a block device.
"""

import json
from hashlib import pbkdf2_hmac
from io import BytesIO
from pathlib import Path
from struct import Struct
from typing import Callable, List, NamedTuple, Optional

from Crypto.Cipher import AES
from Crypto.Random import get_random_bytes
from Crypto.Util.Padding import pad, unpad


FILE_START = b'__SKELETONSTICK\n'
"""The symbols to detect the file starting."""

CURRENT_VERSION = 0
"""The current file format version."""

VERSION_FMT = Struct('>I')
"""The format of the version in the file."""

HEADER_FMT = Struct('>32s16s16sI')
"""
The format of the file header after the version number. In order, it is:
- 32 bytes salt
- 16 bytes nonce
- 16 bytes MAC
- 4 bytes unsigned int body length
"""

PBKDF_ITERS = 1_000_000
"""Number of iterations to run PBKDF for."""


class PasswordEntry(NamedTuple):
    """
    How the password is represented on disk.
    """
    name: str
    password: str


def save(file: BytesIO, passphrase: str, passwords: List[PasswordEntry]):
    """
    Securely saves password data to the given path.
    """

    # Generate the key
    salt = get_random_bytes(32)
    key = kdf(passphrase, salt)

    plaintext = serialize(passwords)

    nonce = get_random_bytes(16)
    cipher = AES.new(key, AES.MODE_EAX, nonce=nonce, mac_len=16)
    ciphertext, mac_tag = cipher.encrypt_and_digest(pad(plaintext, 32))

    file.write(FILE_START)
    file.write(VERSION_FMT.pack(CURRENT_VERSION))
    file.write(HEADER_FMT.pack(salt, mac_tag, nonce, len(ciphertext)))
    file.write(ciphertext)


def make_loader(path: Path) -> Callable[[str], Optional[PasswordEntry]]:
    """Curried version of load."""
    def verify(key: str):
        with path.open('rb') as file:
            try:
                return load(file, key)
            except ValueError:
                return None
    return verify


def load(file: BytesIO, passphrase: str) -> PasswordEntry:
    """
    Securely loads password data from the given file.
    """
    start = file.read(len(FILE_START))
    if start != FILE_START:
        raise ValueError("Bad file start")

    version, = VERSION_FMT.unpack(file.read(VERSION_FMT.size))
    if version > CURRENT_VERSION:
        raise ValueError("Unsupported version")

    salt, mac_tag, nonce, ciphertext_length = HEADER_FMT.unpack(
        file.read(HEADER_FMT.size))
    ciphertext = file.read(ciphertext_length)

    key = kdf(passphrase, salt)
    cipher = AES.new(key, AES.MODE_EAX, nonce=nonce, mac_len=16)

    try:
        decrypted = cipher.decrypt_and_verify(ciphertext, mac_tag)
    except ValueError as ex:
        raise ValueError("Wrong password provided") from ex

    plaintext = unpad(decrypted, 32)
    return deserialize(plaintext)


def kdf(passphrase: str, salt: bytes) -> bytes:
    """The key derivation function."""
    return pbkdf2_hmac('sha256', passphrase.encode('utf8'), salt, PBKDF_ITERS)


def serialize(passwords: List[PasswordEntry]) -> bytes:
    """
    Save a list of passwords to a bytes object.
    """
    json_object = [x._asdict() for x in passwords]
    return json.dumps(json_object).encode('utf8')


def deserialize(data: bytes) -> List[PasswordEntry]:
    """
    Reads a bytes object into a PasswordEntry list.
    """
    json_object = json.loads(data.decode('utf8'))
    return [PasswordEntry(**x) for x in json_object]
