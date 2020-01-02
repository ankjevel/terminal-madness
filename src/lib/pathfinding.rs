use crate::lib::shared::{Point, Tile};
use std::collections::{BinaryHeap, HashMap, HashSet};

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug, Ord, PartialOrd)]
struct State {
    cost: isize,
    point: Point,
}

fn distance(p1: &Point, p2: &Point) -> f64 {
    ((p2.x as f64 - p1.x as f64).powf(2f64) + (p2.y as f64 - p1.y as f64).powf(2f64)).sqrt()
}

fn is_between(a: &Point, c: &Point, b: &Point) -> bool {
    approx_eq!(
        f64,
        distance(&a, &c) + distance(&c, &b),
        distance(&a, &b),
        ulps = 2
    )
}

type Map = HashMap<Point, Tile>;

pub fn best_match(input: &Map, position: &Point, visited: &Vec<Point>) -> Option<Vec<Point>> {
    let within_range = || -> Option<Vec<Point>> {
        let get_closest = |range: usize| -> Option<Vec<Point>> {
            let input = input
                .clone()
                .iter()
                .filter(|(pos, tile)| {
                    (tile == &&Tile::Unknown || tile == &&Tile::Empty) && !visited.contains(pos)
                })
                .filter(|(pos, _title)| {
                    let x = ((position.x as f64) - (pos.x as f64)).abs() as usize;
                    let y = ((position.y as f64) - (pos.y as f64)).abs() as usize;

                    x <= range && y <= range
                })
                .map(|(pos, _tile)| pos.to_owned())
                .collect::<Vec<Point>>();

            if input.len() <= 0 {
                None
            } else {
                Some(input.to_owned())
            }
        };

        let mut range = 0;
        let mut result = None;

        loop {
            if let Some(res) = get_closest(range) {
                result = Some(res);
                break;
            }
            range += 1;
            if range >= input.len() {
                break;
            }
        }

        result
    };

    let available = match within_range() {
        Some(res) => res.to_owned(),
        None => return None,
    };

    let mut steps = vec![];

    for current in available.clone() {
        let path = match find_path(&input, position.to_owned(), current.to_owned()) {
            Some(path) => path.to_owned(),
            None => continue,
        };

        if steps.len() == 0 || path.len() < steps.len() {
            steps = path.to_owned();
        }
    }

    if steps.len() == 0 {
        None
    } else {
        Some(steps.to_owned())
    }
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

pub fn find_leafs(map: &Map, current: &Vec<Point>) -> Vec<Point> {
    let mut new_leafs: HashSet<Point> = HashSet::new();

    for point in current {
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
                if tile == &Tile::Empty {
                    new_leafs.insert(new_pos.to_owned());
                }
            }
        }
    }

    new_leafs.into_iter().collect::<Vec<Point>>()
}

pub fn find_path(map: &Map, start: Point, goal: Point) -> Option<Vec<Point>> {
    let can_move = |point: &Point| -> bool {
        match map.get(&point) {
            Some(tile) => tile == &Tile::Empty,
            None => false,
        }
    };

    let mut frontier = BinaryHeap::new();
    frontier.push(State {
        cost: 0,
        point: start,
    });

    let mut came_from = HashMap::new();
    came_from.insert(start, None);

    while frontier.len() != 0 {
        let current = frontier.pop();
        if current.unwrap().point == goal {
            break;
        }
        for next_point in adjacent(map, &current.unwrap().point) {
            if !came_from.contains_key(&next_point) && can_move(&next_point) {
                frontier.push(State {
                    point: next_point,
                    cost: distance(&goal, &next_point) as isize,
                });
                came_from.insert(next_point, current.map(|a| a.point));
            }
        }
    }

    let mut current = goal;
    let mut path = vec![current];

    while current != start {
        if let Some(c) = came_from.get(&current) {
            if let Some(c) = *c {
                current = c;
                path.push(current);
            } else {
                return None;
            }
        } else {
            return None;
        }
    }

    Some(path)
}
