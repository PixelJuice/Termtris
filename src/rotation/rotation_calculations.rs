pub enum Rotation {
    R0,
    R90,
    R180,
    R270,
}

impl Rotation {
    pub fn rotate_clockwise(p_rotation: &Rotation) -> Rotation {
        match p_rotation {
            Rotation::R0 => Rotation::R90,
            Rotation::R90 => Rotation::R180,
            Rotation::R180 => Rotation::R270,
            Rotation::R270 => Rotation::R0,
        }
    }
    pub fn _rotate_counter_clockwise(p_rotation: &Rotation) -> Rotation {
        match p_rotation {
            Rotation::R0 => Rotation::R270,
            Rotation::R90 => Rotation::R0,
            Rotation::R180 => Rotation::R90,
            Rotation::R270 => Rotation::R180,
        }
    }

    pub fn rotate(p_pos_x: i16, p_pos_y: i16, p_rotation: &Rotation) -> i16 {
        match p_rotation {
            Rotation::R0 => p_pos_y * 4 + p_pos_x,
            Rotation::R90 => 12 + p_pos_y - (4 * p_pos_x),
            Rotation::R180 => 15 - (p_pos_y * 4) - p_pos_x,
            Rotation::R270 => 3 - p_pos_y + (4 * p_pos_x),
        }
    }
}
