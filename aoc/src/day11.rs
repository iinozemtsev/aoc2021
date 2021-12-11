use super::points::*;
use arrayvec::ArrayVec;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::str::FromStr;

#[derive(Clone, Copy)]
pub struct Octopi {
    values: [u8; 100],
}

impl ToString for Octopi {
    fn to_string(&self) -> String {
        let mut result = String::new();
        for row in 0..10 {
            for col in 0..10 {
                result.push(('0' as u8 + self.values[row * 10 + col]) as char);
            }
            result.push('\n');
        }
        result
    }
}
impl Octopi {
    fn get(&self, point: Point) -> u8 {
        self.values[(point.y * 10 + point.x) as usize]
    }

    fn set(&mut self, point: Point, val: u8) {
        self.values[(point.y * 10 + point.x) as usize] = val;
    }
}

fn evolve_times(o: &Octopi, times: u32) -> (u32, Octopi) {
    (0..times).fold((0, *o), |(total, result), _| {
        let (inc, new_o) = evolve(&result);
        return (total + inc as u32, new_o);
    })
}
fn evolve(o: &Octopi) -> (u8, Octopi) {
    let mut result = *o;
    let mut overflown: HashSet<Point> = HashSet::new();
    let mut queue: VecDeque<Point> = VecDeque::new();
    for y in 0..10 {
        for x in 0..10 {
            let point = Point { x, y };
            let val = o.get(point);
            if val == 9 {
                overflown.insert(point);
                queue.push_back(point);
                result.set(point, 0);
            } else {
                result.set(point, val + 1);
            }
        }
    }

    while !queue.is_empty() {
        let next = queue.pop_front().unwrap();
        for neighbor in neighbors(&next, true, 10, 10) {
            if overflown.contains(&neighbor) {
                continue;
            }
            let val = result.get(neighbor);
            if val == 9 {
                overflown.insert(neighbor);
                queue.push_back(neighbor);
                result.set(neighbor, 0);
            } else if !overflown.contains(&neighbor) {
                result.set(neighbor, val + 1);
            }
        }
    }
    return (overflown.len() as u8, result);
}

impl FromStr for Octopi {
    type Err = String;
    fn from_str(text: &str) -> Result<Octopi, String> {
        Ok(Octopi {
            values: text
                .lines()
                .flat_map(|l| l.chars())
                .map(|ch| ch as u8 - ('0' as u8))
                .collect::<ArrayVec<u8, 100>>()
                .into_inner()
                .unwrap(),
        })
    }
}

pub fn part1(o: &Octopi) -> u32 {
    let (c, _) = evolve_times(&o, 100);
    return c;
}

pub fn part2(o: &Octopi) -> u32 {
    let mut current = *o;
    let mut step = 0;
    loop {
        step += 1;
        let (count, new_current) = evolve(&current);
        current = new_current;
        if count == 100 {
            return step;
        }
    }
    
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_input() -> Octopi {
        "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526"
            .parse::<Octopi>()
            .unwrap()
    }

    fn input() -> Octopi {
        "7222221271
6463754232
3373484684
4674461265
1187834788
1175316351
8211411846
4657828333
5286325337
5771324832"
            .parse::<Octopi>()
            .unwrap()
    }

    #[test]
    fn part1() {
        assert_eq!(super::part1(&sample_input()), 1656);
        println!("part1: {}", super::part1(&input()));
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(&sample_input()), 195);
        println!("part2: {}", super::part2(&input()));
    }

}
