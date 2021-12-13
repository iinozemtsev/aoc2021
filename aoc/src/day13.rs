use super::points::*;
use super::sparse_field::*;
use std::str::FromStr;

enum Fold {
    Horizontal { x: i32 },
    Vertical { y: i32 },
}

impl Fold {
    fn fold_horizontally(point: &Point, x: i32) -> Option<Point> {
        if point.x < x {
            // as is
            Some(*point)
        } else if point.x == x {
            None
        } else {
            let delta = point.x - x;
            Some(Point {
                x: x - delta,
                y: point.y,
            })
            // translated
        }
    }

    fn fold_vertically(point: &Point, y: i32) -> Option<Point> {
        if point.y < y {
            // as is
            Some(*point)
        } else if point.y == y {
            None
        } else {
            let delta = point.y - y;
            Some(Point {
                y: y - delta,
                x: point.x,
            })
            // translated
        }
    }

    fn fold_point(&self, point: &Point) -> Option<Point> {
        match self {
            Fold::Horizontal { x } => Fold::fold_horizontally(point, *x),
            Fold::Vertical { y } => Fold::fold_vertically(point, *y),
        }
    }
    fn fold(&self, field: &SparseBoolField) -> SparseBoolField {
        let mut result = SparseBoolField::new();

        for point in &field.points {
            let folded = self.fold_point(&point);
            if folded.is_some() {
                result.set(&folded.unwrap());
            }
        }
        result
    }
}
impl FromStr for Fold {
    type Err = String;
    fn from_str(text: &str) -> Result<Fold, String> {
        if !text.starts_with("fold along ") {
            return Err(format!("Invalid fold instruction: {}", text));
        }

        let stripped = text.replace("fold along ", "");

        let (axis, coord_str) = stripped
            .split_once("=")
            .ok_or(format!("Cannot split once by ="))?;

        let coord = coord_str.parse::<i32>().map_err(|e| e.to_string())?;

        match axis {
            "x" => Ok(Fold::Horizontal { x: coord }),
            "y" => Ok(Fold::Vertical { y: coord }),
            _ => Err(format!("Wrong axis: {}", axis)),
        }
    }
}

pub struct Input {
    field: SparseBoolField,
    folds: Vec<Fold>,
}

impl FromStr for Input {
    type Err = String;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let (field_str, folds_str) = text
            .split_once("\n\n")
            .ok_or(format!("Cannot split text by \n\n"))?;
        Ok(Input {
            field: SparseBoolField::from_coords(field_str)?,
            folds: folds_str
                .lines()
                .map(|line| line.parse::<Fold>())
                .collect::<Result<Vec<Fold>, String>>()?,
        })
    }
}

pub fn part1(input: &Input) -> u32 {
    input.folds.first().unwrap().fold(&input.field).points.len() as u32
}

pub fn part2(input: &Input) -> SparseBoolField {
    input.folds.iter().fold(input.field.clone(), |r,e| e.fold(&r))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> Input {
        include_str!("../resources/day13_sample.txt")
            .parse::<Input>()
            .unwrap()
    }

    fn input() -> Input {
        include_str!("../resources/day13.txt")
            .parse::<Input>()
            .unwrap()
    }
    #[test]
    fn parse_test() {
        let input = sample();
        println!("{}", input.field);
        let result = input.folds.iter().fold(input.field, |r, e| e.fold(&r));
        println!("{}", result);
    }

    #[test]
    fn part1() {
        assert_eq!(super::part1(&sample()), 17);
        println!("part1: {}", super::part1(&input()));
    }

    #[test]
    fn part2() {
        println!("{}", super::part2(&input()));
    }

}
