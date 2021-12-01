pub fn get_input() -> Vec<i64> {
    return std::str::from_utf8(include_bytes!("part1.txt"))
        .unwrap()
        .lines()
        .map(|line| line.parse::<i64>().unwrap())
        .collect();
}

pub fn part1(input: &Vec<i64>) -> i64 {
    return input
        .windows(2)
        .map(|w| if w[1] > w[0] { 1 } else { 0 })
        .fold(0, |acc, i| acc + i);
}

pub fn part2(input: &Vec<i64>) -> i64 {
    let sums = input
        .windows(3)
        .map(|w| w[0] + w[1] + w[2])
        .collect::<Vec<i64>>();
    return part1(&sums);
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1() {
        let input = super::get_input();
        let answer = super::part1(&input);
        println!("Answer: {}", answer);
    }

    #[test]
    fn part2() {
        let input = super::get_input();
        let answer = super::part2(&input);
        println!("Answer: {}", answer);
    }

}
