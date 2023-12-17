use std::io::{BufRead, BufReader};
use std::fs::File;

fn reverse_string(input: &str) -> String {
    input.chars().rev().collect()
}

fn convert_to_number(str: &str) -> i32 {
    if str.contains("two") {
        return 2;
    }

    return match str {
        _ if str.contains("one") => 1,
        _ if str.contains("two") => 2,
        _ if str.contains("three") => 3,
        _ if str.contains("four") => 4,
        _ if str.contains("five") => 5,
        _ if str.contains("six") => 6,
        _ if str.contains("seven") => 7,
        _ if str.contains("eight") => 8,
        _ if str.contains("nine") => 9,
        _ => -1,
    }
}

fn get_number(line: String) -> i32 {
    let mut line_chr = line.chars();
    let mut first_numeric_found = String::new();
    let mut stack = Vec::<String>::new();
    while first_numeric_found.is_empty() {
        let i = line_chr.next(); 

        if i.is_none() {
            break;
        }

        let i_str = i.unwrap().to_string();

        if i_str.trim().parse::<i32>().is_ok() && first_numeric_found.is_empty() {
            first_numeric_found = i_str.clone();
        } else {
            stack.push(i_str.to_string());
        }

        let potential_numerical_word = &stack.join("");
        let potential_number = convert_to_number(potential_numerical_word);

        if potential_number != -1 {
            first_numeric_found = potential_number.to_string();
        }
    }

    stack.clear();
    let mut line_chr_rev = line.chars().rev();
    let mut second_numeric_found = String::new();
    while second_numeric_found.is_empty() {
        let j = line_chr_rev.next();

        if j.is_none() {
            break;
        }

        let j_str = j.unwrap().to_string();

        if j_str.trim().parse::<i32>().is_ok() && second_numeric_found.is_empty() {
            second_numeric_found = j_str;
        } else {
            stack.push(j_str);
        }

        let potential_numerical_word = &reverse_string(&stack.join(""));
        let potential_number = convert_to_number(potential_numerical_word);

        if potential_number != -1 {
            second_numeric_found = potential_number.to_string();
        }
    }

    return (first_numeric_found + &second_numeric_found).parse::<i32>().unwrap();
}

fn main() {
    let ifile = File::open("src/input.txt").unwrap();
    let ifile = BufReader::new(ifile);
    let mut sum = 0;

    for line in ifile.lines() {
        sum += get_number(line.unwrap());
    }

    println!("sum: {sum}");
}
