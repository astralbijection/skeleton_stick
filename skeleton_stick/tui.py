import curses
from curses import wrapper
from pathlib import Path
from typing import Callable, List, Optional, TypeVar

from skeleton_stick.storage import make_loader

SPINNER = "/-\\|"


def start_tui(password_file: Path):
    """Start the TUI."""
    def main(stdscr):
        """
        Main entrypoint
        """

        # Clear screen
        stdscr.clear()

        while True:
            result = password_prompt(stdscr, verify=make_loader(password_file))
    
    wrapper(main)


R = TypeVar('R')


def password_prompt(stdscr, verify: Callable[[str], Optional[R]]) -> Optional[R]:
    """
    Gets a password from the user.
    """

    # Clear screen
    stdscr.clear()

    win = curses.newwin(10, 50)

    pw: List[str] = []
    status: Optional[str] = None

    def render():
        win.clear()
        win.addstr(0, 0, 'Enter your password using the arrows')
        win.addstr(1, 0, "*" * len(pw))
        if status:
            win.addstr(2, 0, status)
        win.refresh()

    render()
    stdscr.refresh()

    while True:
        render()

        match stdscr.getch():
            case curses.ERR:
                continue
            case curses.KEY_LEFT:
                pw.append('L')
            case curses.KEY_RIGHT:
                pw.append('R')
            case curses.KEY_UP:
                pw.append('U')
            case curses.KEY_DOWN:
                pw.append('D')
            case curses.KEY_BACKSPACE:
                if len(pw) > 0:
                    pw.pop()
                else:
                    return None
            case 10:
                status = "Verifying..."
                render()
                result = verify(''.join(pw))
                if result:
                    return result

                status = "Invalid password"


if __name__ == '__main__':
    wrapper(main)
