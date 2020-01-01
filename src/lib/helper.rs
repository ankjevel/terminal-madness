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

                let (x_range, y_range, tile, meta) = (
                    iter.next().unwrap_or("0"),
                    iter.next().unwrap_or("0"),
                    iter.next().unwrap_or("0").parse::<u8>().unwrap_or(0),
                    (
                        iter.next().unwrap_or("0").parse::<u8>().unwrap_or(0),
                        iter.next().unwrap_or("0").parse::<u8>().unwrap_or(0),
                    ),
                );

                let mut range = x_range.split("-");
                let x_start = range.next().unwrap().parse::<usize>().unwrap_or(0);
                let x_end = range
                    .next()
                    .unwrap_or(&x_start.to_string())
                    .parse::<usize>()
                    .unwrap();

                let mut range = y_range.split("-");
                let y_start = range.next().unwrap().parse::<usize>().unwrap_or(0);
                let y_end = range
                    .next()
                    .unwrap_or(&y_start.to_string())
                    .parse::<usize>()
                    .unwrap();

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
