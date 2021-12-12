use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

#[derive(Hash, Eq, PartialEq, Clone, Copy, Debug)]
struct Cave {
    name: &'static str,
    is_small: bool,
}

impl Cave {
    fn new(name: &'static str) -> Self {
        let is_small = name.chars().all(|ch| ch.is_lowercase());
        Cave { name, is_small }
    }
}

#[derive(Eq, PartialEq, Clone)]
struct CavePath {
    caves: Vec<Cave>,
    small_caves: HashSet<Cave>,
    double_small_cave: Option<Cave>,
    allow_double_small_cave: bool,
}

impl CavePath {
    fn new(alllow_double_small_cave: bool) -> Self {
        CavePath {
            caves: Vec::new(),
            small_caves: HashSet::new(),
            double_small_cave: None,
            allow_double_small_cave: alllow_double_small_cave,
        }
    }

    fn of<I>(caves: I, allow_double_small_cave: bool) -> Option<Self>
    where
        I: IntoIterator<Item = Cave>,
    {
        caves.into_iter().fold(
            Some(CavePath::new(allow_double_small_cave)),
            |result, cave| match result {
                Some(path) => path.extend(&cave),
                None => None,
            },
        )
    }

    /// Returns a new path with a [cave] at the end if possible.
    fn extend(&self, cave: &Cave) -> Option<CavePath> {
        if self.small_caves.contains(cave) {
            if self.allow_double_small_cave
                && self.double_small_cave.is_none()
                && cave.name != "start"
                && cave.name != "end"
            {
                let mut copy = self.clone();
                copy.caves.push(*cave);
                copy.double_small_cave = Some(*cave);
                Some(copy)
            } else {
                None
            }
        } else {
            let mut copy = self.clone();
            copy.caves.push(*cave);
            if cave.is_small {
                copy.small_caves.insert(*cave);
            }
            Some(copy)
        }
    }

    fn print(&self) -> String {
        self.caves
            .iter()
            .map(|c| c.name)
            .collect::<Vec<&str>>()
            .join(" -> ")
    }
}

pub struct CaveSystem {
    caves: HashMap<Cave, Vec<Cave>>,
}

impl CaveSystem {
    fn new() -> Self {
        CaveSystem {
            caves: HashMap::new(),
        }
    }

    fn add_path(&mut self, from: Cave, to: Cave) {
        self.caves
            .entry(from)
            .and_modify(|v| v.push(to.clone()))
            .or_insert(vec![to]);
        self.caves
            .entry(to)
            .and_modify(|v| v.push(from.clone()))
            .or_insert(vec![from]);
    }
    fn parse(text: &'static str) -> Self {
        let mut result = CaveSystem::new();
        for line in text.lines() {
            let (from, to) = line.split_once('-').unwrap();
            result.add_path(Cave::new(from), Cave::new(to));
        }
        result
    }

    fn print(&self) -> String {
        let mut result = String::new();
        for (from, tos) in &self.caves {
            for to in tos {
                result = format!("{}\n{} -> {}", result, from.name, to.name,);
            }
        }
        result
    }
}

fn traverse(cave_system: &CaveSystem, allow_double_small_cave: bool) -> Vec<CavePath> {
    let start = Cave::new("start");
    let end = Cave::new("end");

    // Start with a single path consisting of a "start".
    let mut queue = VecDeque::from(vec![
        CavePath::of(vec![start], allow_double_small_cave).unwrap()
    ]);
    let mut finished_paths: Vec<CavePath> = Vec::new();

    while !queue.is_empty() {
        let path = queue.pop_front().unwrap();
        let last = path.caves.last().unwrap();

        // If path is finished, don't go anywhere else.
        if *last == end {
            // println!("Found finished path: {}", path.print());
            finished_paths.push(path);
            continue;
        }

        // Otherwise, find where else we can go.
        let destinations = cave_system.caves.get(last);
        if destinations.is_none() {
            println!("No destinations from {:?}", last);
            continue;
        }
        queue.extend(
            destinations
                .unwrap()
                .iter()
                .filter_map(|cave| path.extend(cave)),
        );
    }

    finished_paths
}
pub fn part1(cave_system: &CaveSystem) -> u32 {
    traverse(cave_system, false).len() as u32
}

pub fn part2(cave_system: &CaveSystem) -> u32 {
    traverse(cave_system, true).len() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> CaveSystem {
        CaveSystem::parse(include_str!("../resources/day12.txt"))
    }
    #[test]
    fn part1() {
        let sample1 = CaveSystem::parse(
            "start-A
start-b
A-c
A-b
b-d
A-end
b-end
",
        );
        assert_eq!(super::part1(&sample1), 10);
        println!("part1: {}", super::part1(&input()));
    }

    #[test]
    fn part2() {
        let sample1 = CaveSystem::parse(
            "start-A
start-b
A-c
A-b
b-d
A-end
b-end
",
        );
        assert_eq!(super::part2(&sample1), 36);
        println!("part2: {}", super::part2(&input()));
    }
}
