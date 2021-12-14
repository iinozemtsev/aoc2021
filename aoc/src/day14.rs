use std::collections::HashMap;
use std::iter::FromIterator;

pub struct Substitutions {
    table: Vec<u8>,
}

impl Substitutions {
    fn new() -> Self {
        Substitutions {
            table: vec![0; u16::MAX as usize],
        }
    }
    fn get(&self, first: u8, second: u8) -> u8 {
        self.table[Substitutions::index(first, second)]
    }

    fn index(first: u8, second: u8) -> usize {
        (first as usize) * 256 + second as usize
    }

    fn add(&mut self, first: u8, second: u8, substitution: u8) {
        self.table[Substitutions::index(first, second)] = substitution;
    }

    fn add_from_str(&mut self, text: &str) {
        let (from_str, to_str) = text.split_once(" -> ").unwrap();
        if from_str.len() != 2 || to_str.len() != 1 {
            panic!("Bad text: {}", text)
        }
        let mut from_chars = from_str.chars();
        self.add(
            from_chars.next().unwrap() as u8,
            from_chars.next().unwrap() as u8,
            to_str.chars().next().unwrap() as u8,
        );
    }

    fn from_str(text: &str) -> Self {
        text.lines().fold(Substitutions::new(), |mut acc, line| {
            acc.add_from_str(line);
            acc
        })
    }
}

pub struct Sequence {
    elements: Vec<u8>,
}

impl Sequence {
    pub fn new() -> Self {
        Sequence {
            elements: Vec::new(),
        }
    }

    fn from_str(text: &str) -> Self {
        text.chars().map(|c| c as u8).collect()
    }

    fn to_string(&self) -> String {
        self.elements.iter().map(|i| *i as char).collect()
    }

    fn substitute(&self, substitutions: &Substitutions) -> Sequence {
        let len = self.elements.len();
        let mut result: Vec<u8> = Vec::new();
        for i in 0..len - 1 {
            let a = self.elements[i];
            let b = self.elements[i + 1];
            result.push(a);
            result.push(substitutions.get(a, b));
            if i == len - 2 {
                result.push(b);
            }
        }
        let result = Sequence { elements: result };
        return result;
    }

    fn substitute_times(&self, substitutions: &Substitutions, times: u16) -> Sequence {
        if times == 1 {
            self.substitute(substitutions)
        } else {
            self.substitute(substitutions)
                .substitute_times(substitutions, times - 1)
        }
    }
}

impl FromIterator<u8> for Sequence {
    fn from_iter<T: IntoIterator<Item = u8>>(iter: T) -> Self {
        Sequence {
            elements: iter.into_iter().collect(),
        }
    }
}

pub struct FrequencySequence {
    pairs: HashMap<(u8, u8), u64>,
    singles: HashMap<u8, u64>,
}

impl FrequencySequence {
    fn new() -> Self {
        FrequencySequence {
            pairs: HashMap::new(),
            singles: HashMap::new(),
        }
    }
    pub fn from_str(text: &str) -> Self {
        let chars: Vec<u8> = text.chars().map(|c| c as u8).collect();
        let mut result = FrequencySequence::new();
        for i in 0..chars.len() - 1 {
            let a = chars[i];
            let b = chars[i + 1];
            *result.pairs.entry((a, b)).or_default() += 1;
            *result.singles.entry(b).or_default() += 1;
        }
        result
    }

    pub fn substitute(&self, subs: &Substitutions) -> FrequencySequence {
        let mut result = FrequencySequence::new();
        result.singles = self.singles.clone();

        for ((a, b), count) in &self.pairs {
            let c = subs.get(*a, *b);
            *result.pairs.entry((*a, c)).or_default() += count;
            *result.pairs.entry((c, *b)).or_default() += count;
            *result.singles.entry(c).or_default() += count;
        }

        result
    }

    fn substitute_times(&self, substitutions: &Substitutions, times: u16) -> FrequencySequence {
        if times == 1 {
            self.substitute(substitutions)
        } else {
            self.substitute(substitutions)
                .substitute_times(substitutions, times - 1)
        }
    }
}

pub struct Input {
    substitutions: Substitutions,
    sequence: Sequence,
}

impl Input {
    pub fn from_str(text: &str) -> Self {
        let (sequence, substitutions) = text.split_once("\n\n").unwrap();
        Input {
            sequence: Sequence::from_str(sequence),
            substitutions: Substitutions::from_str(substitutions),
        }
    }
}

pub fn part1(input: &Input) -> u32 {
    let result_sequence = input.sequence.substitute_times(&input.substitutions, 10);
    let mut frequency_map: HashMap<u8, u32> = HashMap::new();
    for element in result_sequence.elements {
        *frequency_map.entry(element).or_default() += 1;
    }
    
    let most = frequency_map.values().max().unwrap();
    let least = frequency_map.values().min().unwrap();
    return most - least;
}

pub fn part2(input: &Input) -> u64 {
    let freq_sequence = FrequencySequence::from_str(&input.sequence.to_string())
        .substitute_times(&input.substitutions, 40);

    let most = freq_sequence.singles.values().max().unwrap();
    let least = freq_sequence.singles.values().min().unwrap();
    return most - least;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> Input {
        Input::from_str(include_str!("../resources/day14_sample.txt"))
    }

    fn input() -> Input {
        Input::from_str(include_str!("../resources/day14.txt"))
    }
    #[test]
    fn part1() {
        println!("part1 sample: {}", super::part1(&sample()));
        println!("part1: {}", super::part1(&input()));
    }

    #[test]
    fn part2() {
        println!("part2 sample: {}", super::part2(&sample()));
        println!("part2: {}", super::part2(&input()));
        //        println!("{}", super::part2(&input()));
    }
}
