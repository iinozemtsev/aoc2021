#![allow(dead_code)]
use std::ops::Range;

use regex::Regex;

#[derive(Debug, Clone)]
struct Cuboid {
    xs: Range<i64>,
    ys: Range<i64>,
    zs: Range<i64>,
    on: bool,
}

impl Cuboid {
    fn from_str(text: &str) -> Self {
        let nums = Regex::new(r"-?\d+")
            .unwrap()
            .captures_iter(text)
            .map(|c| c.get(0).unwrap().as_str().parse::<i64>().unwrap())
            .collect::<Vec<i64>>();
        Cuboid {
            on: text.starts_with("on "),
            xs: nums[0]..(nums[1] + 1),
            ys: nums[2]..(nums[3] + 1),
            zs: nums[4]..(nums[5] + 1),
        }
    }

    fn contains(&self, x: i64, y: i64, z: i64) -> bool {
        self.xs.contains(&x) && self.ys.contains(&y) && self.zs.contains(&z)
    }
}

#[derive(Debug)]
struct Instruction {
    on: bool,
    cuboid: Cuboid,
}

impl Instruction {
    fn from_str(text: &str) -> Self {
        Instruction {
            on: text.starts_with("on "),
            cuboid: Cuboid::from_str(text),
        }
    }
}

#[derive(Debug)]
struct Input {
    cuboids: Vec<Cuboid>,
}

impl Input {
    fn from_str(text: &str) -> Self {
        Input {
            cuboids: text.trim().lines().map(Cuboid::from_str).collect(),
        }
    }
}

fn part1(input: &Input) -> u64 {
    let mut result = 0;
    for x in -50..=50 {
        for y in -50..=50 {
            for z in -50..=50 {
                let mut state = false;
                for cuboid in &input.cuboids {
                    if cuboid.contains(x, y, z) {
                        state = cuboid.on;
                    }
                }
                if state {
                    result += 1;
                }
            }
        }
    }
    result
}

fn part2(input: &Input) -> u64 {
    /*

    main idea:
    1. Fold list of instructions into a list of non-overlapping cuboids, where each cuboid is on.
    2. Sum the volume of all cuboids.

    ## Folding

    Suppose we have a list of non-overlapping cuboids and a next
    "instruction" cuboid. We need to "expand" a list of cuboids:

    1. Each cuboid

     */
    let mut result: Vec<Cuboid> = Vec::new();
    let mut i = 0;
    for next in &input.cuboids {
        println!("instruction: {}, result length: {}", i, result.len());
        if i == 3 {
            println!("at 3: {:?}", result);
        }
        i += 1;
        if result.is_empty() {
            if !next.on {
                continue;
            }
            result.push(next.clone());
        } else {
            if !next.on {
                // off-instruction cuts pieces out of existing rects, splitting
                // those into multiple cubes.
                result = result.into_iter().flat_map(|c| cut_out(c, next)).collect();
            } else {
                // other way around: all existing cuboids cut pieces out of on instruction
                let mut remainder = vec![next.clone()];
                for cut in &result {
                    remainder = remainder
                        .into_iter()
                        .flat_map(|r| cut_out(r, cut))
                        .collect();
                }
                result.extend(remainder);
            }
        }
    }

    // Now, simply count pixels in result.
    result
        .into_iter()
        .map(|c| {
            ((c.xs.end - c.xs.start) * (c.ys.end - c.ys.start) * (c.zs.end - c.zs.start)) as u64
        })
        .sum()
}

fn cut_out(from: Cuboid, cut: &Cuboid) -> Vec<Cuboid> {
    // 1. each axis has three intervals:
    //         x1..a1,a1..a2,a2..x2
    //    middle interval is "special"
    // 2. fit each interval into bounds
    // 3. take carthesian product of intervals.
    // 4. if _all_ intervals are "special", ignore this segment

    // Shortcut for non-overlapping cases:
    if cut.xs.end <= from.xs.start
        || cut.xs.start >= from.xs.end
        || cut.ys.end <= from.ys.start
        || cut.ys.start >= from.ys.end
        || cut.zs.end <= from.zs.start
        || cut.zs.start >= from.zs.end
    {
        return vec![from];
    }
    let mut result: Vec<Cuboid> = Vec::new();
    for (include_x, xrange) in intervals(&from.xs, &cut.xs) {
        for (include_y, yrange) in intervals(&from.ys, &cut.ys) {
            for (include_z, zrange) in intervals(&from.zs, &cut.zs) {
                if !include_x && !include_y && !include_z {
                    continue;
                }
                result.push(Cuboid {
                    xs: xrange.clone(),
                    ys: yrange.clone(),
                    zs: zrange,
                    on: true,
                })
            }
        }
    }

    result
}
fn intervals(input: &Range<i64>, cut: &Range<i64>) -> Vec<(bool, Range<i64>)> {
    let i1 = (true, input.start, input.end.min(cut.start));
    let i2 = (false, input.start.max(cut.start), input.end.min(cut.end));
    let i3 = (true, input.start.max(cut.end), input.end);
    let include_i1 = i1.1 < i1.2 && i1 != i2 && i1 != i3;
    let include_i2 = i2.1 < i2.2 && i2 != i3;
    let include_i3 = i3.1 < i3.2;
    let mut result = Vec::new();
    if include_i1 {
        result.push((i1.0, i1.1..i1.2));
    }
    if include_i2 {
        result.push((i2.0, i2.1..i2.2));
    }
    if include_i3 {
        result.push((i3.0, i3.1..i3.2));
    }
    result
}
#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> Input {
        Input::from_str(include_str!("../resources/day22_sample.txt"))
    }
    fn sample2() -> Input {
        Input::from_str(include_str!("../resources/day22_sample2.txt"))
    }

    fn input() -> Input {
        Input::from_str(include_str!("../resources/day22.txt"))
    }
    #[test]
    fn parse_test() {
        let cuboid = Cuboid::from_str("on x=-20..26,y=-36..17,z=-47..7");
        println!("result: {:?}", cuboid);
    }

    #[test]
    fn part1() {
        assert_eq!(super::part1(&sample()), 590784);
        println!("part1: {}", super::part1(&input()));
    }

    #[test]
    fn part2() {
        let my_input = Input {
            cuboids: vec![
                Cuboid {
                    xs: 0..10,
                    ys: 0..10,
                    zs: 0..10,
                    on: true,
                },
                Cuboid {
                    xs: 1..2,
                    ys: 1..2,
                    zs: 1..2,
                    on: false,
                },
                Cuboid {
                    xs: 9..11,
                    ys: 9..11,
                    zs: 9..11,
                    on: true,
                },
            ],
        };
        assert_eq!(super::part2(&my_input), 1006);
        println!("part2 : {}", super::part2(&input()));
    }

    #[test]
    fn intervals_test() {
        println!("{:?}", intervals(&(0..10), &(3..5)));
        println!("{:?}", intervals(&(0..10), &(50..55)));
        println!("{:?}", intervals(&(0..10), &(-50..55)));
        println!("{:?}", intervals(&(0..10), &(5..15)));
    }
}
