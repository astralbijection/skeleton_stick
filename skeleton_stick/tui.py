import curses
import os
from pathlib import Path
from typing import Callable, List, Optional, TypeVar

from cursesmenu import CursesMenu
from cursesmenu.items import FunctionItem
from skeleton_stick.hid import write_keyboard

from skeleton_stick.storage import PasswordEntry, make_loader

SPINNER = "/-\\|"


def start_tui(password_file: Path):
    """Start the TUI."""

    device_path = Path(os.environ.get("HID_GADGET", "/dev/hidg0"))

    def use_password(entry: PasswordEntry) -> None:
        """The callback for when passwords are used."""
        with device_path.open('wb') as file:
            write_keyboard(file, entry.password)

    while True:
        entries = password_prompt(verify=make_loader(password_file))
        if not entries:
            continue

        password_browser(entries, use_password=use_password)


R = TypeVar('R')


def password_prompt(verify: Callable[[str], Optional[R]]) -> Optional[R]:
    """
    Gets a password from the user.
    """

    def loop(stdscr):
        pw: List[str] = []
        status: Optional[str] = None

        def render():
            stdscr.clear()
            stdscr.addstr(0, 0, 'Enter your password using the arrows')
            stdscr.addstr(1, 0, "*" * len(pw))
            if status:
                stdscr.addstr(2, 0, status)
            stdscr.refresh()

        while True:
            render()

            ch = stdscr.getch()
            if ch == curses.ERR:
                continue
            elif ch == curses.KEY_LEFT:
                pw.append('L')
            elif ch == curses.KEY_RIGHT:
                pw.append('R')
            elif ch == curses.KEY_UP:
                pw.append('U')
            elif ch == curses.KEY_DOWN:
                pw.append('D')
            elif ch == curses.KEY_BACKSPACE:
                if len(pw) > 0:
                    pw.pop()
                else:
                    return None
            elif ch == 10:
                status = "Verifying..."
                render()
                result = verify(''.join(pw))
                if result:
                    return result

                status = "Invalid password"

    return curses.wrapper(loop)


def password_browser(entries: List[PasswordEntry], use_password: Callable[[PasswordEntry], None]):
    """Browse passwords and use them."""
    menu = CursesMenu("Passwords", "Choose a password to send")

    for entry in entries:
        menu.append_item(
            FunctionItem(
                entry.name,
                use_password,
                args=[entry],
                should_exit=False
            )
        )

    menu.show(show_exit_option=True)
