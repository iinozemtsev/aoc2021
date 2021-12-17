use std::ops::{Add, AddAssign};
use std::str::FromStr;

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new() -> Self {
        Point { x: 0, y: 0 }
    }
}
#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
pub struct Rect {
    pub top_left: Point,
    pub bottom_right: Point,
}

impl Rect {
    pub fn contains(&self, point: &Point) -> bool {
        point.x <= self.bottom_right.x
            && point.x >= self.top_left.x
            && point.y <= self.top_left.y
            && point.y >= self.bottom_right.y
    }
}
#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
pub struct Offset(pub i32, pub i32);

static TOP: Offset = Offset(0, -1);
static TOP_LEFT: Offset = Offset(-1, -1);
static TOP_RIGHT: Offset = Offset(1, -1);
static BOTTOM: Offset = Offset(0, 1);
static BOTTOM_LEFT: Offset = Offset(-1, 1);
static BOTTOM_RIGHT: Offset = Offset(1, 1);
static LEFT: Offset = Offset(-1, 0);
static RIGHT: Offset = Offset(1, 0);

pub fn neighbors(point: &Point, diagonals: bool, width: u32, height: u32) -> Vec<Point> {
    if diagonals {
        vec![
            TOP_LEFT,
            TOP,
            TOP_RIGHT,
            LEFT,
            RIGHT,
            BOTTOM_LEFT,
            BOTTOM,
            BOTTOM_RIGHT,
        ]
    } else {
        vec![TOP, LEFT, RIGHT, BOTTOM]
    }
    .iter()
    .cloned()
    .map(|o| point + &o)
    .filter(|p| p.x >= 0 && p.x < width as i32 && p.y >= 0 && p.y < height as i32)
    .collect()
}
impl Add<&Offset> for &Point {
    type Output = Point;
    fn add(self, rhs: &Offset) -> Point {
        Point {
            x: self.x + rhs.0,
            y: self.y + rhs.1,
        }
    }
}

impl AddAssign<&Offset> for Point {
    fn add_assign(&mut self, Offset(dx, dy): &Offset) {
        self.x += dx;
        self.y += dy;
    }
}

impl FromStr for Point {
    type Err = String;
    fn from_str(text: &str) -> Result<Point, String> {
        let (x_str, y_str) = text
            .split_once(",")
            .ok_or(format!("Cannot split string {} once by comma", text))?;
        Ok(Point {
            x: x_str.parse::<i32>().map_err(|e| e.to_string())?,
            y: y_str.parse::<i32>().map_err(|e| e.to_string())?,
        })
    }
}
