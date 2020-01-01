use crate::lib::shared::{MapStruct, Point};
use std::collections::HashMap;

pub fn parse_input(str: &str) -> Vec<MapStruct> {
    str.lines()
        .map(str::trim_end)
        .map(str::to_owned)
        .collect::<Vec<_>>()
        .chunks(2)
        .map(|chunk| {
            let mut input = chunk.into_iter();
            let default_input = &"0|0|0,0|0,0,0".to_string();
            let mut meta = input.next().unwrap_or(default_input).split("|");
            let (area, part, size, player) = (
                meta.next().unwrap_or("0").parse::<u8>().unwrap_or(0),
                meta.next().unwrap_or("0").parse::<u8>().unwrap_or(0),
                meta.next().unwrap_or("0,0"),
                meta.next().unwrap_or("0,0,0"),
            );

            let mut iter = size
                .split(",")
                .map(|str| str.parse::<usize>().unwrap_or(0))
                .collect::<Vec<_>>()
                .into_iter();

            let (max_x, max_y) = (iter.next().unwrap_or(0), iter.next().unwrap_or(0));

            let mut iter = player
                .split(",")
                .map(|str| str.parse::<usize>().unwrap_or(0))
                .collect::<Vec<_>>()
                .into_iter();

            let (player_x, player_y, player_direction) = (
                iter.next().unwrap_or(0),
                iter.next().unwrap_or(0),
                iter.next().unwrap_or(0) as u8,
            );

            let mut grid = HashMap::new();
            for chars in input.next().unwrap_or(&"".to_string()).split("|") {
                let mut iter = chars.split(",");
                let (x, y, tile) = (
                    iter.next().unwrap_or("0").parse::<usize>().unwrap_or(0),
                    iter.next().unwrap_or("0").parse::<usize>().unwrap_or(0),
                    iter.next().unwrap_or("0").parse::<u8>().unwrap_or(0),
                );
                grid.insert(Point { x, y }, tile.to_owned());
            }

            for y in 0..max_y {
                for x in 0..max_x {
                    let point = Point { x, y };
                    if y == 0 || y == max_y - 1 || x == 0 || x == max_x - 1 {
                        grid.insert(point, 0);
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
