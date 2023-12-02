use lazy_static::lazy_static;

use regex::Regex;

lazy_static! {
    static ref COLORS_REGEX: Regex = Regex::new(r"(\d+)\s(\w+)").unwrap();
    static ref GAME_REGEX: Regex = Regex::new(r"Game\s(\d+)").unwrap();
}

pub fn solve_part_1(input: &str) -> i32 {
    let mut id_total = 0;
    for line in input.split("\n") {
        let mut parts = line.split(":");
        let game_part = parts.next().unwrap();
        let sets_part = parts.next().unwrap();

        let mut possible = true;
        for set in sets_part.split(";") {
            for capture in COLORS_REGEX.captures_iter(set) {
                let (_, [num, color]) = capture.extract();

                possible = match color {
                    "green" => num.parse::<i32>().unwrap() <= 13,
                    "red" => num.parse::<i32>().unwrap() <= 12,
                    "blue" => num.parse::<i32>().unwrap() <= 14,
                    _ => panic!("invalid color = {}", color),
                };

                if !possible {
                    break;
                }
            }

            if !possible {
                break;
            }
        }

        if possible {
            let cap = GAME_REGEX.captures_iter(game_part).next().unwrap();
            let (_, [id]) = cap.extract();

            id_total += id.parse::<i32>().unwrap();
        }
    }

    return id_total;
}

pub fn solve_part_2(input: &str) -> i32 {
    return input.split("\n").fold(0, |total, line| {
        let sets_part = line.split(":").skip(1).next().unwrap();

        let (min_red, min_blue, min_green) =
            sets_part
                .split(";")
                .fold((i32::MIN, i32::MIN, i32::MIN), |mins, set| {
                    let mut mr = mins.0;
                    let mut mb = mins.1;
                    let mut mg = mins.2;

                    for capture in COLORS_REGEX.captures_iter(set) {
                        let (_, [num, color]) = capture.extract();

                        match color {
                            "green" => {
                                mg = i32::max(mg, num.parse::<i32>().unwrap());
                            }
                            "red" => {
                                mr = i32::max(mr, num.parse::<i32>().unwrap());
                            }
                            "blue" => {
                                mb = i32::max(mb, num.parse::<i32>().unwrap());
                            }
                            _ => panic!("invalid color = {}", color),
                        }
                    }

                    return (mr, mb, mg);
                });

        return total + min_red * min_blue * min_green;
    });
}
