use std::collections::HashMap;
use Direction::*;
pub const INPUTS: &[&str] = &[INPUT, INPUT_A];
pub const INPUT: &str = include_str!("../../input/day12.txt");
pub const INPUT_A: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

#[inline]
pub fn run(input: &str) -> (String, String) {
    let (height_map, start, end) = {
        let map = input
            .lines()
            .enumerate()
            .flat_map(|(row, line)| {
                line.chars()
                    .enumerate()
                    .map(move |(col, ch)| ((col as i32, row as i32), ch))
            })
            .collect::<Vec<_>>();
        let start = map
            .iter()
            .find(|(_k, v)| *v == 'S')
            .map(|(k, _v)| *k)
            .unwrap();
        let end = map
            .iter()
            .find(|(_k, v)| *v == 'E')
            .map(|(k, _v)| *k)
            .unwrap();
        let heights: HashMap<_, _> = map
            .into_iter()
            .map(|(k, v)| {
                let num = match v {
                    'S' => 'a' as u32 - ('a' as u32) + 1,
                    'E' => 'z' as u32 - ('a' as u32) + 1,
                    num => num as u32 - ('a' as u32) + 1,
                };
                (k, num)
            })
            .collect();
        (heights, start, end)
    };

    let mut calculated_map: HashMap<(i32, i32), (u32, Direction)> =
        HashMap::from([(end, (0, End))]);

    let mut to_visit: Vec<((i32, i32), (u32, Direction, u32))> =
        Vec::from(neighbors(end, *height_map.get(&end).unwrap(), 0));

    while !to_visit.is_empty() {
        for (coord, (new_dist, direction, my_height)) in std::mem::take(&mut to_visit)
            .into_iter()
            .filter_map(|tile| {
                let (coord, tile) = tile;
                let (distance, direction, height) = tile;

                let out = height_map
                    .get(&coord)
                    .filter(|&&to_height| height <= to_height + 1)
                    .map(|height| ((coord), (distance, direction, *height)));
                out
            })
        {
            if let Some((distance, _)) = calculated_map.get(&coord) {
                if *distance > new_dist {
                    calculated_map.insert(coord, (new_dist, direction));
                    to_visit.extend(neighbors(coord, my_height, new_dist));
                }
            } else {
                if height_map.get(&coord).is_some() {
                    calculated_map.insert(coord, (new_dist, direction));
                    to_visit.extend(neighbors(coord, my_height, new_dist));
                }
            }
        }
    }

    let parta = {
        let (distance, _) = calculated_map.get(&start).unwrap();
        *distance
    };

    let partb = {
        let (_, (distance, _)) = height_map
            .iter()
            .filter(|(_coord, height)| **height == 'a' as u32 - ('a' as u32) + 1)
            .filter_map(|(coord, _)| match calculated_map.get(coord) {
                Some(tile) => Some((coord, tile)),
                None => None,
            })
            .min_by_key(|(_coord, (distance, _))| distance)
            .unwrap();
        *distance
    };
    (format!("{parta}"), format!("{partb}"))
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    End,
    North,
    South,
    East,
    West,
}

fn neighbors(
    tile: (i32, i32),
    height: u32,
    distance: u32,
) -> [((i32, i32), (u32, Direction, u32)); 4] {
    let (x, y) = tile;
    let dist = distance + 1;
    [
        ((x - 1, y), (dist, West, height)),
        ((x, y - 1), (dist, North, height)),
        ((x + 1, y), (dist, East, height)),
        ((x, y + 1), (dist, South, height)),
    ]
}

fn print_map(map: &HashMap<(i32, i32), (u32, Direction)>, complete_map: &HashMap<(i32, i32), u32>) {
    let min_x = complete_map.keys().map(|(x, _)| x).min().copied().unwrap();
    let min_y = complete_map.keys().map(|(_, y)| y).min().copied().unwrap();
    let max_x = complete_map.keys().map(|(x, _)| x).max().copied().unwrap();
    let max_y = complete_map.keys().map(|(_, y)| y).max().copied().unwrap();

    for row in min_y..(max_y + 1) {
        for col in min_x..(max_x + 1) {
            let coord = (col, row);
            if let Some((_, direction)) = map.get(&coord) {
                let c = match direction {
                    End => 'E',
                    North => 'V',
                    South => '^',
                    East => '<',
                    West => '>',
                };
                print!("{c}");
            } else {
                let height = complete_map.get(&coord).unwrap();
                let c = ALPHABET.get((*height - 1) as usize).unwrap();
                print!(".");
            }
        }
        println!("");
    }
    println!("");
}

fn print_heightmap(map: &HashMap<(i32, i32), u32>) {
    let min_x = map.keys().map(|(x, _)| x).min().copied().unwrap();
    let min_y = map.keys().map(|(_, y)| y).min().copied().unwrap();
    let max_x = map.keys().map(|(x, _)| x).max().copied().unwrap();
    let max_y = map.keys().map(|(_, y)| y).max().copied().unwrap();

    for row in min_y..(max_y + 1) {
        for col in min_x..(max_x + 1) {
            let coord = (col, row);
            if let Some(height) = map.get(&coord) {
                let c = ALPHABET.get((*height - 1) as usize).unwrap();
                print!("{c}");
            } else {
                print!("?");
            }
        }
        println!("");
    }
    println!("");
}

const ALPHABET: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];
