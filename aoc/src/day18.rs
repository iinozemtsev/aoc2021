#![allow(dead_code)]
use std::iter::Peekable;
use std::ops::Add;
use std::str::CharIndices;
use std::string::ToString;

#[derive(Clone, Eq, PartialEq, Debug)]
struct PairNum {
    left: Box<Elem>,
    right: Box<Elem>,
}

impl PairNum {
    fn magnitude(&self) -> u64 {
        self.left.magnitude() * 3 + self.right.magnitude() * 2
    }
    fn new(left: Elem, right: Elem) -> Self {
        PairNum {
            left: Box::new(left),
            right: Box::new(right),
        }
    }

    fn reduce(&mut self) {
        if self.try_explode() {
            self.reduce();
        } else if self.try_split() {
            self.reduce();
        }
    }

    // Explodes if applicable and returns true, returns false otherwise.
    fn try_explode(&mut self) -> bool {
        if let Explode::Explosion {
            replacement,
            left_overflow: _,
            right_overflow,
        } = self.left.try_explode(1)
        {
            *self.left = replacement;
            if let Some(right_num) = right_overflow {
                if let Some(new_right) = self.right.add_right_num(right_num) {
                    *self.right = new_right;
                }
            }
            return true;
        }

        if let Explode::Explosion {
            replacement,
            left_overflow,
            right_overflow: _,
        } = self.right.try_explode(1)
        {
            *self.right = replacement;
            if let Some(left_num) = left_overflow {
                if let Some(new_left) = self.left.add_left_num(left_num) {
                    *self.left = new_left;
                }
            }
            return true;
        }
        return false;
    }

    // Splits if applicable and returns true.
    fn try_split(&mut self) -> bool {
        if let Some(new_left) = self.left.try_split() {
            *self.left = new_left;
            return true;
        } else if let Some(new_right) = self.right.try_split() {
            *self.right = new_right;
            return true;
        }
        return false;
    }

    fn parse(text: &str) -> Self {
        let mut chars = text.char_indices().peekable();
        PairNum::consume_pair(&mut chars)
    }

    fn consume_pair(chars: &mut Peekable<CharIndices>) -> Self {
        PairNum::consume('[', chars);
        let left = PairNum::consume_element(chars);
        PairNum::consume(',', chars);
        let right = PairNum::consume_element(chars);
        PairNum::consume(']', chars);

        PairNum::new(left, right)
    }

    fn consume_element(chars: &mut Peekable<CharIndices>) -> Elem {
        let (_, next) = chars.peek().unwrap();
        if *next == '[' {
            Elem::Pair(PairNum::consume_pair(chars))
        } else {
            Elem::Number(PairNum::consume_num(chars))
        }
    }

    fn consume(expected: char, chars: &mut Peekable<CharIndices>) {
        let (idx, actual) = chars.next().unwrap();
        if actual != expected {
            panic!(
                "Expected '{}' but got '{} at position {}",
                expected, actual, idx
            );
        }
    }

    fn consume_num(chars: &mut Peekable<CharIndices>) -> u64 {
        let mut num_str = String::new();
        loop {
            let (_, peek) = chars.peek().unwrap();
            if peek.is_ascii_digit() {
                num_str.push(chars.next().unwrap().1);
            } else {
                break;
            }
        }
        num_str.parse::<u64>().unwrap()
    }
}

impl ToString for PairNum {
    fn to_string(&self) -> String {
        format!(
            "[{},{}]",
            self.left.as_ref().to_string(),
            self.right.as_ref().to_string()
        )
    }
}
#[derive(Clone, Eq, PartialEq, Debug)]
enum Elem {
    Number(u64),
    Pair(PairNum),
}

impl Elem {
    fn magnitude(&self) -> u64 {
        match self {
            Elem::Number(n) => *n,
            Elem::Pair(p) => p.magnitude(),
        }
    }
    fn try_split(&self) -> Option<Elem> {
        match self {
            Elem::Number(v) => {
                if *v > 9 {
                    let a = *v / 2;
                    let b = *v - a;
                    Some(Elem::Pair(PairNum::new(Elem::Number(a), Elem::Number(b))))
                } else {
                    None
                }
            }
            Elem::Pair(p) => {
                let mut copy = p.clone();
                if copy.try_split() {
                    Some(Elem::Pair(copy))
                } else {
                    None
                }
            }
        }
    }

