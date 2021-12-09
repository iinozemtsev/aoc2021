use super::parse::parse_line_separated;
use arrayvec::ArrayVec;
use std::collections::HashMap;
use std::collections::HashSet;
use std::convert::From;
use std::result::Result;
use std::str::FromStr;
use std::string::ToString;

/// A picture on a 7-digit signal display.
#[derive(PartialEq, Eq, Hash, Clone)]
struct Picture {
    pixels: u8,
}

/// Same as picture, but must contain only one bit.
#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
struct Pixel {
    value: u8,
}

impl Pixel {
    fn from_index(index: u8) -> Pixel {
        Pixel { value: 1 << index }
    }
    fn all_pixels() -> Vec<Pixel> {
        (0u8..7u8).map(|i| Pixel::from_index(i)).collect()
    }
}
impl Picture {
    fn clear(&self, pixel: &Pixel) -> Picture {
        Picture {
            pixels: self.pixels & !pixel.value,
        }
    }

    fn pixels(&self) -> Vec<Pixel> {
        Pixel::all_pixels()
            .into_iter()
            .filter(|i| self.contains(i))
            .collect()
    }

    fn pixel_count(&self) -> u8 {
        self.pixels.count_ones() as u8
    }

    fn contains(&self, pixel: &Pixel) -> bool {
        self.pixels & pixel.value == pixel.value
    }

    fn substitute(&self, table: &HashMap<Pixel, Pixel>) -> Picture {
        Picture {
            pixels: self
                .pixels()
                .into_iter()
                .map(|p| table.get(&p).unwrap())
                .fold(0, |pic, pix| pic | pix.value),
        }
    }
}

impl ToString for Picture {
    fn to_string(&self) -> String {
        self.pixels()
            .into_iter()
            .map(|p| p.to_string())
            .collect::<Vec<String>>()
            .join("")
    }
}

impl ToString for Pixel {
    fn to_string(&self) -> String {
        let mut result = String::new();
        for (i, p) in Pixel::all_pixels().into_iter().enumerate() {
            if *self == p {
                result.push((U8A + i as u8) as char);
            }
        }
        result
    }
}

static U8A: u8 = 'a' as u8;

impl FromStr for Picture {
    type Err = String;
    fn from_str(text: &str) -> Result<Self, String> {
        let mut pixels = 0u8;
        for ch in text.chars() {
            pixels |= 1 << ((ch as u8) - U8A);
        }
        Ok(Picture { pixels })
    }
}

/// Four-digit display.
struct Display {
    all_digits: [Picture; 10],
    display: [Picture; 4],
}

impl FromStr for Display {
    type Err = String;

    fn from_str(text: &str) -> Result<Self, String> {
        let (digits_str, display_str) = text
            .trim()
            .split_once(" | ")
            .ok_or(format!("Cannot split by |: {}", text))?;
        Ok(Display {
            all_digits: digits_str
                .split(" ")
                .map(|s| s.parse::<Picture>().unwrap())
                .collect::<ArrayVec<Picture, 10>>()
                .into_inner()
                .map_err(|v| format!("wrong length: {}", v.len()))?,
            display: display_str
                .split(" ")
                .map(|s| s.parse::<Picture>().unwrap())
                .collect::<ArrayVec<Picture, 4>>()
                .into_inner()
                .map_err(|v| format!("wrong length: {}", v.len()))?,
        })
    }
}
pub struct Input {
    displays: Vec<Display>,
}

impl FromStr for Input {
    type Err = String;

    fn from_str(text: &str) -> Result<Input, String> {
        Ok(Input {
            displays: parse_line_separated(text.trim())?,
        })
    }
}

fn get_true_digits() -> Vec<Picture> {
    vec![
        "abcefg".parse().unwrap(),
        "cf".parse().unwrap(),
        "acdeg".parse().unwrap(),
        "acdfg".parse().unwrap(),
        "bcdf".parse().unwrap(), // 4
        "abdfg".parse().unwrap(),
        "abdefg".parse().unwrap(), // 6
        "acf".parse().unwrap(),
        "abcdefg".parse().unwrap(), // 8
        "abcdfg".parse().unwrap(),
    ]
}

#[derive(Eq, PartialEq, Debug, Clone, Hash)]
struct Segment {
    index: u8,
}

impl From<u8> for Segment {
    fn from(val: u8) -> Self {
        Segment { index: val }
    }
}

