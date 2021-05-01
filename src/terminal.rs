use crossterm::execute;
use crossterm::terminal;
use crossterm::cursor;

use std::{time::Duration};

use std::io::{self, Write, stdout};

use crossterm::event;
use crossterm::event::*;
use crossterm::terminal::enable_raw_mode;
use crossterm::terminal::disable_raw_mode;
use crossterm::style::{SetForegroundColor, Color as CrossColor};
use crossterm::style::ResetColor;

use crate::{Size, Position, Color, Window, Pixel};

pub struct Terminal {
    pub size: Size,
    pub pixel_buffer: Vec<Pixel>,
    pub windows: Vec<Window>,
    pub key_event: KeyEvent,
} impl Terminal {
    pub fn init(size: Size) -> Self {
        // Terminal setup
        execute!(stdout(), terminal::SetSize((size.x + 1) as u16, (size.y + 1) as u16))
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
            pixel_buffer: vec![Pixel::new(" ".to_string(), Color::white()); (size.x * size.y) as usize], 
            windows: Vec::new(),
            key_event: KeyEvent {code: KeyCode::Char(' '), modifiers: KeyModifiers::NONE},
        }
    }
    pub fn clear(&mut self) {
        self.pixel_buffer = vec![Pixel::new(" ".to_string(), Color::white()); (self.size.x * self.size.y) as usize];
    }
    pub fn move_cursor(&self, pos: Position) {
        execute!(stdout(), cursor::MoveTo(pos.x as u16, pos.y as u16))
            .expect("failed to move cursor");
    }

    pub fn update(&mut self) {
        self.key_event = KeyEvent {code: KeyCode::Char(' '), modifiers: KeyModifiers::NONE};
            loop {
                if poll(Duration::from_millis(0)).expect("") {
                    match read().expect("") {
                        Event::Key(event) => {self.key_event = event},
                        _ => (),
                    }
                }
                else {
                    break;
                }
            }
        self.size.x = terminal::size().unwrap().0 as i32;
        self.size.y = terminal::size().unwrap().1 as i32;
        self.pixel_buffer = vec![Pixel::new(" ".to_string(), Color::white()); (self.size.x * self.size.y) as usize];
    }

    pub fn read_line(&mut self, pos: Position, output_string: &str, color: Color, write_line: bool) -> String {
        execute!(stdout(), cursor::MoveTo(pos.x as u16, pos.y as u16))
            .expect("failed to move cursor :(");
        execute!(stdout(), SetForegroundColor(CrossColor::Rgb {r: color.r, g : color.g, b : color.b}))
            .expect("failed to change color");
        print!("{}", output_string);
        io::stdout().flush().unwrap();

        let mut line = String::new();
        let mut old_color = Color::rgb(0, 0, 0);
        'running: loop {

        while let Event::Key(KeyEvent { code, .. }) = event::read().expect("failed to read event :(") {
            match code {
                KeyCode::Char(c) => {
                    if write_line {
                        let output_vec: Vec<char> = output_string.chars().collect();
                        let line_vec: Vec<char> = line.chars().collect();

                        let x = pos.x as usize + line_vec.len() + output_vec.len();
                        let y = pos.y as usize;
                        self.move_cursor(Position::new(x as i32, y as i32));

                        if color.r != old_color.r || color.g != old_color.g || color.b != old_color.b {
                            execute!(stdout(), SetForegroundColor(CrossColor::Rgb {r: color.r, g : color.g, b : color.b})).expect("failed to change color");
                            old_color = color;
                        }

                        print!("{}", c.to_string());
                        io::stdout().flush().unwrap();
                    }
                    line.push(c);
                },
                KeyCode::Enter => {
                    break 'running;
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

        }
        let color = Color::white();
        execute!(stdout(), SetForegroundColor(CrossColor::Rgb {r: color.r, g : color.g, b : color.b}))
            .expect("failed to change color");
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

    pub fn render_fast(&mut self, c: Color) { // 80ms !!!
        // draws every window in windowbuffer front to back to text- and colorbuffer
        for i in 0..self.windows.len() {
            let window = &self.windows[i];
            for y in 0..window.size.y {
                for x in 0..window.size.x {

                    if x + window.pos.x < self.size.x && y + window.pos.y < self.size.y
                    && x + window.pos.x >= 0 && y + window.pos.y >= 0 {

                        let text: String = window.pixel_buffer[(x + (y * window.size.x)) as usize].text.clone();
                        let color = window.pixel_buffer[(x + (y * window.size.x)) as usize].color.clone();

                        self.pixel_buffer[((x + window.pos.x) + ((y + window.pos.y) * self.size.x)) as usize] = Pixel::new(text, color);
                    }
                }
            }
        }
        // draws terminal text- and colorbuffer to real terminal
        execute!(stdout(), SetForegroundColor(CrossColor::Rgb {r: c.r, g : c.g, b : c.b}))
            .expect("failed to change color");
        for y in 0..self.size.y {
            self.move_cursor(Position::new(0, y)); // performance bummer!

            let mut line: String = String::new();

            for x in 0..self.size.x {
                line.push_str(&self.pixel_buffer[(x + (y * self.size.x)) as usize].text);
            }
            print!("{}", line);
        }
        io::stdout().flush().unwrap(); // 0ms
    }

    pub fn render(&mut self) { // 80ms !!!
        // draws every window in windowbuffer front to back to text- and colorbuffer
        for i in 0..self.windows.len() {
            let window = &self.windows[i];
            for y in 0..window.size.y {
                for x in 0..window.size.x {

                    if x + window.pos.x < self.size.x && y + window.pos.y < self.size.y 
                    && x + window.pos.x >= 0 && y + window.pos.y >= 0 {

                        let text: String = window.pixel_buffer[(x + (y * window.size.x)) as usize].text.clone();
                        let color = window.pixel_buffer[(x + (y * window.size.x)) as usize].color.clone();

                        self.pixel_buffer[((x + window.pos.x) + ((y + window.pos.y) * self.size.x)) as usize] = Pixel::new(text, color);
                    }
                }
            }
        }
        // draws terminal text- and colorbuffer to real terminal
        let mut old_color = Color::white();
        for y in 0..self.size.y {
        	self.move_cursor(Position::new(0, y)); // performance bummer!
            for x in 0..self.size.x {

                let text: String = self.pixel_buffer[(x + (y * self.size.x)) as usize].text.clone();
                let color = self.pixel_buffer[(x + (y * self.size.x)) as usize].color.clone();

                if color.r != old_color.r || color.g != old_color.g || color.b != old_color.b {
                    execute!(stdout(), SetForegroundColor(CrossColor::Rgb {r: color.r, g : color.g, b : color.b})).expect("failed to change color");
                       old_color = color;
                }
                print!("{}", text);
            }
        }
        io::stdout().flush().unwrap(); // 0ms
    }

    pub fn render_accurate(&mut self) { // 80ms !!!
        // draws every window in windowbuffer front to back to text- and colorbuffer
        for i in 0..self.windows.len() {
            let window = &self.windows[i];
            for y in 0..window.size.y {
                for x in 0..window.size.x {

                    if x + window.pos.x < self.size.x && y + window.pos.y < self.size.y 
                    && x + window.pos.x >= 0 && y + window.pos.y >= 0 {

                        let text: String = window.pixel_buffer[(x + (y * window.size.x)) as usize].text.clone();
                        let color = window.pixel_buffer[(x + (y * window.size.x)) as usize].color.clone();

                        self.pixel_buffer[((x + window.pos.x) + ((y + window.pos.y) * self.size.x)) as usize] = Pixel::new(text, color);
                    }
                }
            }
        }
        // draws terminal text- and colorbuffer to real terminal
        let mut old_color = Color::white();
        for y in 0..self.size.y {
            for x in 0..self.size.x {

                self.move_cursor(Position::new(x, y)); // performance bummer!

                let text: String = self.pixel_buffer[(x + (y * self.size.x)) as usize].text.clone();
                let color = self.pixel_buffer[(x + (y * self.size.x)) as usize].color.clone();

                if color.r != old_color.r || color.g != old_color.g || color.b != old_color.b {
                    execute!(stdout(), SetForegroundColor(CrossColor::Rgb {r: color.r, g : color.g, b : color.b})).expect("failed to change color");
                    old_color = color;
                }

                print!("{}", text);
            }
        }
        io::stdout().flush().unwrap(); // 0ms
    }

    // possible very bad, slow and memory hungry !!!
    pub fn set_top_window(&mut self, window: &Window) {
        for i in 0..self.windows.len() {
            if self.windows[i].id == window.id {
                // swaps position of desired window to be at the end of windows
                self.windows.push(self.windows[i].clone());
                for j in i..self.windows.len() - 1 {
                    self.windows[j] = self.windows[j + 1].clone();
                }
                self.windows.pop();
            }
        }
    }

    pub fn write_window(&mut self, window: &Window) {
        self.windows.push(window.clone());
    }
    pub fn clear_windows(&mut self) {
        self.windows.clear();
    }
}