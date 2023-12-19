use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let ifile = File::open("src/input.txt").unwrap();
    let ifile = BufReader::new(ifile);
    let mut cards: Vec<Vec<String>> = Vec::new();
    for line in ifile.lines() {
        cards.push(
            line.unwrap()
                .split(" ")
                .map(|s| s.to_string())
                .filter(|s| !s.is_empty())
                .collect(),
        );
    }

    // println!("{:?}", get_total_points(cards));
    println!("{:?}", get_total_points_pt2(cards));
}

fn clean_card(card: &mut Vec<String>) -> () {
    let semi_index = match card.iter().position(|c| c.contains(":")) {
        Some(opt) => opt,
        _ => return,
    };

    *card = card.split_off(semi_index + 1);
}

fn increment_points(points: &mut i32) -> () {
    if *points == 0 {
        *points = 1;
    } else {
        *points = *points * 2;
    }
}

fn get_total_points(cards: Vec<Vec<String>>) -> i32 {
    let mut total_points = 0;
    for mut card in cards.clone() {
        clean_card(&mut card);
        let mut is_user_card = false;
        let mut winning_numbers: HashSet<i32> = HashSet::new();
        let mut cur_points = 0;
        for element in card {
            if let Ok(num) = element.parse::<i32>() {
                if is_user_card {
                    if winning_numbers.contains(&num) {
                        increment_points(&mut cur_points);
                    }
                } else {
                    winning_numbers.insert(num);
                }
            } else {
                is_user_card = true;
            }
        }

        println!("{cur_points}, {:?}", winning_numbers);

        total_points += cur_points;
    }

    total_points
}

fn get_total_points_pt2(cards: Vec<Vec<String>>) -> i32 {
    let mut total_points = 0;
    let mut card_count_by_id: HashMap<usize, i32> = HashMap::new();
    for (i, card) in cards.clone().iter_mut().enumerate() {
        clean_card(card);
        let mut is_user_card = false;
        let mut winning_numbers: HashSet<i32> = HashSet::new();
        let mut count = 0;
        for element in card {
            if let Ok(num) = element.parse::<i32>() {
                if is_user_card {
                    if winning_numbers.contains(&num) {
                        count += 1
                    }
                } else {
                    winning_numbers.insert(num);
                }
            } else {
                is_user_card = true;
            }
        }

        let cur_card_count = card_count_by_id.entry(i).or_insert(0);
        *cur_card_count += 1;

        let mult = cur_card_count.clone();
        // increment for itself and the copies
        for j in 1..count + 1 {
            println!("{i} {j} {mult}");
            if i + j >= cards.len() {
                break;
            }
            let card_count = card_count_by_id.entry(i + j).or_insert(0);
            // how many cards do we have of current card and multiply
            *card_count += 1 * mult;
        }
    }

    println!("{:?}", card_count_by_id);

    for (_, card_count) in &card_count_by_id {
        total_points += card_count;
    }

    total_points
}