fn decode_display(display: &Display) -> u32 {
    let true_pics: HashMap<Picture, u8> = get_true_digits()
        .into_iter()
        .enumerate()
        .map(|(i, v)| (v, i as u8))
        .collect();

    let mut pics: HashMap<Picture, HashSet<Picture>> = HashMap::new();

    for pic in &display.all_digits {
        pics.entry(pic.clone()).or_insert(
            true_pics
                .keys()
                .filter(|p| p.pixel_count() == pic.pixel_count())
                .cloned()
                .collect(),
        );
    }

    let resolved_substitution = try_candidates(pics, HashMap::new()).unwrap();
    let substituted_display = display
        .display
        .iter()
        .map(|d| d.substitute(&resolved_substitution))
        .map(|p| true_pics.get(&p).cloned())
        .fold(0u32, |acc, o| acc * 10 + (o.unwrap() as u32));

    return substituted_display;
}

pub fn part2(input: &Input) -> u32 {
    input.displays.iter().map(|d| decode_display(d)).sum()
}

fn try_candidates(
    pics: HashMap<Picture, HashSet<Picture>>,
    resolved_pixels: HashMap<Pixel, Pixel>,
) -> Option<HashMap<Pixel, Pixel>> {
    let mut updated_pics: HashMap<Picture, HashSet<Picture>> = HashMap::new();
    for (k, v) in pics {
        if k.pixel_count() == 1 && v.len() == 1 && resolved_pixels.contains_key(&k.pixels()[0]) {
            continue;
        }
        let mut new_k = k.clone();
        let mut new_v = v.clone();
        for (from, to) in &resolved_pixels {
            if !new_k.contains(from) {
                // Exclude values containing `to`.
                new_v = new_v.iter().filter(|e| !e.contains(to)).cloned().collect();

                continue;
            }

            // Clear the `from` from a key
            new_k = new_k.clear(from);

            // Clear the `to` from all values.
            new_v = new_v
                .iter()
                .filter(|e| e.contains(to))
                .map(|e| e.clear(to))
                .collect();
        }
        if new_v.is_empty() {
            println!("No candidates for {}, falling back", new_k.to_string());
            return None;
        }
        // if new_k.pixel_count() == 0 {
        //     continue;
        // }
        updated_pics.insert(new_k, new_v);
    }
    let mut updated_pixels = resolved_pixels.clone();
    if updated_pics.is_empty() {
        return Some(updated_pixels);
    }

    // Find the most constrained entry.
    let least_constrained = updated_pics
        .iter()
        .min_by_key(|(k, _)| k.pixel_count())
        .unwrap();

    // First pixel in first candidate.
    let very_first = least_constrained.1.iter().next().unwrap().pixels()[0];

    // Try candidates
    for pixel in least_constrained.0.pixels() {
        let is_valid = is_valid_substitution(&updated_pics, &pixel, &very_first);
        if is_valid {
            updated_pixels.insert(pixel, very_first);
            let result = try_candidates(updated_pics.clone(), updated_pixels.clone());
            if result.is_some() {
                return result;
            } else {
                updated_pixels.remove(&pixel);
            }
        }
    }

    return None;
}

fn is_valid_substitution(
    pics: &HashMap<Picture, HashSet<Picture>>,
    from: &Pixel,
    to: &Pixel,
) -> bool {
    for (k, v) in pics {
        if k.contains(&from) {
            if v.iter().all(|d| !d.contains(&to)) {
                return false;
            }
        } else {
            if v.iter().all(|d| d.contains(&to)) {
                return false;
            }
        }
    }
    return true;
}

pub fn part1(input: &Input) -> u32 {
    let unique_segment_count_digits: HashSet<u8> = vec![2, 3, 4, 7].into_iter().collect();
    let mut result = 0;
    for display in &input.displays {
        for digit in &display.display {
            if unique_segment_count_digits.contains(&digit.pixel_count()) {
                result += 1;
            }
        }
    }
    return result;
}

#[cfg(test)]
mod tests {
    fn sample_input() -> Input {
        include_str!("../resources/day8_sample.txt")
            .parse::<Input>()
            .unwrap()
    }

    fn input() -> Input {
        include_str!("../resources/day8.txt")
            .parse::<Input>()
            .unwrap()
    }

    use super::*;
    #[test]
    fn picture_test() {
        let digit = "agf".parse::<Picture>().unwrap();
        assert_eq!(digit.pixel_count(), 3);
        assert_eq!(digit.to_string(), "afg");
    }

    #[test]
    fn part1() {
        assert_eq!(super::part1(&sample_input()), 26);
        println!("part1: {}", super::part1(&input()));
    }

    #[test]
    fn part2() {
        decode_display(&sample_input().displays[5]);

        println!("part2 sample: {}", super::part2(&sample_input()));
        println!("part2: {}", super::part2(&input()));
    }
}
