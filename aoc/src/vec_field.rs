use super::points::*;

pub struct VecField<T: Copy> {
    pub width: u32,
    pub height: u32,
    pub values: Vec<T>,
}

impl<T: Copy> VecField<T> {
    pub fn get(&self, point: &Point) -> T {
        self.values[(point.x + point.y * (self.width as i32)) as usize]
    }
}

pub fn parse_matrix(text: &str) -> VecField<u8> {
    let mut values: Vec<u8> = Vec::new();
    let mut height = 0;
    let mut width = 0;
    for line in text.lines() {
        height += 1;
        width = 0;
        for char in line.chars() {
            values.push(char as u8 - ('0' as u8));
            width += 1;
        }
    }

    VecField {
        height,
        width,
        values,
    }
}