    fn add_right_num(&self, num: u64) -> Option<Elem> {
        match self {
            Elem::Number(v) => Some(Elem::Number(v + num)),
            Elem::Pair(p) => {
                let maybe_new_left = p.left.add_right_num(num);
                match maybe_new_left {
                    None => None,
                    Some(new_left) => {
                        Some(Elem::Pair(PairNum::new(new_left, p.right.as_ref().clone())))
                    }
                }
            }
        }
    }

    fn add_left_num(&self, num: u64) -> Option<Elem> {
        match self {
            Elem::Number(v) => Some(Elem::Number(v + num)),
            Elem::Pair(p) => {
                let maybe_new_right = p.right.add_left_num(num);
                match maybe_new_right {
                    None => None,
                    Some(new_right) => {
                        Some(Elem::Pair(PairNum::new(p.left.as_ref().clone(), new_right)))
                    }
                }
            }
        }
    }
    fn try_explode(&self, depth: u8) -> Explode {
        match self {
            // Numbers never explode.
            Elem::Number(_) => Explode::None,
            Elem::Pair(p) => {
                if depth < 4 {
                    if let Explode::Explosion {
                        replacement,
                        left_overflow,
                        right_overflow,
                    } = p.left.try_explode(depth + 1)
                    {
                        if let Some(v) = right_overflow {
                            // Has right overflow
                            if let Some(new_right) = p.right.add_right_num(v) {
                                // consumed right overflow.
                                return Explode::Explosion {
                                    replacement: Elem::Pair(PairNum::new(replacement, new_right)),
                                    left_overflow,
                                    right_overflow: None,
                                };
                            }
                        }
                        return Explode::Explosion {
                            replacement: Elem::Pair(PairNum::new(
                                replacement,
                                p.right.as_ref().clone(),
                            )),
                            left_overflow,
                            right_overflow,
                        };
                    } else if let Explode::Explosion {
                        replacement,
                        left_overflow,
                        right_overflow,
                    } = p.right.try_explode(depth + 1)
                    {
                        if let Some(v) = left_overflow {
                            // Has left overflow
                            if let Some(new_left) = p.left.add_left_num(v) {
                                // consumed left overflow.
                                return Explode::Explosion {
                                    replacement: Elem::Pair(PairNum::new(new_left, replacement)),
                                    left_overflow: None,
                                    right_overflow,
                                };
                            }
                        }
                        return Explode::Explosion {
                            replacement: Elem::Pair(PairNum::new(
                                p.left.as_ref().clone(),
                                replacement,
                            )),
                            left_overflow,
                            right_overflow,
                        };
                    } else {
                        return Explode::None;
                    }
                } else {
                    // depth == 4
                    // both arms must be nums

                    if let (Elem::Number(left_num), Elem::Number(right_num)) =
                        (p.left.as_ref(), p.right.as_ref())
                    {
                        return Explode::Explosion {
                            replacement: Elem::Number(0),
                            left_overflow: Some(*left_num),
                            right_overflow: Some(*right_num),
                        };
                    } else {
                        panic!("Pair contains not only nums: {}", p.to_string())
                    }
                }
            }
        }
    }
}

enum Explode {
    None,
    Explosion {
        replacement: Elem,
        left_overflow: Option<u64>,
        right_overflow: Option<u64>,
    },
}

impl ToString for Elem {
    fn to_string(&self) -> String {
        match self {
            Self::Number(v) => v.to_string(),
            Self::Pair(v) => v.to_string(),
        }
    }
}
impl Add<&PairNum> for &PairNum {
    type Output = PairNum;

    fn add(self, rhs: &PairNum) -> Self::Output {
        let mut result = PairNum::new(Elem::Pair(self.clone()), Elem::Pair(rhs.clone()));
        result.reduce();
        return result;
    }
}

