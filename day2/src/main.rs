use std::io::{BufRead, BufReader, Split};
use std::fs::File;

const MAX_RED_CUBES: i32 = 12;
const MAX_GREEN_CUBES: i32 = 13;
const MAX_BLUE_CUBES: i32 = 14;

fn is_valid_game(line: &str) -> i32 {
    let mut mut_line = line.to_string();
    let semi_index = match line.find(":") {
        Some(opt) => opt,
        _ => 1,
    };

    if semi_index == 1 {
        return 0;
    }

    // println!("opt: {semi_index}");
    let rounds: String = mut_line.split_off(semi_index+2);    

    // println!("rounds: {rounds}");

    // actual round should be split by ;
    let iter_rounds = rounds.split_whitespace();
    let mut cur_fetch_count = 0;
    for round in iter_rounds {
        println!("{round}");
        let round_as_int = round.parse::<i32>();
        if round_as_int.is_ok() {
            cur_fetch_count = round_as_int.unwrap();
        } else {
            match round {
                _ if round.contains("red") => {
                    if cur_fetch_count > MAX_RED_CUBES {
                        return 0;
                    }
                }
                _ if round.contains("blue") => {
                    if cur_fetch_count > MAX_BLUE_CUBES {
                        return 0;
                    }
                }
                _ if round.contains("green") => {
                    if cur_fetch_count > MAX_GREEN_CUBES {
                        return 0;
                    }
                }
                _ => return 1,
            }
        }
    }

    return 1;
}

fn get_power_set(line: &str) -> i64 {
    let mut mut_line = line.to_string();
    let semi_index = match line.find(":") {
        Some(opt) => opt,
        _ => 1,
    };

    if semi_index == 1 {
        return 0;
    }

    // println!("opt: {semi_index}");
    let rounds: String = mut_line.split_off(semi_index+2);    

    // [red, green, blue]
    let mut min_cubes_necessary = [0, 0, 0];

    // println!("rounds: {rounds}");

    // actual round should be split by ;
    let iter_rounds = rounds.split_whitespace();
    let mut cur_fetch_count = 0;
    for round in iter_rounds {
        println!("{round}");
        let round_as_int = round.parse::<i32>();
        if round_as_int.is_ok() {
            cur_fetch_count = round_as_int.unwrap();
        } else {
            match round {
                _ if round.contains("red") => {
                    if cur_fetch_count > min_cubes_necessary[0]{
                        min_cubes_necessary[0] = cur_fetch_count;
                    }
                }
                _ if round.contains("green") => {
                    if cur_fetch_count > min_cubes_necessary[1] {
                        min_cubes_necessary[1] = cur_fetch_count;
                    }
                }
                _ if round.contains("blue") => {
                    if cur_fetch_count > min_cubes_necessary[2] {
                        min_cubes_necessary[2] = cur_fetch_count;
                    }
                }
                _ => continue,
            }
        }
    }

    return (min_cubes_necessary[0] * min_cubes_necessary[1] * min_cubes_necessary[2]).into();
}

fn main() {
    let ifile = File::open("src/input.txt").unwrap();
    let ifile = BufReader::new(ifile);
    let mut sum = 0;
    // let mut mult = 1;
    for line in ifile.lines() {
        // sum += is_valid_game(&line.unwrap()) * mult;
        //mult += 1;

        sum += get_power_set(&line.unwrap());
    }

    println!("sum: {sum}");
}
