use crossterm::{queue, Result,
    cursor::{MoveTo},
    style::{ Print, Color, SetForegroundColor, SetBackgroundColor}};
use std::io::{stdout, Write};

pub trait ScreenElement {
    fn get_width(&self) -> u16;
    fn get_height(&self) -> u16;
    fn get_foreground_color(&self, x: u16, y: u16) -> Color;
    fn get_background_color(&self, x: u16, y: u16) -> Color;
    fn get_part(&self, x: u16, y: u16) -> char;
}

pub struct Screen {
    buffer: Vec<char>,
    foreground_colors: Vec<Color>,
    background_colors: Vec<Color>,
    width: u16,
    height: u16,
}

impl Screen {
    pub fn new(width: u16, height: u16) -> Screen {
        Screen {
            width,
            height,
            buffer: vec![' '; (width*height) as usize],
            foreground_colors: vec![Color::Black; (width*height) as usize],
            background_colors: vec![Color::Black; (width*height) as usize],
        }
    }

    pub fn begin_render(&mut self) {
        let size = (self.width*self.height) as usize;
        self.buffer = vec![' '; size];
        self.foreground_colors = vec![Color::Black; size];
        self.background_colors = vec![Color::Black; size];
    }

    pub fn add_element_at<T: ScreenElement>(&mut self, elem : &T, pos_x:u16, pos_y:u16) {
        for x in 0..elem.get_width() {
            for y in 0..elem.get_height() {
                let index = ((y + pos_y) * self.width + (x +pos_x)) as usize;
                self.buffer[index] = elem.get_part(x, y);
                self.foreground_colors[index] = elem.get_foreground_color(x, y);
                self.background_colors[index] = elem.get_background_color(x, y);
            }
        }
    }

    pub fn add_directly(&mut self, content: char, foreground_color: Color, background_color: Color, pos_x:u16, pos_y:u16, ) {
        let index = ((pos_y) * self.width + (pos_x)) as usize;
        self.buffer[index] = content;
        self.foreground_colors[index] = foreground_color;
        self.background_colors[index] = background_color;
    }

    pub fn add_string_at(&mut self, text: String, foreground_color: Color, background_color: Color, pos_x: u16, pos_y: u16) {
        let mut iter = 0;
        for this_char in text.chars() {
            let index = ((pos_y) * self.width + (pos_x + iter)) as usize;
            self.buffer[index] = this_char;
            self.foreground_colors[index] = foreground_color;
            self.background_colors[index] = background_color;
            iter += 1;
        }      
    }

    pub fn end_render(&self) -> Result<()>{
        for x in 0..self.width {
            for y in 0..self.height {
                let index = (y * self.width + x) as usize;
                let current_char = self.buffer[index];
                let foreground_color = self.foreground_colors[index];
                let background_color = self.background_colors[index];
                queue!(stdout(), MoveTo(x,y), SetBackgroundColor(background_color), SetForegroundColor(foreground_color), Print(current_char))?;
            }
        }
        stdout().flush()?;
        Ok(())
    }
}