fn part1(text: &str) -> u64 {
    let nums = text
        .lines()
        .map(|line| PairNum::parse(line))
        .collect::<Vec<PairNum>>();
    let sum = nums.iter().skip(1).fold(nums[0].clone(), |acc, e| &acc + e);
    return sum.magnitude();
}

fn part2(text: &str) -> u64 {
    let nums = text
        .lines()
        .map(|line| PairNum::parse(line))
        .collect::<Vec<PairNum>>();
    let mut best_magnitude = u64::MIN;
    for i in 0..nums.len() {
        for j in 0..nums.len() {
            best_magnitude = best_magnitude.max((&nums[i] + &nums[j]).magnitude());
        }
    }
    return best_magnitude;
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn add_test() {
        assert_eq!(
            &PairNum {
                left: Box::new(Elem::Number(1)),
                right: Box::new(Elem::Number(2))
            } + &PairNum {
                left: Box::new(Elem::Number(3)),
                right: Box::new(Elem::Number(4))
            },
            PairNum {
                left: Box::new(Elem::Pair(PairNum {
                    left: Box::new(Elem::Number(1)),
                    right: Box::new(Elem::Number(2))
                })),
                right: Box::new(Elem::Pair(PairNum {
                    left: Box::new(Elem::Number(3)),
                    right: Box::new(Elem::Number(4))
                }))
            }
        )
    }

    #[test]
    fn to_str_test() {
        assert_eq!(
            PairNum::new(
                Elem::Pair(PairNum::new(Elem::Number(1), Elem::Number(2))),
                Elem::Pair(PairNum::new(Elem::Number(3), Elem::Number(4)))
            )
            .to_string(),
            "[[1,2],[3,4]]"
        )
    }

    #[test]
    fn parse_test() {
        assert_eq!(
            PairNum::parse("[[1,2],[3,4]]"),
            PairNum::new(
                Elem::Pair(PairNum::new(Elem::Number(1), Elem::Number(2))),
                Elem::Pair(PairNum::new(Elem::Number(3), Elem::Number(4)))
            )
        )
    }

    fn check_split(num_str: &str, result: &str) {
        let mut num = PairNum::parse(num_str);
        assert_eq!(num.try_split(), true);
        assert_eq!(num.to_string(), result);
    }

    fn check_explode(num_str: &str, result: &str) {
        let mut num = PairNum::parse(num_str);
        assert_eq!(num.try_explode(), true);
        assert_eq!(num.to_string(), result);
    }

    #[test]
    fn split_test() {
        check_split("[15,1]", "[[7,8],1]");
        check_split("[15,11]", "[[7,8],11]");
        check_split("[1,11]", "[1,[5,6]]");
        check_split(
            "[[[[0,7],4],[[7,8],[0,13]]],[1,1]]",
            "[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]",
        );
    }

    #[test]
    fn explode_test() {
        check_explode("[[[[[9,8],1],2],3],4]", "[[[[0,9],2],3],4]");
        check_explode("[7,[6,[5,[4,[3,2]]]]]", "[7,[6,[5,[7,0]]]]");
        check_explode(
            "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]",
            "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
        );
        check_explode(
            "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
            "[[3,[2,[8,0]]],[9,[5,[7,0]]]]",
        );
    }

    fn check_reduce(num_str: &str, result: &str) {
        let mut num = PairNum::parse(num_str);
        num.reduce();
        assert_eq!(num.to_string(), result);
    }

    #[test]
    fn reduce_test() {
        let sum = &PairNum::parse("[[[[4,3],4],4],[7,[[8,4],9]]]") + &PairNum::parse("[1,1]");
        println!("sum: {}", sum.to_string());
    }

    #[test]
    fn magnitude_test() {
        assert_eq!(PairNum::parse("[9,1]").magnitude(), 29);
        assert_eq!(
            PairNum::parse("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]").magnitude(),
            3488
        );
    }

    #[test]
    fn part1() {
        println!(
            "{}",
            super::part1(
                "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]"
            )
        );
        println!(
            "part1: {}",
            super::part1(include_str!("../resources/day18.txt"))
        );
    }

    #[test]
    fn part2() {
        println!(
            "part2: {}",
            super::part2(include_str!("../resources/day18.txt"))
        );
    }
}
