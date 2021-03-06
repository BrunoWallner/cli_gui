mod window;
mod terminal;

pub use window::*;
pub use terminal::*;

pub use crossterm::event::KeyCode;
pub use crossterm::event::KeyEvent;
pub use crossterm::event::KeyModifiers;

#[derive(Copy, Clone)]
pub struct Size {
    pub x: u16,
    pub y: u16,
} impl Size {
    pub fn new(x: u16, y: u16) ->  Self {
        Size {x: x, y: y}
    }
}

#[derive(Copy, Clone)]
pub struct Position {
    pub x: u16,
    pub y: u16,
} impl Position {
    pub fn new(x: u16, y: u16) ->  Self {
        Position {x: x, y: y}
    }
}

#[derive(Copy, Clone)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
} impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Color {r: r, g: g, b: b}
    }
}


