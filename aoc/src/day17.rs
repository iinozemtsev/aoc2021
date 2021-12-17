#![allow(dead_code)]
use crate::points::{Offset, Point, Rect};

fn highest_y_if_hits(target: &Rect, velocity: &Offset) -> Option<i32> {
    let mut pos = Point::new();
    let mut vel = *velocity;
    let mut highest_y = pos.y;
    loop {
        if target.contains(&pos) {
            return Some(highest_y);
        }
        pos += &vel;
        highest_y = highest_y.max(pos.y);

        vel.0 -= vel.0.signum();
        vel.1 -= 1;

        // Target is always to the right and to the bottom, so
        // if we below the target or to the right side of the target,
        // we definitely miss.
        if pos.x > target.bottom_right.x || pos.y < target.bottom_right.y {
            return None;
        }
    }
}
// There is likely an analytical solution, but let's do a brute force for now.
fn part1(target: &Rect) -> i32 {
    // The higher vx, the less steps we'll need to reach the target.
    // Ideally, it barely (with a speed of 1)touches a right border of a target.
    // Rough estimate for this x would be:
    //
    // target_right_x = vx * (vx + 1) / 2
    // vx ~= sqrt(2 tlx)
    //
    //
    // On the other hand, if target is sufficiently large, we might want to touch its rightmost
    let min_x = ((target.top_left.x * 2) as f64).sqrt().floor() as i32;
    let max_x = ((target.bottom_right.x * 2) as f64).sqrt().ceil() as i32;

    let min_y = target.top_left.y.abs();
    let max_y = target.bottom_right.y.abs();
    let mut highest_y = 0;
    let mut highest_y_speed = Offset(0, 0);
    for vx in min_x..max_x {
        for vy in min_y..max_y {
            let v = Offset(vx, vy);
            let y = highest_y_if_hits(target, &v).unwrap_or(i32::MIN);
            if y > highest_y {
                highest_y = y;
                highest_y_speed = v;
            }
        }
    }

    println!("highest y speed: {:?}", highest_y_speed);
    return highest_y;
}

fn part2(target: &Rect) -> i32 {
    // This differs from part1 because one-step hits also count.
    // Ideally we'd need to use two ranges for x: smaller one for
    // multi-steps and a bigger one for single-steps. But come on, the
    // problem is small enough that we can wait for a few seconds.

    let min_x = ((target.top_left.x * 2) as f64).sqrt().floor() as i32 - 1;
    let max_x = target.bottom_right.x;

    let min_y = target.bottom_right.y - 1;
    let max_y = target.bottom_right.y.abs() + 1;
    let mut hit_count = 0;
    for vx in min_x..=max_x {
        for vy in min_y..=max_y {
            let v = Offset(vx, vy);
            let y = highest_y_if_hits(target, &v);
            if y.is_some() {
                println!("hit: {:?}", v);
                hit_count += 1;
            }
        }
    }

    return hit_count;
}

#[cfg(test)]
mod tests {
    use super::*;
    use regex::Regex;
    fn parse_input(text: &str) -> Rect {
        let re = Regex::new(r"target area: x=(-?\d+)..(-?\d+), y=(-?\d+)..(-?\d+)").unwrap();
        let coords: Vec<i32> = re
            .captures(text)
            .unwrap()
            .iter()
            .skip(1)
            .map(|v| v.unwrap().as_str().parse::<i32>().unwrap())
            .collect();

        return Rect {
            top_left: Point {
                x: coords[0],
                y: coords[3],
            },
            bottom_right: Point {
                x: coords[1],
                y: coords[2],
            },
        };
    }

    fn input() -> Rect {
        parse_input("target area: x=155..215, y=-132..-72")
    }
    fn sample() -> Rect {
        parse_input("target area: x=20..30, y=-10..-5")
    }
    #[test]
    fn parse_test() {
        println!("{:?}", sample());
    }

    #[test]
    fn part1() {
        assert_eq!(sample().contains(&Point { x: 28, y: -7 }), true);
        assert_eq!(
            highest_y_if_hits(&sample(), &Offset(7, 2)),
            Some(3)
        );
        println!("part1 sample: {}", super::part1(&sample()));
        println!("part1: {}", super::part1(&input()));
    }

    #[test]
    fn part2() {
        println!("part2 sample: {}", super::part2(&sample()));
        println!("part2: {}", super::part2(&input()));
    }
}
