#![allow(dead_code)]
use super::points::*;
use super::sparse_field::*;
use std::collections::HashMap;

struct Dict {
    bits: Vec<bool>,
}

#[derive(Clone)]
struct TrickyImage {
    map: HashMap<Point, bool>,
    background: bool,
}

impl TrickyImage {
    fn new() -> Self {
        TrickyImage {
            map: HashMap::new(),
            background: false,
        }
    }

    fn get(&self, p: &Point) -> bool {
        *self.map.get(p).unwrap_or(&self.background)
    }

    fn from_sparse_field(field: &SparseBoolField) -> Self {
        let (tl, br) = field.bounds();
        let mut res = TrickyImage::new();
        for x in tl.x..=br.x {
            for y in tl.y..=br.y {
                let p = Point { x, y };
                res.map.insert(p, field.get(&p));
            }
        }
        res
    }

    fn to_sparse_field(&self) -> SparseBoolField {
        let mut res = SparseBoolField::new();
        for (k, v) in &self.map {
            if *v {
                res.set(k);
            }
        }
        res
    }

    fn set_count(&self) -> u32 {
        self.map.values().filter(|v| **v).count() as u32
    }

    pub fn bounds(&self) -> (Point, Point) {
        self.map.keys().fold(
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
impl Dict {
    fn parse(text: &str) -> Dict {
        Dict {
            bits: text
                .chars()
                .map(|ch| match ch {
                    '.' => false,
                    '#' => true,
                    _ => panic!("unsupported char: {}", ch),
                })
                .collect(),
        }
    }

    fn get(&self, i: u16) -> bool {
        self.bits[i as usize]
    }

    fn decode(&self, image: &TrickyImage) -> TrickyImage {
        let new_background = if image.background {
            self.get(511)
        } else {
            self.get(0)
        };
        let (tl, br) = image.bounds();
        let mut res = TrickyImage::new();
        res.background = new_background;
        for x in tl.x - 1..=br.x + 1 {
            for y in tl.y - 1..=br.y + 1 {
                let p = Point { x, y };
                res.map.insert(p, self.decode_at(image, &p));
            }
        }
        res
    }

    fn decode_at(&self, image: &TrickyImage, p: &Point) -> bool {
        self.get(Dict::get_window(image, p))
    }
    fn get_window(image: &TrickyImage, p: &Point) -> u16 {
        let mut res = 0;
        for y in p.y - 1..=p.y + 1 {
            for x in p.x - 1..=p.x + 1 {
                res = res << 1;
                if image.get(&Point { x, y }) {
                    res += 1
                }
            }
        }
        res
    }
}

struct Input {
    dict: Dict,
    image: TrickyImage,
}

impl Input {
    fn parse(text: &str) -> Input {
        let (dict_str, field_str) = text.split_once("\n\n").unwrap();
        Input {
            dict: Dict::parse(dict_str),
            image: TrickyImage::from_sparse_field(&SparseBoolField::from_map(field_str, '#')),
        }
    }
}

fn part1(input: &Input) -> u32 {
    let once = input.dict.decode(&input.image);
    let twice = input.dict.decode(&once);
    twice.set_count() as u32
}

fn part2(input: &Input) -> u32 {
    let result = (0..50).fold(input.image.clone(), |acc, _| input.dict.decode(&acc));
    result.set_count() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> Input {
        Input::parse(include_str!("../resources/day20_sample.txt"))
    }

    fn input() -> Input {
        Input::parse(include_str!("../resources/day20.txt"))
    }
    #[test]
    fn dict_parse_test() {
        let dict = Dict::parse("###...###...");
        assert_eq!(dict.get(0), true);
        assert_eq!(dict.get(5), false);
    }

    #[test]
    fn field_parse_test() {
        let map = "#..#.
#....
##..#
..#..
..###
";
        let field = SparseBoolField::from_map(map, '#');
        assert_eq!(format!("{}", field), map);
    }

    #[test]
    fn input_parse_test() {
        let i = input();
        assert_eq!(i.dict.get(30), true);
        assert_eq!(i.image.get(&Point { x: 33, y: 2 }), false);

        let s = sample();
        assert_eq!(s.dict.get(1), false);
        assert_eq!(i.image.get(&Point { x: 2, y: 3 }), true);
    }

    #[test]
    fn get_window_test() {
        let i = sample();
        assert_eq!(Dict::get_window(&i.image, &Point { x: 2, y: 2 }), 34);
        assert_eq!(i.dict.decode_at(&i.image, &Point { x: 2, y: 2 }), true);
    }

    #[test]
    fn part1() {
        assert_eq!(super::part1(&sample()), 35);
        println!("part1: {}", super::part1(&input()));
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(&sample()), 3351);
        println!("part2: {}", super::part2(&input()));
    }
}
