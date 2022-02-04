import json
import os
from pathlib import Path
import sys
import click

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
