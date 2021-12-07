use std::collections::HashMap;

fn evolve(fish_ages: &mut Vec<u8>) {
    let mut new_generation: Vec<u8> = Vec::new();
    for i in fish_ages.iter_mut() {
        if *i == 0 {
            *i = 6;
            new_generation.push(8);
        } else {
            *i = *i - 1;
        }
    }
    fish_ages.append(&mut new_generation);
}

pub fn evolve_days(fish_ages: &mut Vec<u8>, days: u32) {
    for _ in 0..days {
        evolve(fish_ages)
    }
}

fn evolve_map(fish_by_age: &mut HashMap<u8, u64>) {
    let new_generation_count = *fish_by_age.get(&0).unwrap_or(&0);
    for i in 0..8u8 {
        fish_by_age.insert(i, *fish_by_age.get(&(i + 1)).unwrap_or(&0));
    }
    fish_by_age
        .entry(6u8)
        .and_modify(|e| *e += new_generation_count);
    fish_by_age.insert(8, new_generation_count);
}

pub fn evolve_map_days(fish_by_age: &mut HashMap<u8, u64>, days: u32) {
    for _ in 0..days {
        evolve_map(fish_by_age);
    }
}

pub fn fish_count(fish_by_age: &HashMap<u8, u64>) -> u64 {
    fish_by_age.values().sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::parse::parse_comma_separated;

    fn to_map(fish_ages: Vec<u8>) -> HashMap<u8, u64> {
        let mut result = HashMap::new();
        for age in fish_ages {
            result.entry(age).and_modify(|e| *e += 1).or_insert(1);
        }
        return result;
    }

    fn get_sample_input() -> Vec<u8> {
        parse_comma_separated(include_str!("../resources/day6/sample.txt"))
    }

    fn get_input() -> Vec<u8> {
        parse_comma_separated(include_str!("../resources/day6/input.txt"))
    }

    #[test]
    fn part1_sample() {
        let mut fish_ages = to_map(get_sample_input());
        evolve_map_days(&mut fish_ages, 18);
        println!("after 18 days: {}", fish_count(&fish_ages));
        evolve_map_days(&mut fish_ages, 80 - 18);
        println!("after 80 days: {}", fish_count(&fish_ages));
    }

    #[test]
    fn part1() {
        let mut fish_ages = to_map(get_input());
        evolve_map_days(&mut fish_ages, 80);
        println!("part1: {}", fish_count(&fish_ages));
    }

    #[test]
    fn part2_sample() {
        let mut fish_ages = to_map(get_sample_input());
        evolve_map_days(&mut fish_ages, 256);
        println!("part2 sample: {}", fish_count(&fish_ages));
    }

    #[test]
    fn part2() {
        let mut fish_ages = to_map(get_input());
        evolve_map_days(&mut fish_ages, 256);
        println!("part2: {}", fish_count(&fish_ages));
    }
}
