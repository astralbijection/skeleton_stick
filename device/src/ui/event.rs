#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Event {
    KeyDown(Key),
    KeyUp(Key),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Key {
    Up,
    Down,
    Left,
    Right,
    Center,

    Back,
    Enter,
    Shift,
}