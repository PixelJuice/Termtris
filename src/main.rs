use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{poll, read, Event, KeyCode},
    execute,
    style::{Color, Print},
    terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen, SetSize, SetTitle},
    Result,
};
use rand::Rng;
use std::io::stdout;
use std::{thread, time};

mod render;
mod rotation;
use render::Block;
use render::Frame;
use render::FrameStyle;
use render::Screen;
use render::Text;
use rotation::Rotation;

struct Input {
    right: bool,
    left: bool,
    down: bool,
    rotate: bool,
}

impl Input {
    fn new() -> Input {
        Input {
            right: false,
            left: false,
            down: false,
            rotate: false,
        }
    }
}

struct ScreenSetting {
    field_width: i16,
    field_height: i16,
    screen_width: i16,
    screen_height: i16,
}

impl ScreenSetting {
    fn new(
        field_width: i16,
        field_height: i16,
        screen_width: i16,
        screen_height: i16,
    ) -> ScreenSetting {
        ScreenSetting {
            field_width,
            field_height,
            screen_height,
            screen_width,
        }
    }
}

struct TetrisShape {
    current_piece: i16,
    current_rotation: Rotation,
    current_color: Color,
    current_x: i16,
    current_y: i16,
}

impl TetrisShape {
    fn new(current_x: i16, current_y: i16) -> TetrisShape {
        let random_value = rand::thread_rng().gen_range(0..7);
        let piece_colors = [
            Color::Cyan,
            Color::Green,
            Color::Blue,
            Color::Yellow,
            Color::Magenta,
            Color::Red,
            Color::Green,
        ];
        TetrisShape {
            current_piece: random_value,
            current_color: piece_colors[random_value as usize],
            current_rotation: Rotation::R0,
            current_x,
            current_y,
        }
    }
}

fn main() -> Result<()> {
    //Startup=================================================================
    let tetromino = build_tetromino();
    let screen_settings = ScreenSetting::new(12, 18, 34, 20);
    execute!(
        stdout(),
        EnterAlternateScreen,
        SetSize(
            screen_settings.screen_width as u16,
            screen_settings.screen_height as u16
        ),
        SetTitle("Tetris"),
        Clear(ClearType::All),
        Hide,
    )?;

    //Run=====================================================================

    intro()?;
    run_game(&tetromino, &screen_settings)?;

    //Exit=================================================================

    quit()?;
    Ok(())
}

fn quit() -> Result<()> {
    execute!(stdout(), LeaveAlternateScreen, Show,)?;
    Ok(())
}

fn run_game(tetromino: &Vec<String>, screen_settings: &ScreenSetting) -> Result<()> {
    let mut field = create_initial_field(&screen_settings);
    let mut piece = TetrisShape::new(screen_settings.field_width / 2, 0);
    let mut input_state = Input::new();
    let duration = time::Duration::from_millis(50);
    let mut ticks = 1;
    let mut handicap = 20;
    let mut lines: Vec<i16> = Vec::new();
    let mut points = 0;
    let mut game_over = false;
    let mut pieces_spawned = 0;
    let score = Frame::new(19, 3, FrameStyle::DoubleLine, Color::White, Color::Black);
    let mut screen = Screen::new(
        screen_settings.screen_width as u16,
        screen_settings.screen_height as u16,
    );
    let score_title = Text::new(String::from(" SCORE "), Color::Cyan, Color::Black);
    while !game_over {
        set_input(&mut input_state, &mut game_over)?;
        move_shape(
            &mut input_state,
            &mut piece,
            &tetromino,
            &screen_settings,
            &field,
        );
        if ticks % handicap == 0 {
            piece = move_down(
                piece,
                &tetromino,
                &screen_settings,
                &mut field,
                &mut lines,
                &mut game_over,
                &mut handicap,
                &mut pieces_spawned,
            );
        }
        screen.begin_render();
        screen.add_element_at(&field, 2, 2);
        screen.add_element_at(&score, 15, 2);
        screen.add_element_at(&score_title, 21, 2);
        let score = format!("{:0>11}", points);
        screen.add_string_at(score, Color::DarkBlue, Color::Black, 19, 3);
        render_current_piece(&tetromino, &mut screen, &piece);
        screen.end_render()?;
        ticks += 1;
        thread::sleep(duration);
        points += add_points_to_score(&mut lines, &screen_settings, &mut field);
    }
    Ok(())
}

fn intro() -> Result<()> {
    execute!(stdout(), MoveTo(0, 5), Print("Instructions:\nUse arrow keys to move, up to rotate.\n\nWhen you are done ESC to quit\n\nPress any key to continue"))?;
    loop {
        match read()? {
            Event::Key(_event) => return Ok(()),
            Event::Mouse(_event) => (),
            Event::Resize(_width, _height) => (),
        }
    }
}

fn render_current_piece(tetromino: &Vec<String>, screen: &mut Screen, piece: &TetrisShape) {
    for px in 0..4 {
        for py in 0..4 {
            let char_as_bytes: u8 = tetromino[piece.current_piece as usize].as_bytes()
                [Rotation::rotate(px, py, &piece.current_rotation) as usize];
            if char_as_bytes as char == 'X' {
                let this_y = (piece.current_y + py + 2) as u16;
                let this_x = (piece.current_x + px + 2) as u16;
                screen.add_directly('0', Color::Grey, piece.current_color, this_x, this_y)
            }
        }
    }
}

