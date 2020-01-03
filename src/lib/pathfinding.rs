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
            Some(tile) => tile == &Tile::Empty,
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

    lazy_static! {
        static ref MAP: Map = {
            let mut map = HashMap::new();
            for y in 0..5 {
                for x in 0..5 {
                    map.insert(Point { x, y }, Tile::Empty);
                }
            }
            map
        };
    }

    #[test]
    fn should_find_best_route_between_points() {
        let start = Point { x: 1, y: 1 };
        let end = Point { x: 4, y: 4 };

        let expected_path = vec![(4, 4), (3, 4), (3, 3), (2, 3), (2, 2), (1, 2)]
            .into_iter()
            .map(|(x, y)| Point { x, y })
            .collect::<Vec<_>>();

        assert_eq!(find_path(&MAP, start, end), expected_path);
    }
}
