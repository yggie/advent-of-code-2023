use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet};

lazy_static! {
    static ref NOT_DOT_REGEX: Regex = Regex::new(r"(\d+|[^\.\s\d]+)").unwrap();
}

pub fn solve_part_1(input: &str) -> i32 {
    let mut numbers = HashMap::new();
    let mut symbols = Vec::<(usize, char)>::new();

    let mut first_num: Option<usize> = None;
    for (i, c) in input.char_indices() {
        if c == '.' || c == '\n' {
            first_num = None;
        } else if c.is_digit(10) {
            match first_num {
                None => {
                    first_num = Some(i);
                    numbers.insert(i, i);
                }

                Some(j) => {
                    numbers.insert(i, j);
                }
            }
        } else {
            first_num = None;
            symbols.push((i, c));
        }
    }

    let width = input.chars().position(|c| c == '\n').unwrap() + 1;
    let length = input.chars().count();
    let height = length / width + if length % width == 0 { 0 } else { 1 };
    let mut number_indices = HashSet::<usize>::new();

    for (idx, _) in symbols {
        let row_number = idx / width;
        let vertical_offset = if row_number == 0 {
            0i32..=1
        } else if row_number == height {
            -1..=0
        } else {
            -1..=1
        };

        let col_number = idx % width;
        let horizontal_offset = if col_number == 0 {
            0i32..=1
        } else if col_number == width {
            -1..=0
        } else {
            -1..=1
        };

        for i in vertical_offset {
            for j in horizontal_offset.clone() {
                let index = i * (width as i32) + j + (idx as i32);

                if ((index + 1) as usize) % width == 0 || index as usize >= length {
                    // this column will only contain newlines
                    continue;
                }

                match numbers.get(&(index as usize)) {
                    Some(start_idx) => {
                        number_indices.insert(*start_idx);
                    }
                    None => (),
                }
            }
        }
    }

    return number_indices.iter().fold(0, |total, lookup| {
        let i = numbers.get(lookup).unwrap();
        let val = input
            .chars()
            .skip(*i)
            .take_while(|c| c.is_digit(10))
            .collect::<String>()
            .parse::<i32>()
            .unwrap();

        return total + val;
    });
}

pub fn solve_part_2(input: &str) -> i32 {
    let mut numbers = HashMap::new();
    let mut maybe_gears = Vec::<usize>::new();

    let mut first_num: Option<usize> = None;
    for (i, c) in input.char_indices() {
        if c.is_digit(10) {
            match first_num {
                None => {
                    first_num = Some(i);
                    numbers.insert(i, i);
                }

                Some(j) => {
                    numbers.insert(i, j);
                }
            }
        } else {
            first_num = None;

            if c == '*' {
                maybe_gears.push(i);
            }
        }
    }

    let width = input.chars().position(|c| c == '\n').unwrap() + 1;
    let length = input.chars().count();
    let height = length / width + if length % width == 0 { 0 } else { 1 };
    let index_to_num = |i: usize| {
        return input
            .chars()
            .skip(i)
            .take_while(|c| c.is_digit(10))
            .collect::<String>()
            .parse::<i32>()
            .unwrap();
    };

    return maybe_gears.iter().fold(0, |total, idx| {
        let row_number = idx / width;
        let vertical_offset = if row_number == 0 {
            0i32..=1
        } else if row_number == height {
            -1..=0
        } else {
            -1..=1
        };

        let col_number = idx % width;
        let horizontal_offset = if col_number == 0 {
            0i32..=1
        } else if col_number == width {
            -1..=0
        } else {
            -1..=1
        };

        let mut adjacent_nums = HashSet::new();
        for i in vertical_offset {
            for j in horizontal_offset.clone() {
                let index = i * (width as i32) + j + (*idx as i32);

                if ((index + 1) as usize) % width == 0 || index as usize >= length {
                    // this column will only contain newlines
                    continue;
                }

                match numbers.get(&(index as usize)) {
                    Some(start_idx) => {
                        adjacent_nums.insert(*start_idx);
                    }
                    None => (),
                }
            }
        }

        return total
            + if adjacent_nums.len() == 2 {
                adjacent_nums.iter().fold(1, |t, &v| t * index_to_num(v)) as i32
            } else {
                0
            };
    });
}
