use std::collections::HashSet;
use std::io::{BufRead, BufReader};
use std::fs::File;


fn main() {
    let ifile = File::open("src/input.txt").unwrap();
    let ifile = BufReader::new(ifile);
    let mut schematic: Vec<Vec<char>> = Vec::new();
    for line in ifile.lines() {
        schematic.push(line.unwrap().chars().collect());
    }

    // let sum = get_sum_of_part_numbers(&schematic);

    let sum = get_gear_ratio(&schematic);

    println!("sum: {sum}");
}

fn get_gear_ratio(schematic: &Vec<Vec<char>>) -> i32 {
    let mut sum = 0;
    let mut potential_gear_ratios = std::iter::repeat::<Vec<u32>>(vec![]).take(50000).collect::<Vec<_>>();
    for i in 0..schematic.len() {
        let mut stack = Vec::<u32>::new();
        let mut gear_index = Vec::<u32>::new();
        let mut is_gear_ratio = false;
        for j in 0..schematic[i].len() {
            let cur_char = schematic[i][j];
            if cur_char.is_digit(10) {
                match cur_char.to_digit(10) {
                    Some(digit) => stack.push(digit),
                    None => println!("DJSAKLDJSA"),
                }

                let potential_gear_ratio = has_nearby_gear(schematic, i as i32, j as i32, &mut gear_index);

                // println!("{:#?} {:#?}", schematic, stack);
                is_gear_ratio = is_gear_ratio || potential_gear_ratio;
                //println!("{:#?} {:#?} {:#?} {is_cur_part_number}", schematic[i][j], i, j);

                if is_gear_ratio && j == schematic[i].len()-1 {
                    let addend = stack.iter().fold(0, |acc, elem| acc * 10 + elem);
                    println!("{addend}");
                    while let Some(gear_idx) = gear_index.pop() {
                        potential_gear_ratios[gear_idx as usize].push(addend);
                    }
                    is_gear_ratio = false;
                    stack.clear();
                    gear_index.clear();
                }
            } else {
                if is_gear_ratio {
                    // println!("{:#?}", stack);
                    let addend = stack.iter().fold(0, |acc, elem| acc * 10 + elem);
                    // println!("{addend}");
                    while let Some(gear_idx) = gear_index.pop() {
                        potential_gear_ratios[gear_idx as usize].push(addend);
                    }
                } 

                gear_index.clear();
                stack.clear();
                is_gear_ratio = false;
            }
        }
    }

    for gear_ratio in &mut potential_gear_ratios {
        let set: HashSet<_> = gear_ratio.drain(..).collect(); // dedup
        gear_ratio.extend(set.into_iter());
    }
    
    for mut gear_ratio in potential_gear_ratios {
        if gear_ratio.len() == 2 {
            let addend1 = gear_ratio.pop().unwrap();
            let addend2 = gear_ratio.pop().unwrap();
            sum += addend1 * addend2;
        }
    }


    return sum.try_into().unwrap(); 
}

fn has_nearby_gear(schematic: &Vec<Vec<char>>, i: i32, j: i32, gear_idx: &mut Vec<u32>) -> bool {
    let directions: [(i32, i32); 8] = [(0, -1), (-1, -1), (-1, 0), (-1, 1), (0, 1), (1, 1), (1, 0), (1, -1)];
    let mut has_gear = false;

    for dir in directions {
        let i_prime = i + dir.0;
        let j_prime = j + dir.1;

        if i_prime < 0 || i_prime >= schematic.len() as i32 || j_prime < 0 || j_prime >= schematic[i as usize].len() as i32 {
            continue
        }
        match schematic[i_prime as usize][j_prime as usize] == '*' {
            true => {
                let mut key;
                if i_prime >= j_prime {
                    key = i_prime * i_prime + i_prime + j_prime;
                } else {
                    key = i_prime + j_prime * j_prime;
                }
                gear_idx.push(key as u32);
                has_gear = true;
            }
            false => continue,
        };
    }

    return has_gear;

}

fn is_symbol(char: &char) -> bool {
    // println!("{char}");
    !(char.is_digit(10) || *char == '.')
}

fn is_part_number(schematic: &Vec<Vec<char>>, i: i32, j: i32) -> bool {
    let directions: [(i32, i32); 8] = [(0, -1), (-1, -1), (-1, 0), (-1, 1), (0, 1), (1, 1), (1, 0), (1, -1)];

    for dir in directions {
        let i_prime = i + dir.0;
        let j_prime = j + dir.1;

        if i_prime < 0 || i_prime >= schematic.len() as i32 || j_prime < 0 || j_prime >= schematic[i as usize].len() as i32 {
            continue
        }
        // Szudzik's function bcuz i havent learned hash maps in rust yet lol ;p
        // https://stackoverflow.com/questions/919612/mapping-two-integers-to-one-in-a-unique-and-deterministic-way
        match is_symbol(&schematic[i_prime as usize][j_prime as usize]) {
            true => return true,
            false => continue,
        };
    }

    return false;
}

fn get_sum_of_part_numbers(schematic: &Vec<Vec<char>>) -> i32 {
    let mut sum = 0;
    for i in 0..schematic.len() {
        let mut stack = Vec::<u32>::new();
        let mut is_cur_part_number = false;
        for j in 0..schematic[i].len() {
            let cur_char = schematic[i][j];
            if cur_char.is_digit(10) {
                match cur_char.to_digit(10) {
                    Some(digit) => stack.push(digit),
                    None => println!("DJSAKLDJSA"),
                }

                // println!("{:#?} {:#?}", schematic, stack);
                is_cur_part_number = is_cur_part_number || is_part_number(schematic, i as i32, j as i32);
                //println!("{:#?} {:#?} {:#?} {is_cur_part_number}", schematic[i][j], i, j);

                if (is_cur_part_number && j == schematic[i].len()-1) {
                    let addend = stack.iter().fold(0, |acc, elem| acc * 10 + elem);
                    // println!("{addend}");
                    sum += addend;     
                    stack.clear();
                    is_cur_part_number = false;
                }
            } else {
                if is_cur_part_number {
                    // println!("{:#?}", stack);
                    let addend = stack.iter().fold(0, |acc, elem| acc * 10 + elem);
                    // println!("{addend}");
                    sum += addend;     
                } 

                stack.clear();
                is_cur_part_number = false;
            }
        }
    }

    return sum.try_into().unwrap(); 
}
