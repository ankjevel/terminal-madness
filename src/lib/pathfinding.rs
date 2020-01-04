use crate::lib::shared::{Point, Tile};
use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
    usize,
};

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
struct State {
    cost: usize,
    point: Point,
}

impl Ord for State {
    fn cmp(&self, other: &State) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.point.cmp(&other.point))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &State) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

type Map = HashMap<Point, Tile>;

fn distance(p1: &Point, p2: &Point) -> f64 {
    ((p2.x as f64 - p1.x as f64).powf(2f64) + (p2.y as f64 - p1.y as f64).powf(2f64)).sqrt()
}

pub fn adjacent(map: &Map, point: &Point) -> Vec<Point> {
    let mut tiles = Vec::new();

    let mut vec: Vec<(isize, isize)> = vec![(0, 1), (1, 0)];

    if point.x > 0 {
        vec.push((-1, 0));
    }

    if point.y > 0 {
        vec.push((0, -1));
    }

    for (x, y) in vec {
        let new_pos = Point {
            x: (point.x as isize + x) as usize,
            y: (point.y as isize + y) as usize,
        };
        if let Some(tile) = map.get(&new_pos) {
            if tile != &Tile::Wall {
                tiles.push(new_pos.to_owned());
            }
        };
    }

    tiles
}

pub fn find_path(map: &Map, start: Point, goal: Point) -> Vec<Point> {
    let can_move = |point: &Point| -> bool {
        match map.get(&point) {
            Some(tile) => tile != &Tile::Wall,
            None => false,
        }
    };

    let mut frontier = BinaryHeap::new();
    let mut dist: HashMap<Point, usize> = HashMap::new();

    frontier.push(State {
        cost: 0,
        point: start,
    });

    let mut came_from = HashMap::new();
    came_from.insert(start, None);

    dist.insert(start.to_owned(), 0);

    while let Some(State { point, cost }) = frontier.pop() {
        if point == goal {
            break;
        }

        let to_point = dist.entry(point.to_owned()).or_insert(usize::MAX);

        if cost > *to_point {
            continue;
        }

        for edge in adjacent(map, &point) {
            if !came_from.contains_key(&edge) && can_move(&edge) {
                let next_cost = distance(&goal, &edge) as usize;

                let prev_cost = dist.entry(edge.to_owned()).or_insert(usize::MAX);

                if next_cost > *prev_cost {
                    continue;
                }

                frontier.push(State {
                    point: edge,
                    cost: next_cost,
                });

                came_from.insert(edge, Some(point));
                *prev_cost = next_cost;
            }
        }
    }

    let mut current = goal;
    let mut path = vec![current];
    while current != start {
        match came_from.get(&current) {
            Some(c) => {
                if c.is_none() {
                    return vec![];
                }

                current = c.unwrap();

                if current == start {
                    continue;
                }

                path.push(current);
            }
            _ => return vec![],
        }
    }

    path
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_map(input: &String) -> HashMap<Point, Tile> {
        input
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty())
            .map(str::to_owned)
            .map(|line| line.chars().filter(|char| char != &'|').collect::<String>())
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, char)| {
                        (
                            Point { x, y },
                            match char {
                                '█' => Tile::Wall,
                                'C' => Tile::Current,
                                'X' => Tile::NPC,
                                _ => Tile::Empty,
                            },
                        )
                    })
                    .collect::<Vec<_>>()
            })
            .flatten()
            .collect()
    }

    #[test]
    ///
    /// Using Dijkstras algoritm; the prop should find the shortest route
    /// between `1,1` to `4,4`. There should be, _at the most_, 6 steps.
    ///
    ///     0,0 1,0 2,0 3,0 4,0
    ///     0,1 1,1 2,1 3,1 4,1
    ///     0,2 1,2 2,2 3,2 4,2
    ///     0,3 1,3 2,3 3,3 4,3
    ///     0,4 1,4 2,4 3,4 4,4
    ///
    fn it_should_find_best_route_between_points() {
        let example = "
            █     █
            █ S   █
            █     █
            █     █
            █    E█
        ";

        let start = Point { x: 1, y: 1 };
        let end = Point { x: 4, y: 4 };

        assert_eq!(
            find_path(&parse_map(&example.to_string()), start, end).len(),
            6
        );
    }

    #[test]
    fn it_should_go_around_walls() {
        let example = "
            █    ███
            █S █   █
            █  ███ █
            █ █E   █
            █  █████
        ";

        let start = Point { x: 0, y: 1 };
        let end = Point { x: 2, y: 3 };

        assert_eq!(
            find_path(&parse_map(&example.to_string()), start, end).len(),
            12
        );
    }

    #[test]
    /// ignore characters, and other npcs, since they can move before the prop
    /// gets to the target
    fn it_ignores_character_and_other_npc_when_finding_route() {
        let example = "
            ████████
            █  XX  █
            █S C E █
            █  XX  █
            █  ██  █
            ████████
        ";

        let start = Point { x: 0, y: 1 };
        let end = Point { x: 4, y: 1 };

        assert_eq!(
            find_path(&parse_map(&example.to_string()), start, end).len(),
            4
        );
    }
}
