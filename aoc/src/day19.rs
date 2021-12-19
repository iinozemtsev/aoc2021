#![allow(dead_code)]
use nalgebra::Matrix3;
use nalgebra::Vector3;
use std::collections::HashSet;

#[derive(Debug)]
struct Transform {
    offset: Point,
    rotation: Matrix,
}

impl Transform {
    fn rotate(rotation: &Matrix) -> Self {
        Transform {
            offset: vector![0, 0, 0],
            rotation: rotation.clone(),
        }
    }
}
#[derive(Clone, Debug)]
struct Map {
    beacons: Vec<Point>,
}

impl Map {
    fn transform(&self, t: &Transform) -> Map {
        // first rotate, then move.
        Map {
            beacons: self
                .beacons
                .iter()
                .map(|p| (t.rotation * p) + &t.offset)
                .collect(),
        }
    }
}

fn join_maps(maps: &Vec<Map>) -> (Map, Vec<Point>) {
    // for now, don't care for map order.
    let mut result = maps[0].clone();
    let mut remaining = maps.iter().cloned().collect::<Vec<Map>>();
    let mut scanners: Vec<Point> = Vec::new();
    while !remaining.is_empty() {
        let (mut index, mut transform): (usize, Option<Transform>) = (0, None);
        for (i, cand) in remaining.iter().enumerate() {
            if let Some(t) = find_intersection(&result, &cand) {
                index = i;
                scanners.push(t.offset.clone());
                transform = Some(t);
                break;
            }
        }
        if transform.is_none() {
            panic!("No intersections!");
        }
        let appendix = remaining[index].transform(&transform.unwrap());
        result.beacons = result
            .beacons
            .into_iter()
            .chain(appendix.beacons.into_iter())
            .collect::<HashSet<Point>>()
            .into_iter()
            .collect::<Vec<Point>>();
        remaining.remove(index);
    }
    (result, scanners)
}
fn find_intersection(a: &Map, b: &Map) -> Option<Transform> {
    for rotation in ROTATIONS.iter() {
        let rotated = b.transform(&Transform::rotate(rotation));
        if let Some(offset) = find_offset(a, &rotated) {
            return Some(Transform {
                offset,
                rotation: rotation.clone(),
            });
        }
    }
    return None;
}

fn find_offset(a: &Map, b: &Map) -> Option<Point> {
    for i in 0..a.beacons.len() {
        let as_relative_to_i = a.beacons
            .iter()
            .map(|p| p - a.beacons[i])
            .collect::<HashSet<Point>>();
        for j in 0..b.beacons.len() {
            let overlaps = b.beacons
                .iter()
                .map(|p| p - b.beacons[j])
                .map(|p| as_relative_to_i.contains(&p) as usize)
                .sum::<usize>();
            if overlaps >= 12 {
                let offset = a.beacons[i] - b.beacons[j];
                // b = a + offset
                // offset = a - b
                return Some(offset);
            }
        }
    }
    return None;
}

type Point = Vector3<i32>;
type Matrix = Matrix3<i32>;

lazy_static! {
    static ref ROTATIONS: Vec<Matrix> = rotation_matrices();
}

// Simplified integer 90-degree geometry, angles are multiples of pi/2
fn my_cos(angle: i32) -> i32 {
    match angle {
        0 => 1,
        1 => 0,
        2 => -1,
        -1 => 0,
        _ => panic!("wat: {}", angle),
    }
}

// Simplified integer 90-degree geometry, angles are multiples of pi/2
fn my_sin(angle: i32) -> i32 {
    match angle {
        0 => 0,
        1 => 1,
        2 => 0,
        -1 => -1,
        _ => panic!("wat: {}", angle),
    }
}

// Simplified integer 90-degree geometry, angles are multiples of pi/2
fn rotation_matrix(alpha: i32, beta: i32, gamma: i32) -> Matrix {
    let cos_alpha = my_cos(alpha);
    let sin_alpha = my_sin(alpha);
    let cos_beta = my_cos(beta);
    let sin_beta = my_sin(beta);
    let cos_gamma = my_cos(gamma);
    let sin_gamma = my_sin(gamma);
    matrix![
        cos_alpha * cos_beta,
        cos_alpha * sin_beta * sin_gamma - sin_alpha * cos_gamma,
        cos_alpha * sin_beta * cos_gamma + sin_alpha * sin_gamma;
        sin_alpha * cos_beta,
        sin_alpha * sin_beta * sin_gamma + cos_alpha * cos_gamma,
        sin_alpha * sin_beta * cos_gamma - cos_alpha * sin_gamma;
        -sin_beta, cos_beta * sin_gamma, cos_beta * cos_gamma;
    ]
}

fn rotation_matrices() -> Vec<Matrix> {
    let mut result = HashSet::new();
    for alpha in -1..3 {
        for beta in -1..3 {
            for gamma in -1..3 {
                result.insert(rotation_matrix(alpha, beta, gamma));
            }
        }
    }
    return result.into_iter().collect();
}
struct Input {
    maps: Vec<Map>,
}

fn parse_input(text: &str) -> Input {
    let mut result = Input { maps: Vec::new() };

    for part in text.split("\n\n") {
        let mut map = Map {
            beacons: Vec::new(),
        };
        for line in part.lines().skip(1) {
            let coords = line.split(",").collect::<Vec<&str>>();
            map.beacons.push(vector![
                coords[0].parse::<i32>().unwrap(),
                coords[1].parse::<i32>().unwrap(),
                coords[2].parse::<i32>().unwrap()
            ]);
        }
        result.maps.push(map);
    }
    result
}

fn part2(scanners: &Vec<Point>) -> i32 {
    let mut max = i32::MIN;
    for i in 0..(scanners.len() - 1) {
        for j in (i + 1)..scanners.len() {
            let offset = scanners[i] - scanners[j];
            let manh = offset[0].abs() + offset[1].abs() + offset[2].abs();
            max = max.max(manh);
        }
    }
    max
}
#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> Input {
        parse_input(include_str!("../resources/day19_sample.txt"))
    }

    fn input() -> Input {
        parse_input(include_str!("../resources/day19.txt"))
    }

    #[test]
    fn parts() {
        let (map, scanners) = join_maps(&input().maps);
        println!("part1: {}", map.beacons.len());
        println!("part2: {}", super::part2(&scanners));
    }
}
