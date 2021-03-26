use crossterm::style::Color;
use super::screen::ScreenElement;

pub struct Text {
    buffer: String,
    width: u16,
    height: u16,
    foreground_color: Color,
    background_color: Color,
}

impl Text {
    pub fn new(text: String, foreground_color: Color, background_color: Color) -> Text{
        Text {
            width: text.len() as u16,
            buffer: text,
            height: 1,
            foreground_color,
            background_color,
        }
    }
}

impl ScreenElement for Text {
    fn get_width(&self) -> u16 { self.width }
    fn get_height(&self) -> u16 { self.height }
    fn get_foreground_color(&self, _x: u16, _y: u16) -> Color { self.foreground_color }
    fn get_background_color(&self, _x: u16, _y: u16) -> Color { self.background_color }
    fn get_part(&self, x: u16, y: u16) -> char { 
        self.buffer.as_bytes()[(y*self.width+x) as usize] as char
     }
}