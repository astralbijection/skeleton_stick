import json
import os
from pathlib import Path
from signal import pause
import sys
import click
from skeleton_stick.hid import setup_hid
from skeleton_stick.oled import start_oled
from skeleton_stick.pitft_kb import PiTFTDriver

from skeleton_stick.tui import start_tui

from .storage import PasswordEntry, save


@click.group()
def cli():
    """Main CLI entrypoint."""


@cli.command()
@click.argument("password_file")
def tui(password_file):
    """Run the TUI with the given password file."""
    start_tui(Path(password_file))


@cli.command()
@click.argument("password_file")
def oled(password_file):
    """Run the OLED interface with the given password file."""
    start_oled(Path(password_file))


@cli.command()
def keyboard():
    """Run the keyboard driver daemon."""
    driver = PiTFTDriver()
    pause()
    driver.close()


@cli.command()
def setup_gadget():
    """Set up the USB keyboard gadget."""
    udc_path = next(Path('/sys/class/udc').iterdir())
    udc = udc_path.name
    print("UDC:", udc)
    setup_hid(
        Path('/sys/kernel/config/usb_gadget/skeleton_stick'),
        udc=udc
    )


@cli.command()
def pw_import():
    """
    Generate passwords from Bitwarden JSON in STDIN. Example usage:

    export SKELETON_STICK_PIN=UUDDLLRR
    bw list items --url skeletonstick.astrid.tech | skeleton_stick import > passwords.skst
    """
    password = os.environ.get('SKELETON_STICK_PIN')
    if not password:
        print("Must specify a password using environment variable SKELETON_STICK_PIN")
        sys.exit(1)

    data = json.load(sys.stdin)

    entries = [
        PasswordEntry(
            name=e['name'], password=e['login']['password']
        ) for e in data
    ]

    save(sys.stdout.buffer, password, entries)


if __name__ == '__main__':
    cli()
