#![allow(unused_imports)]
#![allow(dead_code)]

use std::{thread, time::Duration};

use std::io::{self, Write, stdout, stdin};
use crossterm::{
    execute,
    Result,
    cursor,
    terminal,
};
use crossterm::event;
use crossterm::event::*;
use crossterm::terminal::enable_raw_mode;
use crossterm::terminal::disable_raw_mode;

use colored::Colorize;

use crate::Position;
use crate::Size;
use crate::Color;

pub struct Window {
    text_buffer: Vec<Vec<String>>,
    color_buffer: Vec<Vec<Color>>,
    pub size: Size,
    pub pos: Position,
    border_color: Color,
    border_symbols: [String; 4],
    title: String,
    title_color: Color,
}
impl Window {
    pub fn new(pos: Position, size: Size) -> Self {
        Window {
            text_buffer: vec![vec!["  ".to_string(); size.y as usize + 1]; size.x as usize + 1], 
            color_buffer: vec![vec![Color::new(0, 0, 0); size.y as usize + 1]; size.x as usize + 1],
            size: size,
            pos: pos,
            border_color: Color::new(255, 255, 255),
            border_symbols: ["▏".to_string(), "▕".to_string(), "▁".to_string(), "▔".to_string()],
            title: "title".to_string(),
            title_color: Color::new(255, 255, 255),
        }
    }

    pub fn set_to_main(&self) {
            // Terminal setup
            execute!(stdout(), terminal::SetSize(self.size.x + 1, self.size.y + 1))
                .expect("failed to set Terminal size :(");
            execute!(stdout(), cursor::Hide)
                .expect("failed to hide cursor :(");
            execute!(stdout(), terminal::EnterAlternateScreen)
                .expect("failed to enter alternatescreen");
            execute!(stdout(), cursor::MoveTo(0, 0))
                .expect("failed to move cursor");
            enable_raw_mode()
                .expect("failed to go into raw mode :(");
    }

    pub fn write(&mut self, pos: Position, text: String, color: Color) {
        let mut y = 0;
        let mut x = 0;

        let char_vec: Vec<char> = text.chars().collect();
        for i in 0..char_vec.len() as usize {
            // checks if position is valid and corrects it if neccessary
            if pos.x + x < self.size.x && pos.y < self.size.y {

                self.text_buffer[(pos.x + x) as usize][(pos.y + y) as usize] = char_vec[i].to_string();
                self.color_buffer[(pos.x + x) as usize][(pos.y + y) as usize] = color;

                x += 1;

                if char_vec[i].to_string() == "\n".to_string() {
                    y += 1;
                    x = 0;
                }
            }          
        }      
    }

    pub fn set_border_color(&mut self, color: Color) {
        self.border_color = color;
    }
    pub fn set_border_symbols(&mut self, symbols: [String; 4]) {
        self.border_symbols = symbols;
    }

    pub fn set_title(&mut self, title: String) {
        self.title = title;
    }
    pub fn set_title_color(&mut self, color: Color) {
        self.title_color = color;
    }