fn add_points_to_score(
    lines: &mut Vec<i16>,
    screen_settings: &ScreenSetting,
    field: &mut Block,
) -> u16 {
    if lines.len() > 0 {
        let score_duration = time::Duration::from_millis(400);
        thread::sleep(score_duration);
        for elem in lines.to_owned() {
            for px in 1..screen_settings.field_width - 1 {
                field.change_content(px as u16, elem as u16, ' ', Color::Black, Color::Black);
                for py in (1..elem + 1).rev() {
                    let index = ((py - 1) * screen_settings.field_width + px) as usize;
                    let new_char = *field.get_content_by_index(index);
                    let new_color = *field.get_background_color_by_index(index);
                    field.change_content(px as u16, py as u16, new_char, Color::Grey, new_color);
                }
                field.change_content(px as u16, 0, ' ', Color::Black, Color::Black);
            }
        }
        let line_num = lines.len() as u16;
        lines.clear();
        return line_num * 100 + (line_num * 50);
    }
    0
}

fn set_input(input_state: &mut Input, game_over: &mut bool) -> Result<()> {
    input_state.down = false;
    input_state.left = false;
    input_state.right = false;
    input_state.rotate = false;
    if poll(time::Duration::from_millis(0))? {
        match read()? {
            Event::Key(input_event) => {
                if input_event.code == KeyCode::Left {
                    input_state.left = true;
                    input_state.right = false;
                }
                if input_event.code == KeyCode::Right {
                    input_state.right = true;
                    input_state.left = false;
                }
                if input_event.code == KeyCode::Down {
                    input_state.down = true
                }
                if input_event.code == KeyCode::Up {
                    input_state.rotate = true
                }
                if input_event.code == KeyCode::Esc {
                    *game_over = true
                }
            }
            Event::Mouse(_event) => (),
            Event::Resize(_width, _height) => (),
        }
    }
    Ok(())
}

fn move_down(
    mut p_shape: TetrisShape,
    p_tetromino: &Vec<String>,
    p_screen: &ScreenSetting,
    p_field: &mut Block,
    p_lines: &mut Vec<i16>,
    game_over: &mut bool,
    difficulty_handicap: &mut u16,
    pieces_spawned: &mut u16,
) -> TetrisShape {
    if does_piece_fit(
        p_tetromino,
        p_shape.current_piece,
        &p_shape.current_rotation,
        p_shape.current_x,
        p_shape.current_y + 1,
        p_screen,
        p_field,
    ) {
        p_shape.current_y += 1;
        return p_shape;
    } else {
        lock_piece(&p_shape, p_tetromino, p_field);

        test_full_lines(&p_shape, p_screen, p_field, p_lines);

        //new piece and gameover
        p_shape = TetrisShape::new(p_screen.field_width / 2, 0);

        *pieces_spawned += 1;
        if *pieces_spawned % 10 == 0 {
            if *difficulty_handicap > 5 {
                *difficulty_handicap -= 1;
            }
        }
        *game_over = !does_piece_fit(
            p_tetromino,
            p_shape.current_piece,
            &p_shape.current_rotation,
            p_shape.current_x,
            p_shape.current_y,
            p_screen,
            p_field,
        );
    }
    p_shape
}

fn lock_piece(p_shape: &TetrisShape, p_tetromino: &Vec<String>, p_field: &mut Block) {
    for px in 0..4 {
        for py in 0..4 {
            let char_as_bytes: u8 = p_tetromino[p_shape.current_piece as usize].as_bytes()
                [Rotation::rotate(px, py, &p_shape.current_rotation) as usize];
            if char_as_bytes as char == 'X' {
                p_field.change_content(
                    (p_shape.current_x + px) as u16,
                    (p_shape.current_y + py) as u16,
                    '0',
                    Color::Grey,
                    p_shape.current_color,
                );
            }
        }
    }
}

fn test_full_lines(
    p_shape: &TetrisShape,
    p_screen: &ScreenSetting,
    p_field: &mut Block,
    p_lines: &mut Vec<i16>,
) {
    for py in 0..4 {
        if p_shape.current_y + py < p_screen.field_height - 1 {
            let mut line = true;
            for px in 1..p_screen.field_width - 1 {
                let index = ((p_shape.current_y + py) * p_screen.field_width + px) as usize;
                line &= *p_field.get_content_by_index(index) != ' ';
            }
            if line {
                for px in 1..p_screen.field_width - 1 {
                    p_field.change_content(
                        px as u16,
                        (py + p_shape.current_y) as u16,
                        '=',
                        Color::Yellow,
                        Color::Black,
                    );
                }
                p_lines.push(p_shape.current_y + py);
            }
        }
    }
}

