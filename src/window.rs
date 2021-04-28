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

//use colored::Colorize;

use crate::Position;
use crate::Size;
use crate::Color;
use crate::Terminal;
use crate::Pixel;

use rand::{thread_rng, Rng};

#[derive(Clone)]
pub struct Window {
    pub id: u128,
    pub pixel_buffer: Vec<Pixel>,
    pub size: Size,
    pub pos: Position,
    border_color: Color,
    border_symbols: [String; 8],
    title: String,
    title_color: Color,
}
impl Window {
    pub fn new(pos: Position, size: Size) -> Self {
        Window {
            id: thread_rng().gen_range(0..u128::MAX),
            pixel_buffer: vec![Pixel::new(" ".to_string(), Color::white()); (size.x * size.y) as usize],
            size: size,
            pos: pos,
            border_color: Color::rgb(255, 255, 255),
            border_symbols: ["│".to_string(), "│".to_string(), "─".to_string(), "─".to_string(),
                 "┌".to_string(), "┐".to_string(), "┘".to_string(), "└".to_string()], // ─ │ ┘ └ ┐ ┌
            title: "title".to_string(),
            title_color: Color::rgb(255, 255, 255),
        }
    }

    pub fn write(&mut self, pos: Position, text: String, color: Color) {
        let mut y = 0;
        let mut x = 0;

        let char_vec: Vec<char> = text.chars().collect();
        for i in 0..char_vec.len() as usize {
            // checks if position is valid and corrects it if neccessary
            if pos.x + x < self.size.x && pos.y < self.size.y {

                self.pixel_buffer[((pos.x + x) + ((pos.y + y) * self.size.x)) as usize] = Pixel::new(char_vec[i].to_string(), color);

                x += 1;
                
                match char_vec[i].to_string().as_str() {
                    "\n" => {y += 1; x = 0},
                    "\t" => {x += 4},
                    "\r" => {if y >= 1 {y -= 1}},
                    _ => (),
                }
            }          
        }      
    }

    pub fn set_border_color(&mut self, color: Color) {
        self.border_color = color;
    }
    pub fn set_border_symbols(&mut self, symbols: [String; 8]) {
        self.border_symbols = symbols;
    }

    pub fn set_title(&mut self, title: String) {
        self.title = title;
    }
    pub fn set_title_color(&mut self, color: Color) {
        self.title_color = color;
    }
    pub fn resize(&mut self, size: Size) {
        self.size = size;
        self.pixel_buffer = vec![Pixel::new(" ".to_string(), Color::white()); (self.size.x * self.size.y) as usize];
    }

    pub fn clear(&mut self) {
        self.pixel_buffer = vec![Pixel::new(" ".to_string(), Color::white()); (self.size.x * self.size.y) as usize];
    }

    pub fn set_position(&mut self, pos: Position) {
        self.pos = pos;
    }

    pub fn decorate(&mut self) {
        for y in 0..self.size.y {
            for x in 0..self.size.x {
                // draws Borders
                if x == 0 {
                    self.write(Position::new(x, y), self.border_symbols[0].clone(), self.border_color);
                }
                if x == self.size.x - 1 {
                    self.write(Position::new(x, y), self.border_symbols[1].clone(), self.border_color);
                }
                if y == 0 {
                    self.write(Position::new(x, y), self.border_symbols[2].clone(), self.border_color);
                }
                if y == self.size.y - 1 {
                    self.write(Position::new(x, y), self.border_symbols[3].clone(), self.border_color);
                }
            }
        }
        // draws edges
        // upper left
        self.write(Position::new(0, 0), self.border_symbols[4].clone(), self.border_color);

        // upper right
        self.write(Position::new(self.size.x - 1, 0), self.border_symbols[5].clone(), self.border_color);

        // bottom right
        self.write(Position::new(self.size.x - 1, self.size.y - 1), self.border_symbols[6].clone(), self.border_color);

        // bottom left
        self.write(Position::new(0, self.size.y - 1), self.border_symbols[7].clone(), self.border_color);

        // draws Title  100ms !!!
        let pos: Position = Position::new(self.size.x / 2 - (self.title.len() / 2) as i32, 0);
        self.write(pos, self.title.clone(), self.title_color);
    }

    pub fn write_window(&mut self, window: &Window) {
        for y in 0..window.size.y {
            for x in 0..window.size.x {
                if x + window.pos.x < self.size.x && y + window.pos.y < self.size.y
                && x + window.pos.x > 0 && y + window.pos.y > 0 {
                    self.pixel_buffer[((x + window.pos.x) + ((y + window.pos.y) * self.size.x)) as usize] = 
                        window.pixel_buffer[(x + (y * window.size.x)) as usize].clone();
                }
            }
        }
    }
}
