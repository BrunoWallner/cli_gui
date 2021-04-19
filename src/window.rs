#![allow(unused_imports)]
#![allow(dead_code)]

use std::io::{self, Write, stdout, stdin};
use crossterm::{
    execute,
    style::{self, Colorize},
    Result,
    cursor,
    terminal,
};

pub static SIZE: [u16; 2] = [125, 45];

pub struct Size {
    pub x: u16,
    pub y: u16,
} impl Size {
    pub fn new(x: u16, y: u16) ->  Self {
        Size {x: x, y: y}
    }
}

pub struct Position {
    pub x: u16,
    pub y: u16,
} impl Position {
    pub fn new(x: u16, y: u16) ->  Self {
        Position {x: x, y: y}
    }
}

pub struct SubWindow {
    text_buffer: Vec<Vec<String>>,
    color_buffer: Vec<Vec<u8>>,
    pub size: Size,
    pub pos: Position,
    border_color: u8,
    border_symbols: [String; 4],
    title: String,
    title_color: u8,
}
impl SubWindow {
    pub fn new(pos: Position, size: Size) -> Self {
        SubWindow {
            text_buffer: vec![vec!["  ".to_string(); SIZE[1] as usize + 1]; SIZE[0] as usize + 1], 
            color_buffer: vec![vec![0; SIZE[1] as usize + 1]; SIZE[0] as usize + 1],
            size: size,
            pos: pos,
            border_color: 0,
            border_symbols: ["▏".to_string(), "▕".to_string(), "▁".to_string(), "▔".to_string()],
            title: "title".to_string(),
            title_color: 0,
        }
    }

    pub fn write(&mut self, pos: [usize; 2], text: String, color: u8) {
        let mut y = 0;
        let mut x = 0;
        for i in 0..text.len() as usize {
            // checks if position is valid and corrects it if neccessary
            if pos[0] + x < self.size.x as usize && pos[1] < self.size.y as usize {

                let char_vec: Vec<char> = text.chars().collect();

                self.text_buffer[pos[0] + x][pos[1] + y] = char_vec[i].to_string();
                self.color_buffer[pos[0] + x][pos[1] + y] = color;

                x += 1;

                if char_vec[i].to_string() == "\n".to_string() {
                    y += 1;
                    x = 0;
                }

            }          
        }      
    }

    pub fn set_border_color(&mut self, color: u8) {
        self.border_color = color;
    }
    pub fn set_border_symbols(&mut self, symbols: [String; 4]) {
        self.border_symbols = symbols;
    }

    pub fn set_title(&mut self, title: String) {
        self.title = title;
    }
    pub fn set_title_color(&mut self, color: u8) {
        self.title_color = color;
    }

    pub fn clear(&mut self) {
        for y in 0..SIZE[1] as usize {
            for x in 0..SIZE[0] {
                self.text_buffer[x as usize][y as usize] = "".to_string();
                self.color_buffer[x as usize][y as usize] = 0;
            }
        }
    }

    pub fn move_cursor(&self, pos: Position) {
        execute!(stdout(), cursor::MoveTo(pos.x,pos.y))
            .expect("failed to move cursor");
    }

    pub fn move_window(&mut self, pos: Position) {
        self.pos = pos;
    }

    pub fn input(&self, pos: Position, output_string: &str) -> String {
        execute!(stdout(), cursor::MoveTo(pos.x + self.pos.x, pos.y + self.pos.y))
            .expect("failed to move cursor :(");
        print!("{}", output_string);
        io::stdout().flush().unwrap();
    
        let mut input_string = String::new();
        stdin().read_line(&mut input_string)
            .ok()
            .expect("Failed to read line");
        return input_string.trim().to_string();
    }
}

pub struct Window {
    text_buffer: Vec<Vec<String>>,
    color_buffer: Vec<Vec<u8>>,
    size: Size,
}
impl Window {
    pub fn new(size: Size) -> Self {
        // Terminal setup
        execute!(stdout(), terminal::SetSize(SIZE[0] + 1, SIZE[1] + 1))
            .expect("failed to set Terminal size :(");
        execute!(stdout(), cursor::Hide)
            .expect("failed to hide cursor :(");
        execute!(stdout(), terminal::EnterAlternateScreen)
            .expect("failed to enter alternatescreen");
        execute!(stdout(), cursor::MoveTo(0, 0))
            .expect("failed to move cursor");

        Window {
            text_buffer: vec![vec!["  ".to_string(); SIZE[1] as usize + 1]; SIZE[0] as usize + 1], 
            color_buffer: vec![vec![0; SIZE[1] as usize + 1]; SIZE[0] as usize + 1],
            size: size,
        }
    }

