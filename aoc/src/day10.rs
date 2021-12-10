use std::collections::HashMap;
use std::collections::VecDeque;

lazy_static! {
    static ref BRACKETS: HashMap<char, char> =
        vec!(('(', ')'), ('[', ']'), ('{', '}'), ('<', '>'),)
            .into_iter()
            .collect();
    static ref REVERSE: HashMap<char, char> = BRACKETS
        .iter()
        .map(|(k, v)| (v.clone(), k.clone()))
        .collect();
    static ref CHECK_SCORES: HashMap<char, u32> =
        vec!((')', 3), (']', 57), ('}', 1197), ('>', 25137),)
            .into_iter()
            .collect();
    static ref COMPLETE_SCORES: HashMap<char, u32> = vec!((')', 1), (']', 2), ('}', 3), ('>', 4),)
        .into_iter()
        .collect();
}

fn is_open(ch: char) -> bool {
    BRACKETS.contains_key(&ch)
}

fn is_close(ch: char) -> bool {
    REVERSE.contains_key(&ch)
}

fn get_open(ch: char) -> char {
    *REVERSE.get(&ch).unwrap()
}

fn get_close(ch: char) -> char {
    *BRACKETS.get(&ch).unwrap()
}

fn get_complete_score(ch: char) -> u64 {
    *COMPLETE_SCORES.get(&ch).unwrap() as u64
}

fn get_check_score(ch: char) -> u32 {
    *CHECK_SCORES.get(&ch).unwrap()
}

#[derive(Debug, PartialEq)]
enum LineResult {
    Valid,
    Invalid { ch: char },
    Incomplete { completion: Vec<char> },
}

fn process_line(line: &str) -> LineResult {
    let mut stack: VecDeque<char> = VecDeque::new();
    for ch in line.chars() {
        if is_open(ch) {
            stack.push_front(ch);
        } else if is_close(ch) {
            if stack.is_empty() {
                return LineResult::Invalid { ch };
            }
            let expected_open = get_open(ch);
            let prev_open = stack.pop_front().unwrap();
            if prev_open != expected_open {
                return LineResult::Invalid { ch };
            }
        }
    }

    if stack.is_empty() {
        return LineResult::Valid;
    }

    let mut completion = Vec::new();
    while !stack.is_empty() {
        completion.push(get_close(stack.pop_front().unwrap()));
    }
    return LineResult::Incomplete { completion };
}

fn check_score(line: &str) -> u32 {
    match process_line(line) {
        LineResult::Invalid { ch } => get_check_score(ch),
        _ => 0,
    }
}

fn complete_score(line: &str) -> u64 {
    match process_line(line) {
        LineResult::Incomplete { completion } => completion
            .into_iter()
            .fold(0u64, |acc, ch| acc * 5 + get_complete_score(ch)),
        _ => 0,
    }
}

pub fn part1(text: &str) -> u32 {
    text.lines().map(check_score).sum()
}

pub fn part2(text: &str) -> u64 {
    let mut scores = text
        .lines()
        .map(complete_score)
        .filter(|s| *s > 0)
        .collect::<Vec<u64>>();
    scores.sort();
    return scores[scores.len() / 2];
}

#[cfg(test)]
mod tests {
    use super::*;
    fn sample_input() -> &'static str {
        include_str!("../resources/day10_sample.txt")
    }

    fn input() -> &'static str {
        include_str!("../resources/day10.txt")
    }
    #[test]
    fn brackets_test() {
        assert_eq!(is_open('a'), false);
        assert_eq!(is_close('a'), false);
        assert_eq!(is_close('>'), true);
    }

    #[test]
    fn line_score_test() {
        assert_eq!(check_score("{([(<{}[<>[]}>{[]{[(<()>"), 1197);
    }

    #[test]
    fn part1() {
        assert_eq!(super::part1(sample_input()), 26397);
        println!("part1: {}", super::part1(input()));
    }

    #[test]
    fn part2() {
        assert_eq!(
            super::process_line("[({(<(())[]>[[{[]{<()<>>"),
            LineResult::Incomplete {
                completion: vec!('}', '}', ']', ']', ')', '}', ')', ']')
            }
        );
        assert_eq!(super::part2(sample_input()), 288957);
        println!("part2: {}", super::part2(input()));
    }
}
