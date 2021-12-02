struct Offset(i32, i32);

impl std::ops::Add<&Offset> for Offset {
    type Output = Offset;
    fn add(self, rhs: &Offset) -> Offset {
        Offset(self.0 + rhs.0, self.1 + rhs.1)
    }
}
impl std::ops::Mul<i32> for Offset {
    type Output = Offset;

    fn mul(self, rhs: i32) -> Offset {
        Offset(self.0 * rhs, self.1 * rhs)
    }
}
impl std::str::FromStr for Offset {
    fn from_str(input: &str) -> std::result::Result<Self, String> {
        let parts = input
            .split_once(" ")
            .ok_or(format!("Invalid line: {}", input))?;
        let distance = parts.1.parse::<i32>().map_err(|e| e.to_string())?;
        let direction = match parts.0 {
            "forward" => Ok(Offset(1, 0)),
            "down" => Ok(Offset(0, 1)),
            "up" => Ok(Offset(0, -1)),
            _ => Err(format!("Invalid direction: {}", parts.0)),
        }?;

        Ok(direction * distance)
    }
    type Err = String;
}

pub fn part1() -> i32 {
    let position = get_input().iter().fold(Offset(0, 0), |acc, i| acc + i);
    return position.0 * position.1;
}

pub fn part2() -> i32 {
    let mut aim = 0;
    let mut position = Offset(0, 0);

    for offset in get_input() {
        aim += offset.1;
        position.0 += offset.0;
        position.1 += offset.0 * aim;
    }
    return position.0 * position.1;
}

fn get_input() -> Vec<Offset> {
    std::str::from_utf8(include_bytes!("part2.txt"))
        .unwrap()
        .lines()
        .map(|line| line.parse::<Offset>().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1() {
        println!("part 1 answer: {}", super::part1());
    }
    #[test]
    fn part2() {
        println!("part 2 answer: {}", super::part2());
    }
}
