pub const INPUTS: &[&str] = &[INPUT, INPUT_A];
pub const INPUT: &str = include_str!("../../input/day8.txt");

pub const INPUT_A: &str = "30373
25512
65332
33549
35390";

#[inline]
pub fn run(input: &str) -> (String, String) {
    let grove = input
        .lines()
        .enumerate()
        .flat_map(|(row, line)| -> Vec<(usize, usize, u8)> {
            line.chars()
                .enumerate()
                .map(|(column, c)| (row, column, c.to_string().parse::<u8>().unwrap()))
                .collect()
        })
        .collect::<Vec<(usize, usize, u8)>>();

    let parta: usize = grove
        .iter()
        .filter(|(row, col, height)| {
            let taller_than_height = taller_than(*height);
            let others: Vec<_> = grove
                .iter()
                .filter(|(other_row, other_col, _)| row == other_row || col == other_col)
                .collect();
            let above = others
                .iter()
                .filter(|(other_row, other_col, _)| other_col == col && other_row < row)
                .filter(&taller_than_height)
                .count();
            let below = others
                .iter()
                .filter(|(other_row, other_col, _)| other_col == col && other_row > row)
                .filter(&taller_than_height)
                .count();
            let left = others
                .iter()
                .filter(|(other_row, other_col, _)| other_row == row && other_col < col)
                .filter(&taller_than_height)
                .count();
            let right = others
                .iter()
                .filter(|(other_row, other_col, _)| other_row == row && other_col > col)
                .filter(&taller_than_height)
                .count();
            above == 0 || below == 0 || left == 0 || right == 0
        })
        .count();
    let partb = grove
        .iter()
        .map(|(row, col, height)| {
            let others: Vec<_> = grove
                .iter()
                .filter(|(other_row, other_col, _)| row == other_row || col == other_col)
                .collect();
            let mut above: Vec<_> = others
                .iter()
                .filter(|(other_row, other_col, _)| other_col == col && other_row < row)
                .map(|(row, _col, height)| (row, height))
                .collect();
            above.sort_by_key(|a| a.0);
            above.reverse();
            let mut below: Vec<_> = others
                .iter()
                .filter(|(other_row, other_col, _)| other_col == col && other_row > row)
                .map(|(row, _col, height)| (row, height))
                .collect();
            below.sort_by_key(|a| a.0);
            let mut left: Vec<_> = others
                .iter()
                .filter(|(other_row, other_col, _)| other_row == row && other_col < col)
                .map(|(_row, col, height)| (col, height))
                .collect();
            left.sort_by_key(|a| a.0);
            left.reverse();
            let mut right: Vec<_> = others
                .iter()
                .filter(|(other_row, other_col, _)| other_row == row && other_col > col)
                .map(|(_row, col, height)| (col, height))
                .collect();
            right.sort_by_key(|a| a.0);
            let count_above = {
                let mut i = 0;
                
                while i < above.len() && above.get(i).unwrap().1 < height {
                    // if *row == 2 && *col == 1 {
                    //     println!("found 1 above of height {}", above.get(i).unwrap().1);
                    // }
                    i+=1;
                }
                if i < above.len() {
                    i+=1;
                }
                // if *row == 2 && *col == 1 {
                //     println!("above: {i}");
                // }
                i
            };
            let count_below = {
                let mut i = 0;
                
                while i < below.len() && below.get(i).unwrap().1 < height {
                    // if *row == 2 && *col == 1 {
                    //     println!("found 1 below of height {}", below.get(i).unwrap().1);
                    // }
                    i+=1;
                }
                if i < below.len() {
                    i+=1;
                }
                // if *row == 2 && *col == 1 {
                //     println!("below: {i}");
                // }
                i
            };
            let count_left = {
                let mut i = 0;
                
                while i < left.len() && left.get(i).unwrap().1 < height {
                    // if *row == 2 && *col == 1 {
                    //     println!("found 1 left of height {}", left.get(i).unwrap().1);
                    // }
                    i+=1;
                }
                if i < left.len() {
                    i+=1;
                }
                // if *row == 2 && *col == 1 {
                //     println!("left: {i}");
                // }
                i
            };
            let count_right = {
                let mut i = 0;
                
                while i < right.len() && right.get(i).unwrap().1 < height {
                    // if *row == 2 && *col == 1 {
                    //     println!("found 1 right of height {}", right.get(i).unwrap().1);
                    // }
                    i+=1;
                }
                if i < right.len() {
                    i+=1;
                }
                // if *row == 2 && *col == 1 {
                //     println!("right: {i}");
                // }
                i
            };
            count_above * count_below * count_left * count_right
        })
        .max()
        .unwrap();
    (format!("{parta}"), format!("{partb:?}"))
}

fn taller_than(height: u8) -> Box<dyn Fn(&&&(usize, usize, u8)) -> bool> {
    Box::new(move |(_, _, other_height)| *other_height >= height)
}
