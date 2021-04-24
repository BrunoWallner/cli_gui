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

use rand::{thread_rng, Rng};

#[derive(Clone)]
pub struct Window {
    pub id: u128,
    pub text_buffer: Vec<Vec<String>>,
    pub color_buffer: Vec<Vec<Color>>,
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
            text_buffer: vec![vec!["  ".to_string(); size.y as usize + 1]; size.x as usize + 1], 
            color_buffer: vec![vec![Color::new(0, 0, 0); size.y as usize + 1]; size.x as usize + 1],
            size: size,
            pos: pos,
            border_color: Color::new(255, 255, 255),
            border_symbols: ["│".to_string(), "│".to_string(), "─".to_string(), "─".to_string(),
                 "┌".to_string(), "┐".to_string(), "┘".to_string(), "└".to_string()], // ─ │ ┘ └ ┐ ┌
            title: "title".to_string(),
            title_color: Color::new(255, 255, 255),
        }
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
        self.text_buffer = vec![vec!["  ".to_string(); size.y as usize + 1]; size.x as usize + 1];
        self.color_buffer = vec![vec![Color::new(0, 0, 0); size.y as usize + 1]; size.x as usize + 1];
    }

    pub fn clear(&mut self) {
        for y in 1..self.size.y - 1 {
            for x in 1..self.size.x - 1 {
                self.text_buffer[x as usize][y as usize] = " ".to_string();
                self.color_buffer[x as usize][y as usize] = Color::new(0, 0, 0);
            }
        }
    }

    pub fn set_position(&mut self, pos: Position) {
        self.pos = pos;
    }

    pub fn decorate(&mut self) {
        for y in 0..self.size.y {
            for x in 0..self.size.x {

                if x < self.size.x && y < self.size.y {

                    // draws Borders
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
                    // draws edges
                    // upper left
                    self.text_buffer[0][0] = self.border_symbols[4].clone();
                    self.color_buffer[0][0] = self.border_color;

                    // upper right
                    self.text_buffer[(self.size.x - 1) as usize][0] = self.border_symbols[5].clone();
                    self.color_buffer[(self.size.x - 1)as usize][0] = self.border_color;

                    // bottom left
                    self.text_buffer[(self.size.x - 1) as usize][(self.size.y - 1) as usize] = self.border_symbols[6].clone();
                    self.color_buffer[(self.size.x - 1) as usize][(self.size.y - 1) as usize] = self.border_color;

                    // bottom right
                    self.text_buffer[0][(self.size.y - 1) as usize] = self.border_symbols[7].clone();
                    self.color_buffer[0][(self.size.y - 1) as usize] = self.border_color;


                    // draws Title  100ms !!!
                    let pos: Position = Position::new(self.size.x / 2 - (self.title.len() / 2) as u16, 0);
                    self.write(pos, self.title.clone(), self.title_color);
                }
            }
        }
    }

    pub fn write_window(&mut self, window: &Window) {
        for y in 0..window.size.y {
            for x in 0..window.size.x {
                if x + window.pos.x < self.size.x && y + window.pos.y < self.size.y {
                    self.text_buffer[(x + window.pos.x) as usize][(y + window.pos.y) as usize] = window.text_buffer[x as usize][y as usize].clone();
                    self.color_buffer[(x + window.pos.x) as usize][(y + window.pos.y) as usize] = window.color_buffer[x as usize][y as usize].clone();
                }
            }
        }
    }
}
