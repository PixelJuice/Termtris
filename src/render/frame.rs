use super::screen::ScreenElement;
use crossterm::style::Color;

pub enum FrameStyle {
    //SingleLine,       //Unused But I don't want to remove it for future use
    DoubleLine,
}

pub struct Frame {
    content: Vec<char>,
    foreground_color: Color,
    background_color: Color,
    width: u16,
    height: u16,
}

impl Frame {
    pub fn new(
        width: u16,
        height: u16,
        style: FrameStyle,
        foreground_color: Color,
        background_color: Color,
    ) -> Frame {
        let mut frame_content = vec![' '; (height * width) as usize];
        let characters = match style {
            FrameStyle::DoubleLine => [' ', '║', '═', '╚', '╝', '╗', '╔'],
            //FrameStyle::SingleLine => [' ','│','─','└','┘','┐','┌' ],     //Unused But I don't want to remove it for future use
        };
        for x in 0..width {
            for y in 0..height {
                if x == 0 || x == width - 1 || y == height - 1 || y == 0 {
                    if x == 0 && y == height - 1 {
                        frame_content[(y * width + x) as usize] = characters[3];
                    } else if x == 0 && y == 0 {
                        frame_content[(y * width + x) as usize] = characters[6];
                    } else if x == width - 1 && y == 0 {
                        frame_content[(y * width + x) as usize] = characters[5];
                    } else if x == width - 1 && y == height - 1 {
                        frame_content[(y * width + x) as usize] = characters[4];
                    } else if y == height - 1 || y == 0 {
                        frame_content[(y * width + x) as usize] = characters[2];
                    } else if x == 0 || x == width - 1 {
                        frame_content[(y * width + x) as usize] = characters[1];
                    }
                } else {
                    frame_content[(y * width + x) as usize] = characters[0];
                }
            }
        }
        Frame {
            content: frame_content,
            foreground_color,
            background_color,
            width,
            height,
        }
    }
}

impl ScreenElement for Frame {
    fn get_width(&self) -> u16 {
        self.width
    }
    fn get_height(&self) -> u16 {
        self.height
    }
    fn get_foreground_color(&self, _x: u16, _y: u16) -> Color {
        self.foreground_color
    }
    fn get_background_color(&self, _x: u16, _y: u16) -> Color {
        self.background_color
    }
    fn get_part(&self, x: u16, y: u16) -> char {
        self.content[(y * self.width + x) as usize]
    }
}
