use super::points::*;
use super::vec_field::*;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;

#[derive(Clone, Eq, PartialEq, Debug)]
struct Node {
    position: Point,
    cost: u32,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.x.cmp(&other.position.x))
            .then_with(|| self.position.y.cmp(&other.position.y))
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn expand(node: &Node, field: &VecField<u8>) -> Vec<Node> {
    neighbors(
        &node.position,
        /*diagonals=*/ false,
        field.width,
        field.height,
    )
    .into_iter()
    .map(|point| Node {
        position: point,
        cost: node.cost + (field.get(&point) as u32),
    })
    .collect()
}

fn expand2(node: &Node, field: &RepeatedField) -> Vec<Node> {
    neighbors(
        &node.position,
        /*diagonals=*/ false,
        field.width(),
        field.height(),
    )
    .into_iter()
    .map(|point| {
        let new_cost_from_start = node.cost + (field.get(&point) as u32);

        Node {
            position: point,
            cost: new_cost_from_start,
        }
    })
    .collect()
}

pub fn part2(tile: VecField<u8>) -> u32 {
    let field = RepeatedField::new(tile);
    let start = Point { x: 0, y: 0 };
    let finish = Point {
        x: (field.width() - 1) as i32,
        y: (field.height() - 1) as i32,
    };

    let start_node = Node {
        position: start,
        cost: 0,
    };

    // start -> point distances
    let mut costs: HashMap<Point, u32> = HashMap::new();

    let mut queue: BinaryHeap<Node> = BinaryHeap::new();
    queue.push(start_node);
    while let Some(current) = queue.pop() {
        if current.position == finish {
            return current.cost;
        }

        if current.cost > *costs.get(&current.position).unwrap_or(&u32::MAX) {
            continue;
        }

        for next in expand2(&current, &field) {
            if next.cost < *costs.get(&next.position).unwrap_or(&u32::MAX) {
                costs.insert(next.position, next.cost);
                queue.push(next);
            }
        }
    }
    return 0;
}

struct RepeatedField {
    tile: VecField<u8>,
    tile_width: u8,
    tile_height: u8,
}

impl RepeatedField {
    fn new(tile: VecField<u8>) -> Self {
        RepeatedField {
            tile,
            tile_width: 5,
            tile_height: 5,
        }
    }

    fn width(&self) -> u32 {
        self.tile.width * (self.tile_width as u32)
    }

    fn height(&self) -> u32 {
        self.tile.height * (self.tile_height as u32)
    }

    fn get(&self, point: &Point) -> u8 {
        let tile_x = point.x / (self.tile.width as i32);
        let x_in_tile = point.x % (self.tile.width as i32);
        let tile_y = point.y / (self.tile.width as i32);
        let y_in_tile = point.y % (self.tile.width as i32);
        let mut value = self.tile.get(&Point {
            x: x_in_tile,
            y: y_in_tile,
        }) as i32;
        value += tile_x;
        value += tile_y;

        if value > 9 {
            value -= 9;
        }

        return value as u8;
    }
}

pub fn part1(field: &VecField<u8>) -> u32 {
    let start = Point { x: 0, y: 0 };
    let finish = Point {
        x: (field.width - 1) as i32,
        y: (field.height - 1) as i32,
    };

    let start_node = Node {
        position: start,
        cost: 0,
    };

    // start -> point distances
    let mut distances: HashMap<Point, u32> = HashMap::new();

    let mut queue: BinaryHeap<Node> = BinaryHeap::new();
    queue.push(start_node);
    while let Some(current) = queue.pop() {
        if current.position == finish {
            return current.cost;
        }

        if current.cost > *distances.get(&current.position).unwrap_or(&u32::MAX) {
            continue;
        }

        for next in expand(&current, &field) {
            if next.cost < *distances.get(&next.position).unwrap_or(&u32::MAX) {
                distances.insert(next.position, next.cost);
                queue.push(next);
            }
        }
    }

    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> VecField<u8> {
        parse_matrix(include_str!("../resources/day15_sample.txt"))
    }

    fn input() -> VecField<u8> {
        parse_matrix(include_str!("../resources/day15.txt"))
    }
    #[test]
    fn part1() {
        println!("part1 sample: {}", super::part1(&sample()));
        println!("part1: {}", super::part1(&input()));
    }

    #[test]
    fn part2() {
        println!("part2 sample: {}", super::part2(sample()));
        println!("part2: {}", super::part2(input()));
        //        println!("{}", super::part2(&input()));
    }
}
