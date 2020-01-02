use crate::lib::shared::{Point, Tile};
use std::{
    collections::{BinaryHeap, HashMap},
    usize,
};

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug, Ord, PartialOrd)]
struct State {
    cost: usize,
    point: Point,
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

pub fn find_path(map: &Map, start: Point, goal: Point) -> Option<Vec<Point>> {
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

        if cost > *dist.entry(point.to_owned()).or_insert(usize::MAX) {
            continue;
        }

        for edge in adjacent(map, &point) {
            if !came_from.contains_key(&edge) && can_move(&edge) {
                let next_cost = distance(&goal, &edge) as usize;

                let prev_cost = dist.entry(edge.to_owned()).or_insert(usize::MAX);

                if next_cost >= *prev_cost {
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
            Some(c) => match *c {
                Some(c) => {
                    current = c;

                    if current == start {
                        continue;
                    }

                    path.push(current);
                }
                _ => {
                    return None;
                }
            },
            _ => return None,
        }
    }
    Some(path)
}