fn move_shape(
    input_state: &mut Input,
    p_state: &mut TetrisShape,
    p_tetromino: &Vec<String>,
    p_screen: &ScreenSetting,
    p_field: &Block,
) {
    if input_state.left {
        if does_piece_fit(
            p_tetromino,
            p_state.current_piece,
            &p_state.current_rotation,
            p_state.current_x - 1,
            p_state.current_y,
            p_screen,
            p_field,
        ) {
            p_state.current_x -= 1;
        }
    }
    if input_state.right {
        if does_piece_fit(
            p_tetromino,
            p_state.current_piece,
            &p_state.current_rotation,
            p_state.current_x + 1,
            p_state.current_y,
            p_screen,
            p_field,
        ) {
            p_state.current_x += 1;
        }
    }
    if input_state.down {
        if does_piece_fit(
            p_tetromino,
            p_state.current_piece,
            &p_state.current_rotation,
            p_state.current_x,
            p_state.current_y + 1,
            p_screen,
            p_field,
        ) {
            p_state.current_y += 1;
        }
    }

    if input_state.rotate {
        let new_rotation = Rotation::rotate_clockwise(&p_state.current_rotation);
        if does_piece_fit(
            p_tetromino,
            p_state.current_piece,
            &new_rotation,
            p_state.current_x,
            p_state.current_y,
            p_screen,
            p_field,
        ) {
            p_state.current_rotation = new_rotation;
        }
    }
}

fn does_piece_fit(
    p_tetromino: &Vec<String>,
    p_tetrino: i16,
    p_rotation: &Rotation,
    p_pos_x: i16,
    p_pos_y: i16,
    p_screen: &ScreenSetting,
    p_field: &Block,
) -> bool {
    for px in 0..4 {
        for py in 0..4 {
            let piece_index = Rotation::rotate(px, py, &p_rotation);
            let field_index = ((p_pos_y + py) * p_screen.field_width + (p_pos_x + px)) as usize;
            if (p_pos_x + px) < p_screen.field_width {
                if (p_pos_y + py) < p_screen.field_height {
                    if p_tetromino[p_tetrino as usize].as_bytes()[piece_index as usize] as char
                        == 'X'
                        && *p_field.get_content_by_index(field_index) != ' '
                    {
                        return false;
                    }
                }
            }
        }
    }
    true
}

fn create_initial_field(screen_settings: &ScreenSetting) -> Block {
    let characters = [' ', '║', '═', '╚', '╝', '╗', '╔'];
    let mut field = Block::new(
        screen_settings.field_width as u16,
        screen_settings.field_height as u16,
    );
    for x in 0..screen_settings.field_width {
        for y in 0..screen_settings.field_height {
            if x == 0
                || x == screen_settings.field_width - 1
                || y == screen_settings.field_height - 1
            {
                if x == 0 && y == screen_settings.field_height - 1 {
                    field.change_content(
                        x as u16,
                        y as u16,
                        characters[3],
                        Color::White,
                        Color::Black,
                    );
                } else if x == screen_settings.field_width - 1
                    && y == screen_settings.field_height - 1
                {
                    field.change_content(
                        x as u16,
                        y as u16,
                        characters[4],
                        Color::White,
                        Color::Black,
                    );
                } else if y == screen_settings.field_height - 1 {
                    field.change_content(
                        x as u16,
                        y as u16,
                        characters[2],
                        Color::White,
                        Color::Black,
                    );
                } else if x == 0 || x == screen_settings.field_width - 1 {
                    field.change_content(
                        x as u16,
                        y as u16,
                        characters[1],
                        Color::White,
                        Color::Black,
                    );
                }
            } else {
                field.change_content(
                    x as u16,
                    y as u16,
                    characters[0],
                    Color::White,
                    Color::Black,
                );
            }
        }
    }
    field
}

fn build_tetromino() -> Vec<String> {
    //This could be more optimal left for readability
    let mut tetromino = vec![];
    let mut shape = String::from("");
    shape.push_str("..X.");
    shape.push_str("..X.");
    shape.push_str("..X.");
    shape.push_str("..X.");
    tetromino.push(shape);
    let mut shape = String::from("");
    shape.push_str("..X.");
    shape.push_str(".XX.");
    shape.push_str(".X..");
    shape.push_str("....");
    tetromino.push(shape);
    let mut shape = String::from("");
    shape.push_str(".X..");
    shape.push_str(".XX.");
    shape.push_str("..X.");
    shape.push_str("....");
    tetromino.push(shape);
    let mut shape = String::from("");
    shape.push_str("....");
    shape.push_str(".XX.");
    shape.push_str("..X.");
    shape.push_str("..X.");
    tetromino.push(shape);
    let mut shape = String::from("");
    shape.push_str("....");
    shape.push_str(".XX.");
    shape.push_str(".X..");
    shape.push_str(".X..");
    tetromino.push(shape);
    let mut shape = String::from("");
    shape.push_str("....");
    shape.push_str(".XX.");
    shape.push_str(".XX.");
    shape.push_str("....");
    tetromino.push(shape);
    let mut shape = String::from("");
    shape.push_str("..X.");
    shape.push_str(".XX.");
    shape.push_str("..X.");
    shape.push_str("....");
    tetromino.push(shape);
    tetromino
}
