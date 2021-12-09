use std::collections::HashMap;
use std::result::Result;
use std::str::FromStr;

#[derive(PartialEq, Eq, Hash, Debug)]
struct Point {
    x: i64,
    y: i64,
}

impl FromStr for Point {
    type Err = String;

    fn from_str(txt: &str) -> Result<Self, String> {
        let (x_str, y_str) = txt
            .split_once(",")
            .ok_or(format!("Invalid point: {}", txt))?;
        return Ok(Point {
            x: x_str.parse::<i64>().map_err(|e| e.to_string())?,
            y: y_str.parse::<i64>().map_err(|e| e.to_string())?,
        });
    }
}

#[derive(Debug)]
struct Line {
    from: Point,
    to: Point,
}

impl FromStr for Line {
    type Err = String;
    fn from_str(txt: &str) -> Result<Self, String> {
        let (f_str, t_str) = txt
            .split_once(" -> ")
            .ok_or(format!("invalid line: {}", txt))?;
        return Ok(Line {
            from: f_str.parse::<Point>()?,
            to: t_str.parse::<Point>()?,
        });
    }
}

pub struct Input {
    lines: Vec<Line>,
}

impl FromStr for Input {
    type Err = String;

    fn from_str(txt: &str) -> Result<Self, String> {
        let mut result = Input { lines: Vec::new() };
        for line in txt.lines() {
            result.lines.push(line.parse::<Line>()?);
        }
        return Ok(result);
    }
}

fn points(line: &Line) -> Vec<Point> {
    let mut result: Vec<Point> = Vec::new();
    if line.from.x == line.to.x {
        let x = line.from.x;
        let y_from = std::cmp::min(line.from.y, line.to.y);
        let y_to = std::cmp::max(line.from.y, line.to.y);
        for y in y_from..=y_to {
            result.push(Point { x, y });
        }
    } else if line.from.y == line.to.y {
        let y = line.from.y;
        let x_from = std::cmp::min(line.from.x, line.to.x);
        let x_to = std::cmp::max(line.from.x, line.to.x);

        for x in x_from..=x_to {
            result.push(Point { x, y });
        }
    } else {
    }
    return result;
}

fn points2(line: &Line) -> Vec<Point> {
    let mut result: Vec<Point> = Vec::new();
    let y_from = line.from.y;
    let y_to = line.to.y;
    let x_from = line.from.x;
    let x_to = line.to.x;

    let dx = (x_to - x_from).signum();
    let dy = (y_to - y_from).signum();

    let mut x = x_from;
    let mut y = y_from;
    loop {
        result.push(Point { x, y });

        if x == x_to && y == y_to {
            break;
        } else {
            x += dx;
            y += dy;
        }
    }
    return result;
}

pub fn part1(input: &Input) -> usize {
    let mut frequency: HashMap<Point, usize> = HashMap::new();
    for line in &input.lines {
        for point in points(line) {
            let mut count = 0;
            if frequency.contains_key(&point) {
                count = *frequency.get(&point).unwrap();
            }
            frequency.insert(point, count + 1);
        }
    }

    let mut result = 0;
    for (_, count) in &frequency {
        if *count > 1 {
            result += 1;
        }
    }

    return result;
}

pub fn part2(input: &Input) -> usize {
    let mut frequency: HashMap<Point, usize> = HashMap::new();
    for line in &input.lines {
        for point in points2(line) {
            let mut count = 0;
            if frequency.contains_key(&point) {
                count = *frequency.get(&point).unwrap();
            }
            frequency.insert(point, count + 1);
        }
    }

    let mut result = 0;
    for (_, count) in &frequency {
        if *count > 1 {
            result += 1;
        }
    }
    return result;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let sample = include_str!("sample.txt").parse::<super::Input>().unwrap();
        let input = include_str!("input.txt").parse::<super::Input>().unwrap();
        println!("Part1 sample: {}", super::part1(&sample));
        println!("Part1 : {}", super::part1(&input));
        println!("Part2 sample: {}", super::part2(&sample));
        println!("Part2 : {}", super::part2(&input));
    }
}
