use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct Position(usize, usize);

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: Position,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.0.cmp(&other.position.0))
            .then_with(|| self.position.1.cmp(&other.position.1))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug)]
struct Edge {
    node: Position,
    cost: usize,
}

// Dijkstra's shortest path algorithm.
fn shortest_path(
    adj_list: &HashMap<Position, Vec<Edge>>,
    start: Position,
    end: Position,
) -> Option<usize> {
    let mut dist: HashMap<Position, usize> =
        HashMap::from_iter(adj_list.keys().map(|pos| (*pos, usize::MAX)));

    let mut heap = BinaryHeap::new();

    dist.insert(start, 0);
    heap.push(State {
        cost: 0,
        position: start,
    });

    while let Some(State { cost, position }) = heap.pop() {
        if position == end {
            return Some(cost);
        }

        if cost > dist[&position] {
            continue;
        }

        for edge in &adj_list[&position] {
            let next = State {
                cost: cost + edge.cost,
                position: edge.node,
            };

            if next.cost < dist[&next.position] {
                heap.push(next);
                dist.insert(next.position, next.cost);
            }
        }
    }

    // end not reachable
    None
}

pub fn process_part1(input: &str) -> String {
    let mut grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut adj_list: HashMap<Position, Vec<Edge>> = HashMap::new();
    let (mut start, mut end) = (Position(0, 0), Position(0, 0));

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == 'S' {
                start = Position(i, j);
                grid[i][j] = 'a';
            }
            if grid[i][j] == 'E' {
                end = Position(i, j);
                grid[i][j] = 'z';
            }
        }
    }

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            let position = Position(i, j);
            let value = grid[i][j];
            adj_list.insert(position, vec![]);

            if i != 0 && value as i8 - grid[i - 1][j] as i8 >= -1 {
                adj_list.get_mut(&position).unwrap().push(Edge {
                    node: Position(i - 1, j),
                    cost: 1,
                });
            }

            if j != 0 && value as i8 - grid[i][j - 1] as i8 >= -1 {
                adj_list.get_mut(&position).unwrap().push(Edge {
                    node: Position(i, j - 1),
                    cost: 1,
                });
            }

            if i != grid.len() - 1 && value as i8 - grid[i + 1][j] as i8 >= -1 {
                adj_list.get_mut(&position).unwrap().push(Edge {
                    node: Position(i + 1, j),
                    cost: 1,
                });
            }
            if j != grid[i].len() - 1 && value as i8 - grid[i][j + 1] as i8 >= -1 {
                adj_list.get_mut(&position).unwrap().push(Edge {
                    node: Position(i, j + 1),
                    cost: 1,
                });
            }
        }
    }

    shortest_path(&adj_list, start, end).unwrap().to_string()
}

pub fn process_part2(input: &str) -> String {
    let mut grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut adj_list: HashMap<Position, Vec<Edge>> = HashMap::new();
    let mut end = Position(0, 0);

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == 'E' {
                end = Position(i, j);
                grid[i][j] = 'z';
            }
        }
    }

    let mut lowest_positions = vec![];

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            let position = Position(i, j);
            let value = grid[i][j];
            adj_list.insert(position, vec![]);

            if value == 'a' {
                lowest_positions.push(position);
            }

            if i != 0 && value as i8 - grid[i - 1][j] as i8 >= -1 {
                adj_list.get_mut(&position).unwrap().push(Edge {
                    node: Position(i - 1, j),
                    cost: 1,
                });
            }

            if j != 0 && value as i8 - grid[i][j - 1] as i8 >= -1 {
                adj_list.get_mut(&position).unwrap().push(Edge {
                    node: Position(i, j - 1),
                    cost: 1,
                });
            }

            if i != grid.len() - 1 && value as i8 - grid[i + 1][j] as i8 >= -1 {
                adj_list.get_mut(&position).unwrap().push(Edge {
                    node: Position(i + 1, j),
                    cost: 1,
                });
            }
            if j != grid[i].len() - 1 && value as i8 - grid[i][j + 1] as i8 >= -1 {
                adj_list.get_mut(&position).unwrap().push(Edge {
                    node: Position(i, j + 1),
                    cost: 1,
                });
            }
        }
    }

    lowest_positions
        .iter()
        .filter_map(|start| shortest_path(&adj_list, *start, end))
        .min()
        .unwrap()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    #[test]
    fn part1() {
        let result = process_part1(INPUT);
        assert_eq!(result, "31");
    }

    #[test]
    fn part2() {
        let result = process_part2(INPUT);
        assert_eq!(result, "29");
    }
}
