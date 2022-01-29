pub const UP_VALUE: u8 = 0b1000;
pub const DOWN_VALUE: u8 = 0b0100;
pub const LEFT_VALUE: u8 = 0b0010;
pub const RIGHT_VALUE: u8 = 0b0001;

#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub enum Direction {
    Left = LEFT_VALUE,
    Right = RIGHT_VALUE,
    Up = UP_VALUE,
    Down = DOWN_VALUE,
}

#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub enum Bounding {
    No = 0,
    Left = LEFT_VALUE,
    Right = RIGHT_VALUE,
    Up = UP_VALUE,
    Down = DOWN_VALUE,
    LeftUp = LEFT_VALUE | UP_VALUE,
    RightUp = RIGHT_VALUE | UP_VALUE,
    LeftDown = LEFT_VALUE | DOWN_VALUE,
    RightDown = RIGHT_VALUE | DOWN_VALUE,
    UpDown = UP_VALUE | DOWN_VALUE,
    LeftRight = LEFT_VALUE | RIGHT_VALUE,
    UpPass = LEFT_VALUE | RIGHT_VALUE | DOWN_VALUE,
    LeftPass = UP_VALUE | RIGHT_VALUE | DOWN_VALUE,
    RightPass = UP_VALUE | LEFT_VALUE | DOWN_VALUE,
    DownPass = UP_VALUE | LEFT_VALUE | RIGHT_VALUE,
    AllBlock = 0b1111,
}

impl Bounding {
    /// Check the to direction bound, true if has
    pub fn check_bound(self, to: Direction) -> bool {
        (to as u8 & self as u8) != 0
    }
}
