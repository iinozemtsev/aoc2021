pub fn is_bit_set(num: usize, position: usize) -> bool {
    num & (1 << position) != 0
}

fn count_bits(input: &Vec<usize>, position: usize) -> usize {
    input
        .iter()
        .fold(0usize, |acc, i| acc + is_bit_set(*i, position) as usize)
}

fn get_most_popular_bit(input: &Vec<usize>, position: usize) -> bool {
    let set_bits = count_bits(input, position);
    let unset_bits = input.len() - set_bits;
    set_bits >= unset_bits
}

pub fn part1(text: &str) -> u32 {
    let input = get_input(text);
    let mut gamma = 0;
    let mut epsilon = 0;
    let mut mask = 1;
    for i in 0..input.length {
        let most_popular_bit = get_most_popular_bit(&input.data, i);
        if most_popular_bit {
            gamma += mask;
        } else {
            epsilon += mask;
        }
        mask <<= 1;
    }
    return gamma * epsilon;
}

pub fn part2(text: &str) -> usize {
    let input = get_input(text);

    let mut oxygen_candidates = input.data.clone();
    let mut position = input.length;
    while oxygen_candidates.len() > 1 {
        position -= 1;
        let most_popular_bit = get_most_popular_bit(&oxygen_candidates, position);
        oxygen_candidates = oxygen_candidates
            .iter()
            .cloned()
            .filter(|&x| is_bit_set(x, position) == most_popular_bit)
            .collect();
    }
    let oxygen = oxygen_candidates[0];

    position = input.length;
    let mut co2_candidates = input.data.clone();
    while co2_candidates.len() > 1 {
        position -= 1;
        let least_popular_bit = !get_most_popular_bit(&co2_candidates, position);
        co2_candidates = co2_candidates
            .iter()
            .cloned()
            .filter(|&x| is_bit_set(x, position) == least_popular_bit)
            .collect();
    }
    let co2 = co2_candidates[0];
    co2 * oxygen
}

struct Input {
    data: Vec<usize>,
    length: usize,
}

fn get_input(text: &str) -> Input {
    Input {
        data: text
            .lines()
            .map(|line| usize::from_str_radix(&line, 2).unwrap())
            .collect(),
        length: text.lines().next().unwrap().len(),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1() {
        println!(
            "part1 sample answer is {}",
            super::part1(include_str!("sample.txt"))
        );
        println!(
            "part1 answer is {}",
            super::part1(include_str!("input.txt"))
        );
    }

    #[test]
    fn part2() {
        println!(
            "part2 sample answer is {}",
            super::part2(include_str!("sample.txt"))
        );
        println!(
            "part2 answer is {}",
            super::part2(include_str!("input.txt"))
        );
    }
}
