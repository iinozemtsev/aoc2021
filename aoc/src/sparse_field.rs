use super::points::*;
use std::collections::HashSet;
use std::fmt;

#[derive(Clone)]
pub struct SparseBoolField {
    pub points: HashSet<Point>,
}

impl SparseBoolField {
    pub fn new() -> SparseBoolField {
        SparseBoolField {
            points: HashSet::new(),
        }
    }
    pub fn from_coords(text: &str) -> Result<SparseBoolField, String> {
        Ok(text
            .lines()
            .map(|line| line.parse::<Point>())
            .collect::<Result<Vec<Point>, String>>()?
            .into_iter()
            .fold(SparseBoolField::new(), |mut field, point| {
                field.set(&point);
                field
            }))
    }

    fn get(&self, point: &Point) -> bool {
        self.points.contains(point)
    }

    pub fn set(&mut self, point: &Point) -> bool {
        self.points.insert(*point)
    }

    pub fn bounds(&self) -> (Point, Point) {
        self.points.iter().fold(
            (
                Point {
                    x: i32::MAX,
                    y: i32::MAX,
                },
                Point {
                    x: i32::MIN,
                    y: i32::MIN,
                },
            ),
            |(top_left, bottom_right), point| {
                (
                    Point {
                        x: top_left.x.min(point.x),
                        y: top_left.y.min(point.y),
                    },
                    Point {
                        x: bottom_right.x.max(point.x),
                        y: bottom_right.y.max(point.y),
                    },
                )
            },
        )
    }
}

impl fmt::Display for SparseBoolField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (top_left, bottom_right) = self.bounds();

        for y in top_left.y..=bottom_right.y {
            for x in top_left.x..=bottom_right.x {
                write!(f, "{}", if self.get(&Point { x, y }) { "#" } else { "." })?;
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}
