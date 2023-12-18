use std::collections::{BinaryHeap, HashSet};

type CityMap = Vec<Vec<usize>>;
#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum Direction {
    North,
    East,
    West,
    South
}

impl Direction {
    fn opposite(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    // Return the valid next points from this point. It won't include
    // any that are out of bounds.
    fn valid_next(&self, grid: &CityMap) -> Vec<(Direction, Point)> {
        let mut next = Vec::new();
        if self.x > 0 {
            next.push((Direction::West, Self::new(self.x - 1, self.y)));
        }
        if self.y > 0 {
            next.push((Direction::North, Self::new(self.x, self.y - 1)));
        }
        if self.x < grid[0].len() - 1 {
            next.push((Direction::East, Self::new(self.x + 1, self.y)));
        }
        if self.y < grid.len() - 1 {
            next.push((Direction::South, Self::new(self.x, self.y + 1)));
        }
        next
    }
}

pub fn calculate_shortest_path(input:&str, crucible_type:CrucibleType) -> usize {
    let map:CityMap = input.split("\n")
        .map(|line|line.chars().map(|c|c.to_digit(10).unwrap() as usize).collect::<Vec<_>>())
        .collect();

    match crucible_type {
        CrucibleType::Regular => find_path(&map),
        CrucibleType::Ultra => find_ultra_path(&map),
    }
}

#[derive(Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Node {
    position: Point,
    direction: Direction,
    direction_count: usize,
}

impl Node {
    fn new(pos:Point, direction:Direction, direction_count: usize) -> Node {
        Node {
            position: pos,
            direction: direction,
            direction_count: direction_count
        }
    }
}

trait SteerableCrucible {
    fn successors(&self, map:&CityMap) -> Vec<Self> where Self: Sized;
}

#[derive(Clone, Eq, PartialEq, Hash)]
struct Crucible {
    cost: usize,
    node: Node
}

#[derive(Clone, Eq, PartialEq, Hash)]
struct UltraCrucible {
    cost: usize,
    node: Node
}

impl SteerableCrucible for Crucible {
    fn successors(&self, map:&CityMap) -> Vec<Crucible> {
        let mut neighbours = Vec::new();
        // Get the possible next valid points.
        for (d, p) in self.node.position.valid_next(map) {
            let cost = self.cost + map[p.y][p.x];
            if self.node.direction.opposite() == d {
                continue;
            }

            if self.node.direction == d && self.node.direction_count == 3 {
                continue;
            }

            let direction_count = if self.node.direction == d {
                self.node.direction_count + 1
            } else {
                1
            };

            neighbours.push(Crucible{cost: cost, node: Node::new(p, d, direction_count)});
        }
        neighbours
    }
}

impl SteerableCrucible for UltraCrucible {
    fn successors(&self, map:&CityMap) -> Vec<UltraCrucible> {
        let mut neighbours = Vec::new();
        // Get the possible next valid points.
        for (d, p) in self.node.position.valid_next(map) {
            let cost = self.cost + map[p.y][p.x];
            if self.node.direction.opposite() == d {
                continue;
            }

            if self.node.direction == d && self.node.direction_count == 10 {
                continue;
            }

            if self.node.direction != d && self.node.direction_count < 4 {
                continue;
            }

            let direction_count = if self.node.direction == d {
                self.node.direction_count + 1
            } else {
                1
            };

            neighbours.push(UltraCrucible{cost: cost, node: Node::new(p, d, direction_count)});
        }
        neighbours
    }
}

impl Ord for Crucible {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // We are using a min heap, so we are doing this backwards.
        other
            .cost
            .cmp(&self.cost)
    }
}

impl PartialOrd for Crucible {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for UltraCrucible {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // We are using a min heap, so we are doing this backwards.
        other
            .cost
            .cmp(&self.cost)
    }
}

impl PartialOrd for UltraCrucible {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub enum CrucibleType {
    Regular,
    Ultra
}

fn find_path(map:&CityMap) -> usize {
    let goal = Point::new(map[0].len() - 1, map.len() - 1);

    // Priority Queue
    let mut pq = BinaryHeap::new();
    pq.push(Crucible {
        cost: map[0][1],
        node: Node::new(Point::new(1,0), Direction::East, 1)
    });
    pq.push(Crucible {
        cost: map[1][0],
        node: Node::new(Point::new(0,1), Direction::South, 1)
    });

    let mut seen = HashSet::new();

    while let Some(crucible) = pq.pop() {
        if crucible.node.position == goal {
            return crucible.cost;
        }

        for successor in crucible.successors(map) {
            let node = successor.node.clone();
            if seen.insert(((node.position.x, node.position.y), node.direction, node.direction_count)) {
                pq.push(successor);
            }
        }
    }

    panic!("should never be here");
}

fn find_ultra_path(map:&CityMap) -> usize {
    let goal = Point::new(map[0].len() - 1, map.len() - 1);

    // Priority Queue
    let mut pq = BinaryHeap::new();
    pq.push(UltraCrucible {
        cost: map[0][1],
        node: Node::new(Point::new(1,0), Direction::East, 1)
    });
    pq.push(UltraCrucible {
        cost: map[1][0],
        node: Node::new(Point::new(0,1), Direction::South, 1)
    });

    let mut seen = HashSet::new();

    while let Some(crucible) = pq.pop() {
        if crucible.node.position == goal {
            return crucible.cost;
        }

        for successor in crucible.successors(map) {
            let node = successor.node.clone();
            if seen.insert(((node.position.x, node.position.y), node.direction, node.direction_count)) {
                pq.push(successor);
            }
        }
    }

    panic!("should never be here");
}

#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;

    #[test]
    fn test_calculate_shortest_path() {
        let input = fs::read_to_string("test_input.txt")
            .expect("Could not read file test_input.txt");

        assert_eq!(102, calculate_shortest_path(&input, CrucibleType::Regular))
    }

    #[test]
    fn test_calculate_shortest_ultimate_path() {
        let input = fs::read_to_string("test_input.txt")
            .expect("Could not read file test_input.txt");

        assert_eq!(94, calculate_shortest_path(&input, CrucibleType::Ultra))
    }
}