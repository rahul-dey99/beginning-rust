pub struct Rect {
    pub length: u32,
    pub width: u32
}

impl Rect {
    pub fn area(&self) -> u32 {
        self.length * self.width
    }
    pub fn perimeter(&self) -> u32 {
        2 * (self.length + self.width)
    }
}