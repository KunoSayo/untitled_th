#[derive(Debug, Default)]
pub struct GameInputData {
    pub shoot: u32,
    pub slow: u32,
    pub bomb: u32,
    pub sp: u32,
    pub up: u32,
    pub down: u32,
    pub left: u32,
    pub right: u32,
    pub direction: (i32, i32),
    pub enter: u32,
    pub esc: u32,
}


impl GameInputData {
    pub fn clear(&mut self) {
        *self = Default::default();
    }
}

impl GameInputData {
    pub fn get_move(&self, base_speed: f32) -> (f32, f32) {
        if self.direction.0 == 0 || self.direction.1 == 0 {
            (self.direction.0 as f32 * base_speed, self.direction.1 as f32 * base_speed)
        } else {
            (self.direction.0 as f32 * std::f32::consts::FRAC_1_SQRT_2 * base_speed, self.direction.1 as f32 * base_speed * std::f32::consts::FRAC_1_SQRT_2)
        }
    }
}

