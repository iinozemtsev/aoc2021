pub fn part1(positions: &Vec<i32>) -> i32 {
    solution(&positions, std::convert::identity)
}

fn move_cost(distance: i32) -> i32 {
    distance * (distance + 1) / 2
}

fn total_distance<F>(positions: &Vec<i32>, position: i32, f: F) -> i32
where
    F: Fn(i32) -> i32,
{
    positions.iter().map(|p| f((p - position).abs())).sum()
}

fn solution<F>(positions: &Vec<i32>, f: F) -> i32
where
    F: Fn(i32) -> i32,
{
    let mut from = positions.iter().fold(i32::MAX, |a, v| i32::min(a, *v));
    let mut to = positions.iter().fold(i32::MIN, |a, v| i32::max(a, *v));
    loop {
        let middle = (from + to) / 2;
        let at_from = total_distance(&positions, from, &f);
        let at_to = total_distance(&positions, to, &f);
        let at_middle = total_distance(&positions, middle, &f);

        if at_middle < at_from && at_middle < at_to {
            // middle might be a solution, cannot exclude it.
            if at_from < at_to {
                to = middle;
            } else {
                from = middle;
            }
        } else if at_from < at_to {
            to = middle
        } else if at_from > at_to {
            from = middle + 1
        } else {
            return at_from;
        }
    }
}
pub fn part2(positions: &Vec<i32>) -> i32 {
    solution(&positions, move_cost)
}

#[cfg(test)]
mod tests {
    use super::super::parse::parse_comma_separated;

    fn get_sample_input() -> Vec<i32> {
        parse_comma_separated("16,1,2,0,4,2,7,1,2,14")
    }

    fn get_input() -> Vec<i32> {
        parse_comma_separated(include_str!("../resources/day7.txt"))
    }

    #[test]
    fn part1() {
        assert_eq!(super::part1(&get_sample_input()), 37);
        println!("part1: {}", super::part1(&get_input()))
    }

    #[test]
    fn part2() {
        println!("part2 sample: {}", super::part2(&get_sample_input()));
        println!("part2: {}", super::part2(&get_input()));
    }
}