    pub fn clear(&mut self) {
        for y in 0..self.size.y as usize {
            for x in 0..self.size.x {
                self.text_buffer[x as usize][y as usize] = " ".to_string();
                self.color_buffer[x as usize][y as usize] = Color::new(0, 0, 0);
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


    pub fn read_char(&self) -> char {
        loop {
            if poll(Duration::from_millis(1)).expect("") {
                if let Event::Key(KeyEvent {
                    code: KeyCode::Char(c),
                    ..
                }) = event::read().expect("")
                {
                    return c;
                }
            } else {
                return ' ';
            }
        }
    }

    pub fn read_line(&mut self, pos: Position, output_string: &str, color: Color, write_line: bool) -> String {
        execute!(stdout(), cursor::MoveTo(pos.x + self.pos.x, pos.y + self.pos.y))
            .expect("failed to move cursor :(");
        print!("{}", output_string.truecolor(color.r, color.g, color.b));
        io::stdout().flush().unwrap();

        let mut line = String::new();
        while let Event::Key(KeyEvent { code, .. }) = event::read().expect("failed to read event :(") {
            match code {
                KeyCode::Char(c) => {
                    if write_line {
                        let output_vec: Vec<char> = output_string.chars().collect();
                        let line_vec: Vec<char> = line.chars().collect();

                        let x = (pos.x + self.pos.x) as usize + line_vec.len() + output_vec.len();
                        let y = (pos.y + self.pos.y)  as usize;
                        self.move_cursor(Position::new(x as u16, y as u16));

                        print!("{}", c.to_string().truecolor(color.r, color.g, color.b));
                        io::stdout().flush().unwrap();
                    }
                    line.push(c);
                },
                KeyCode::Enter => {
                    break;
                },
                KeyCode::Backspace => {
                    if write_line {
                        let output_vec: Vec<char> = output_string.chars().collect();
                        let line_vec: Vec<char> = line.chars().collect();

                        let x = (pos.x + self.pos.x) as usize + line_vec.len() + output_vec.len() - 1;
                        let y = (pos.y + self.pos.y)  as usize;
                        
                        execute!(stdout(), cursor::MoveTo(x as u16, y as u16))
                            .expect("failed to move cursor :(");
                        print!(" ");
                        io::stdout().flush().unwrap();
                    }
                    line.pop();
                }
                _ => {}
            }
        }
    
        return line;
    }

    pub fn render(&mut self) {
        for y in 0..self.size.y {
            for x in 0..self.size.x {

                if x < self.size.x && y < self.size.y {

                    let text_slice: &str = &*self.text_buffer[x as usize][y as usize];
                    let color = self.color_buffer[x as usize][y as usize].clone();

                   self.move_cursor(Position::new(x + self.pos.x, y + self.pos.y)); // performance bummer!

                    print!("{}", text_slice.truecolor(color.r, color.g, color.b));
                }
            }
        }
        io::stdout().flush().unwrap();
    }

    pub fn decorate(&mut self) {
        for y in 0..self.size.y {
            for x in 0..self.size.x {

                if x < self.size.x && y < self.size.y {

                    // self Borders
                    if x == 0 {
                        self.text_buffer[x as usize][y as usize] = self.border_symbols[0].clone();
                        self.color_buffer[x as usize ][y as usize ] = self.border_color;
                    }
                    if x == self.size.x - 1 {
                        self.text_buffer[x as usize][y as usize] = self.border_symbols[1].clone();
                        self.color_buffer[x as usize ][y as usize ] = self.border_color;
                    }
                    if y == 0 {
                        self.text_buffer[x as usize][y as usize] = self.border_symbols[2].clone();
                        self.color_buffer[x as usize ][y as usize ] = self.border_color;
                    }
                    if y == self.size.y - 1 {
                        self.text_buffer[x as usize][y as usize] = self.border_symbols[3].clone();
                        self.color_buffer[x as usize ][y as usize ] = self.border_color;
                    }


                    // self Title
                    let mut y_title = 0;
                    let mut x_title = 0;
                    let pos: [u16; 2] = [self.size.x / 2 - (self.title.len() / 2 + 1) as u16, 1];

                    let char_vec: Vec<char> = self.title.chars().collect();
                    for i in 0..char_vec.len() as usize {
                        // checks if position is valid and corrects it if neccessary
                        if pos[0] + x < self.size.x && pos[1] < self.size.y {
            
                            self.text_buffer[(pos[0] + x_title) as usize][(pos[1] + y_title) as usize] = char_vec[i].to_string();
                            self.color_buffer[(pos[0] + x_title) as usize][(pos[1] + y_title) as usize] = self.title_color;
            
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

    pub fn quit(&self) {
        self.move_cursor(Position::new(0, 0));
        execute!(stdout(), terminal::LeaveAlternateScreen)
            .expect("failed to leave alternatescreen :(");
        stdout().flush()
            .expect("failed to flush stdout :(");
        disable_raw_mode()
            .expect("failed to leave raw mode :(");    
    }


    pub fn write_window(&mut self, window: &Window) {
        for y in 0..window.size.y {
            for x in 0..window.size.x {
                if x + window.pos.x < self.size.x && y + window.pos.x < self.size.x {
                    self.text_buffer[(x + window.pos.x) as usize][(y + window.pos.y) as usize] = window.text_buffer[x as usize][y as usize].clone();
                    self.color_buffer[(x + window.pos.x)as usize][(y + window.pos.y) as usize] = window.color_buffer[x as usize][y as usize].clone();
                }
            }
        }
    }
}