    pub fn write_window(&mut self, window: &SubWindow) {
        for y in 0..window.size.y {
            for x in 0..window.size.x {
                if x + window.pos.x < self.size.x && y + window.pos.y < self.size.y {
                    self.text_buffer[ (x + window.pos.x) as usize ][ (y + window.pos.y) as usize] = window.text_buffer[x as usize][y as usize].clone();
                    self.color_buffer[ (x + window.pos.x) as usize ][ (y + window.pos.y) as usize] = window.color_buffer[x as usize][y as usize];

                    // Window Borders
                    if x == 0 {
                        self.text_buffer[(x + window.pos.x) as usize][(y + window.pos.y) as usize] = window.border_symbols[0].clone();
                        self.color_buffer[ (x + window.pos.x) as usize ][ (y + window.pos.y) as usize ] = window.border_color;
                    }
                    if x == window.size.x - 1 {
                        self.text_buffer[(x + window.pos.x) as usize][(y + window.pos.y) as usize] = window.border_symbols[1].clone();
                        self.color_buffer[ (x + window.pos.x) as usize ][ (y + window.pos.y) as usize ] = window.border_color;
                    }
                    if y == 0 {
                        self.text_buffer[(x + window.pos.x) as usize][(y + window.pos.y) as usize] = window.border_symbols[2].clone();
                        self.color_buffer[ (x + window.pos.x) as usize ][ (y + window.pos.y) as usize ] = window.border_color;
                    }
                    if y == window.size.y - 1 {
                        self.text_buffer[(x + window.pos.x) as usize][(y + window.pos.y) as usize] = window.border_symbols[3].clone();
                        self.color_buffer[ (x + window.pos.x) as usize ][ (y + window.pos.y) as usize ] = window.border_color;
                    }

                    // Window Title
                    let mut y_title = 0;
                    let mut x_title = 0;
                    let pos: [u16; 2] = [window.size.x / 2 - (window.title.len() / 2 + 1) as u16 + window.pos.x, 1 + window.pos.y];
                    for i in 0..window.title.len() as usize {
                        // checks if position is valid and corrects it if neccessary
                        if pos[0] + x < self.size.x && pos[1] < self.size.y {
            
                            let char_vec: Vec<char> = window.title.chars().collect();
            
                            self.text_buffer[(pos[0] + x_title) as usize][(pos[1] + y_title) as usize] = char_vec[i].to_string();
                            self.color_buffer[(pos[0] + x_title) as usize][(pos[1] + y_title) as usize] = window.title_color;
            
                            x_title += 1;
                            
                            if char_vec[i].to_string() == "\n".to_string() {
                                y_title += 1;
                                x_title = 0;
                            }
                        }          
                    }  
                }
            }
        }
    }

    pub fn render(&self) {
        for y in 0..self.size.y {
            for x in 0..self.size.x {
                let text_slice: &str = &*self.text_buffer[x as usize][y as usize];

                let mut stdout = stdout();
                match self.color_buffer[x as usize][y as usize] {
                    0 => execute!(stdout, cursor::MoveTo(x,y), style::PrintStyledContent( text_slice.white())),
                    1 => execute!(stdout, cursor::MoveTo(x,y), style::PrintStyledContent( text_slice.red())),
                    2 => execute!(stdout, cursor::MoveTo(x,y), style::PrintStyledContent( text_slice.green())),
                    3 => execute!(stdout, cursor::MoveTo(x,y), style::PrintStyledContent( text_slice.blue())),
                    4 => execute!(stdout, cursor::MoveTo(x,y), style::PrintStyledContent( text_slice.cyan())),
                    5 => execute!(stdout, cursor::MoveTo(x,y), style::PrintStyledContent( text_slice.magenta())),
                    _ => execute!(stdout, cursor::MoveTo(x,y), style::PrintStyledContent( "??".red())),
                }.expect("failed to render buffer :(");

            }
        }
    }
    
    pub fn clear(&mut self) {
        for y in 0..SIZE[1] as usize {
            for x in 0..SIZE[0] {
                self.text_buffer[x as usize][y as usize] = "".to_string();
                self.color_buffer[x as usize][y as usize] = 0;
            }
        }
        execute!(stdout(), terminal::Clear(terminal::ClearType::All)).
            expect("failed to clear Terminal");
    }

    pub fn move_cursor(&self, pos: Position) {
        execute!(stdout(), cursor::MoveTo(pos.x,pos.y))
            .expect("failed to move cursor");
    }

    pub fn quit(&self) {
        self.move_cursor(Position::new(0, 0));
        execute!(stdout(), terminal::LeaveAlternateScreen)
            .expect("failed to leave alternatescreen :(");
        stdout().flush()
            .expect("failed to flush stdout :(");
    }
}