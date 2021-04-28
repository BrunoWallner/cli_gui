mod window;
mod terminal;

pub use window::*;
pub use terminal::*;

pub use crossterm::event::KeyCode;
pub use crossterm::event::KeyEvent;
pub use crossterm::event::KeyModifiers;
pub use crossterm::event::MouseEventKind;
pub use crossterm::event::MouseEvent;

#[derive(Copy, Clone)]
pub struct Size {
    pub x: i32,
    pub y: i32,
} impl Size {
    pub fn new(x: i32, y: i32) ->  Self {
        Size {x: x, y: y}
    }
}

#[derive(Clone)]
pub struct Pixel {
    pub text: String,
    pub color: Color,
} impl Pixel {
    pub fn new(text: String, color: Color) ->  Self {
        Pixel {text: text, color: color}
    }
}

#[derive(Copy, Clone)]
pub struct Position {
    pub x: i32,
    pub y: i32,
} impl Position {
    pub fn new(x: i32, y: i32) ->  Self {
        Position {x: x, y: y}
    }
}

#[derive(Copy, Clone)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
} impl Color {
    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Color {r: r, g: g, b: b}
    }
    pub fn white() -> Self {
        Color {r: 255, g: 255, b: 255}
    }
    pub fn red() -> Self {
        Color {r: 255, g: 0, b: 0}
    }
    pub fn green() -> Self {
        Color {r: 0, g: 255, b: 0}
    }
    pub fn blue() -> Self {
        Color {r: 0, g: 0, b: 255}
    }
}


