use crate::lib::shared::{ParsedMap, Point};
use std::{
    collections::HashMap,
    str::FromStr,
    string::{String, ToString},
};
use termion::color::{self, Color};

lazy_static! {
    static ref EMPTY_STR: String = "".to_string();
    static ref DEFAULT_INPUT: String = "0|0|0,0|0,0,0".to_string();
}

pub fn with_color<'a, C>(tile: &str, print_color: C) -> String
where
    C: 'a + Color,
{
    format!(
        "{}{}{}",
        color::Fg(print_color),
        tile,
        color::Fg(color::Reset)
    )
}

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

fn start_end<'a, T>(input: &'a str, a: &'a str, b: T) -> (T, T)
where
    T: 'a + FromStr + ToString + Copy,
{
    let mut range = input.split("-");
    let start = unwrap_and_parse(&mut range, a, b);
    let start_str = start.to_string();
    let end = unwrap_and_parse(&mut range, &start_str, b);
    (start, end)
}

fn usize_iter<'a>(input: &'a str) -> impl Iterator<Item = usize> {
    input
        .split(",")
        .map(|str| str.parse::<usize>().unwrap_or(0))
        .collect::<Vec<_>>()
        .into_iter()
}

pub fn parse_maps(str: &str) -> Vec<ParsedMap> {
    str.lines()
        .map(str::trim_end)
        .map(str::to_owned)
        .collect::<Vec<_>>()
        .chunks(3)
        .filter(|chunk| {
            let mut chunk = chunk.into_iter();
            !chunk.next().unwrap_or(&EMPTY_STR).is_empty()
        })
        .map(|chunk| {
            let mut input = chunk.into_iter();

            let mut meta = unwrap_or(&mut input, &DEFAULT_INPUT).split("|");
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

            let mut props = HashMap::new();
            for prop in unwrap_or(&mut input, &EMPTY_STR).split("|") {
                if prop.is_empty() {
                    continue;
                }

                let mut iter = prop.split(",");
                let (id, (x_start, x_end), (y_start, y_end)) = (
                    unwrap_and_parse(&mut iter, "0", 0),
                    start_end(unwrap_or(&mut iter, "0"), "0", 0),
                    start_end(unwrap_or(&mut iter, "0"), "0", 0),
                );

                props.insert(id, (x_start..=x_end, y_start..=y_end));
            }

            let mut grid = HashMap::new();
            for chars in unwrap_or(&mut input, &EMPTY_STR).split("|") {
                if chars.is_empty() {
                    continue;
                }

                let mut iter = chars.split(",");
                let ((x_start, x_end), (y_start, y_end), tile, meta) = (
                    start_end(unwrap_or(&mut iter, "0"), "0", 0),
                    start_end(unwrap_or(&mut iter, "0"), "0", 0),
                    unwrap_and_parse(&mut iter, "0", 0),
                    (
                        unwrap_and_parse(&mut iter, "0", 0),
                        unwrap_and_parse(&mut iter, "0", 0),
                    ),
                );

                for x in x_start..=x_end {
                    for y in y_start..=y_end {
                        grid.insert(Point { x, y }, (tile, meta));
                    }
                }
            }

            ParsedMap {
                area,
                part,
                grid,
                max: (max_x, max_y),
                player: (player_x, player_y, player_direction),
                props,
            }
        })
        .collect::<Vec<_>>()
}
