use crossterm::execute;
use crossterm::terminal;
use crossterm::cursor;

use std::{thread, time::Duration};

use std::io::{self, Write, stdout, stdin};

use crossterm::event;
use crossterm::event::*;
use crossterm::terminal::enable_raw_mode;
use crossterm::terminal::disable_raw_mode;

use colored::Colorize;

use crate::{Size, Position, Color, Window};

pub struct Terminal {
    pub size: Size,
    pub text_buffer: Vec<Vec<String>>,
    pub color_buffer: Vec<Vec<Color>>,
} impl Terminal {
    pub fn init(size: Size) -> Self {
        // Terminal setup
        execute!(stdout(), terminal::SetSize(size.x, size.y))
            .expect("failed to set Terminal size :(");
        execute!(stdout(), cursor::Hide)
            .expect("failed to hide cursor :(");
        execute!(stdout(), terminal::EnterAlternateScreen)
            .expect("failed to enter alternatescreen");
        execute!(stdout(), cursor::MoveTo(0, 0))
            .expect("failed to move cursor");
        enable_raw_mode()
            .expect("failed to go into raw mode :(");
        Terminal {
            size: size,
            text_buffer: vec![vec!["  ".to_string(); size.y as usize + 1]; size.x as usize + 1], 
            color_buffer: vec![vec![Color::new(0, 0, 0); size.y as usize + 1]; size.x as usize + 1],
        }
    }
    pub fn clear() {
        execute!(stdout(), terminal::Clear(terminal::ClearType::All)).
        expect("failed to clear Terminal");
    }
    pub fn move_cursor(&self, pos: Position) {
        execute!(stdout(), cursor::MoveTo(pos.x,pos.y))
            .expect("failed to move cursor");
    }
    pub fn read_char(&self) -> char {
        loop {
            if poll(Duration::from_millis(0)).expect("") {
                if let Event::Key(KeyEvent {
                    code: KeyCode::Char(c),
                    ..
                }) = event::read().expect("")
                {
                    return c;
                }
                else {
                return ' ';
                }
            } else {
                return ' ';
            }
        }
    }
    pub fn read_key(&self) -> KeyEvent {
    	loop {
            if poll(Duration::from_millis(0)).expect("") {
                if let Event::Key(KeyEvent { code: keycode, modifiers: modifiers }) = event::read().expect("")
                {
                    return KeyEvent {code: keycode, modifiers: modifiers};
                }
                else {
                	return KeyEvent { code: KeyCode::Char(' '), modifiers: KeyModifiers::NONE };
                }
            } else {
            	return KeyEvent { code: KeyCode::Char(' '), modifiers: KeyModifiers::NONE };
            }
        }
    }
    pub fn read_line(&mut self, pos: Position, output_string: &str, color: Color, write_line: bool) -> String {
        execute!(stdout(), cursor::MoveTo(pos.x, pos.y))
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

                        let x = pos.x as usize + line_vec.len() + output_vec.len();
                        let y = pos.y as usize;
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

                        let x = pos.x as usize + line_vec.len() + output_vec.len() - 1;
                        let y = pos.y as usize;
                        
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
    pub fn quit(&self) {
        self.move_cursor(Position::new(0, 0));
        execute!(stdout(), terminal::LeaveAlternateScreen)
            .expect("failed to leave alternatescreen :(");
        stdout().flush()
            .expect("failed to flush stdout :(");
        disable_raw_mode()
            .expect("failed to leave raw mode :(");    
    }
    pub fn render(&mut self) {
        for y in 0..self.size.y {
        	self.move_cursor(Position::new(0, y)); // performance bummer!
            //print!("\n");
            for x in 0..self.size.x {

                if x < self.size.x && y < self.size.y {

                    let text_slice: &str = &*self.text_buffer[x as usize][y as usize];
                    let color = self.color_buffer[x as usize][y as usize].clone();

                    print!("{}", text_slice.truecolor(color.r, color.g, color.b));
                }
            }
        }
        io::stdout().flush().unwrap();
    }

    pub fn write_window(&mut self, window: &Window) {
        for y in 0..window.size.y {
            for x in 0..window.size.x {

                if x + window.pos.x < self.size.x && y + window.pos.y < self.size.y {

                    let text_slice: String = window.text_buffer[x as usize][y as usize].clone();
                    let color = window.color_buffer[x as usize][y as usize].clone();

                    self.text_buffer[(x + window.pos.x) as usize][(y + window.pos.y) as usize] = text_slice;
                    self.color_buffer[(x + window.pos.x) as usize][(y + window.pos.y) as usize] = color;
                }
            }
        }
    }
}