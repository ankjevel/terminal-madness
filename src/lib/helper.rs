use crate::lib::shared::{MapStruct, Point};
use std::{collections::HashMap, str::FromStr};

pub fn unwrap_or<'a, I, D>(input: &mut I, default: D) -> D
where
    I: Iterator<Item = D>,
    D: 'a,
{
    input.next().unwrap_or(default)
}

pub fn unwrap_and_parse<'a, I, T>(input: &mut I, a: &'a str, b: T) -> T
where
    T: 'a + FromStr,
    I: Iterator<Item = &'a str>,
{
    input.next().unwrap_or(a).parse::<T>().unwrap_or(b)
}

fn start_end<'a>(input: &'a str) -> (usize, usize) {
    let mut range = input.split("-");
    let start = unwrap_and_parse(&mut range, "0", 0);
    let start_str = start.to_string();
    let end = unwrap_and_parse(&mut range, &start_str, 0);
    (start, end)
}

fn usize_iter<'a>(input: &'a str) -> impl Iterator<Item = usize> {
    input
        .split(",")
        .map(|str| str.parse::<usize>().unwrap_or(0))
        .collect::<Vec<_>>()
        .into_iter()
}

pub fn parse_input(str: &str) -> Vec<MapStruct> {
    str.lines()
        .map(str::trim_end)
        .map(str::to_owned)
        .collect::<Vec<_>>()
        .chunks(4)
        .map(|chunk| {
            let mut input = chunk.into_iter();
            let default_input = &"0|0|0,0|0,0,0".to_string();
            let mut meta = unwrap_or(&mut input, default_input).split("|");
            let (area, part, size, player) = (
                unwrap_and_parse(&mut meta, "0", 0),
                unwrap_and_parse(&mut meta, "0", 0),
                unwrap_or(&mut meta, "0,0"),
                unwrap_or(&mut meta, "0,0,0"),
            );

            let mut iter = usize_iter(size);
            let (max_x, max_y) = (unwrap_or(&mut iter, 0), unwrap_or(&mut iter, 0));

            let mut iter = usize_iter(player);
            let (player_x, player_y, player_direction) = (
                unwrap_or(&mut iter, 0),
                unwrap_or(&mut iter, 0),
                unwrap_or(&mut iter, 0) as u8,
            );

            let empty_str = "".to_string();
            for prop in unwrap_or(&mut input, &empty_str).split("|") {
                if prop.is_empty() {
                    continue;
                }

                println!("prop: {:?}", prop)
            }

            for enemy in unwrap_or(&mut input, &empty_str).split("|") {
                if enemy.is_empty() {
                    continue;
                }

                println!("enemy: {:?}", enemy)
            }

            let mut grid = HashMap::new();
            for chars in unwrap_or(&mut input, &empty_str).split("|") {
                let mut iter = chars.split(",");

                let (x_range, y_range, tile, meta) = (
                    unwrap_or(&mut iter, "0"),
                    unwrap_or(&mut iter, "0"),
                    unwrap_and_parse(&mut iter, "0", 0),
                    (
                        unwrap_and_parse(&mut iter, "0", 0),
                        unwrap_and_parse(&mut iter, "0", 0),
                    ),
                );

                let ((x_start, x_end), (y_start, y_end)) = (start_end(x_range), start_end(y_range));

                for x in x_start..=x_end {
                    for y in y_start..=y_end {
                        grid.insert(Point { x, y }, (tile, meta));
                    }
                }
            }

            (
                (area, part),
                grid,
                (max_x, max_y),
                (player_x, player_y, player_direction),
            )
        })
        .collect::<Vec<_>>()
}
