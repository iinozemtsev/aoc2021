use std::collections::HashSet;
use std::collections::VecDeque;
use std::ops::Add;
use std::str::FromStr;

pub struct Field {
    width: u32,
    height: u32,
    values: Vec<u32>,
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
struct Point(u32, u32);

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
struct Offset(i32, i32);

static TOP: Offset = Offset(0, -1);
static BOTTOM: Offset = Offset(0, 1);
static LEFT: Offset = Offset(-1, 0);
static RIGHT: Offset = Offset(1, 0);

impl Add<&Offset> for &Point {
    type Output = Point;
    fn add(self, rhs: &Offset) -> Point {
        Point(
            (self.0 as i32 + rhs.0) as u32,
            (self.1 as i32 + rhs.1) as u32,
        )
    }
}

impl Field {
    fn is_in_bounds(self: &Field, point: &Point) -> bool {
        point.0 < self.width && point.1 < self.height
    }

    fn height_at(self: &Field, point: &Point) -> u32 {
        self.values[(point.0 + point.1 * self.width) as usize]
    }

    fn neighbors(self: &Field, point: &Point) -> Vec<Point> {
        vec![TOP, BOTTOM, LEFT, RIGHT]
            .iter()
            .cloned()
            .map(|o| point + &o)
            .filter(|p| self.is_in_bounds(p))
            .collect()
    }
}

fn lowest_points(field: &Field) -> Vec<Point> {
    let mut result = Vec::new();
    for x in 0..field.width {
        for y in 0..field.height {
            let point = Point(x, y);
            let point_height = field.height_at(&point);
            if field
                .neighbors(&point)
                .into_iter()
                .all(|neighbor| field.height_at(&neighbor) > point_height)
            {
                result.push(point);
            }
        }
    }
    result
}

fn basin_size(field: &Field, point: &Point) -> u32 {
    // BFS until we find nines.
    let mut basin: HashSet<Point> = HashSet::new();
    let mut queue: VecDeque<Point> = VecDeque::from(vec![*point]);

    while !queue.is_empty() {
        let current = queue.pop_front().unwrap();
        if !basin.insert(current) {
            continue;
        }

        let next: Vec<Point> = field
            .neighbors(&current)
            .into_iter()
            .filter(|n| field.height_at(n) != 9)
            .collect();

        for n in next {
            queue.push_back(n)
        }
    }

    return basin.len() as u32;
}

pub fn part1(field: &Field) -> u32 {
    lowest_points(field)
        .into_iter()
        .map(|p| field.height_at(&p) + 1)
        .sum()
}

pub fn part2(field: &Field) -> u64 {
    let mut basin_sizes = lowest_points(field)
        .into_iter()
        .map(|p| basin_size(field, &p))
        .collect::<Vec<u32>>();

    basin_sizes.sort_by(|a, b| b.cmp(a));
    basin_sizes
        .iter()
        .take(3)
        .fold(1u64, |acc, e| acc * (*e as u64))
}

impl FromStr for Field {
    type Err = String;
    fn from_str(text: &str) -> Result<Field, String> {
        let mut heights: Vec<u32> = Vec::new();
        let mut height = 0;
        let mut width = 0;
        for line in text.lines() {
            height += 1;
            width = 0;
            for char in line.chars() {
                heights.push((char as u8 - ('0' as u8)) as u32);
                width += 1;
            }
        }

        Ok(Field {
            height: height,
            width: width,
            values: heights,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_input() -> Field {
        include_str!("../resources/day9_sample.txt")
            .parse::<Field>()
            .unwrap()
    }

    fn input() -> Field {
        include_str!("../resources/day9.txt")
            .parse::<Field>()
            .unwrap()
    }
    #[test]
    fn part1() {
        assert_eq!(super::part1(&sample_input()), 15);
        println!("part1: {}", super::part1(&input()));
    }
    #[test]
    fn part2() {
        assert_eq!(super::part2(&sample_input()), 1134);
        println!("part2: {}", super::part2(&input()));
    }
}
