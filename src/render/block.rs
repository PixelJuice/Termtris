use super::screen::ScreenElement;
use crossterm::style::Color;

pub struct Block {
    content: Vec<char>,
    foreground_colors: Vec<Color>,
    background_colors: Vec<Color>,
    width: u16,
    height: u16,
}

impl Block {
    pub fn new(width: u16, height: u16) -> Block {
        let content = vec![' '; (height * width) as usize];
        let foreground_colors = vec![Color::Black; (height * width) as usize];
        let background_colors = vec![Color::Black; (height * width) as usize];
        Block {
            content,
            width,
            height,
            foreground_colors,
            background_colors,
        }
    }

    pub fn change_content(
        &mut self,
        x: u16,
        y: u16,
        content: char,
        foreground_color: Color,
        background_color: Color,
    ) {
        let index = (y * self.width + x) as usize;
        self.content[index] = content;
        self.foreground_colors[index] = foreground_color;
        self.background_colors[index] = background_color;
    }

    pub fn get_content_by_index(&self, index: usize) -> &char {
        &self.content[index]
    }

    pub fn _get_foreground_color_by_index(&self, index: usize) -> &Color {
        &self.foreground_colors[index]
    }

    pub fn get_background_color_by_index(&self, index: usize) -> &Color {
        &self.background_colors[index]
    }
}

impl ScreenElement for Block {
    fn get_width(&self) -> u16 {
        self.width
    }
    fn get_height(&self) -> u16 {
        self.height
    }
    fn get_foreground_color(&self, x: u16, y: u16) -> Color {
        self.foreground_colors[(y * self.width + x) as usize]
    }
    fn get_background_color(&self, x: u16, y: u16) -> Color {
        self.background_colors[(y * self.width + x) as usize]
    }
    fn get_part(&self, x: u16, y: u16) -> char {
        self.content[(y * self.width + x) as usize]
    }
}
