use regex::Regex;
use std::fs;

fn remove_alphabetical_characters(string: &String) -> String {
    let re = Regex::new("[a-z]").unwrap();

    Regex::replace_all(&re, string, "").to_string()
}

fn convert_outer_digits_to_number(string: &String) -> i32 {
    let first_digit = string.chars().next().unwrap().to_string();
    let last_digit = string.chars().last().unwrap().to_string();

    [first_digit, last_digit].join("").parse::<i32>().unwrap()
}

fn part_1(input: &String) -> String {
    let input_without_text = remove_alphabetical_characters(&input.to_string());
    let lines_with_numbers = input_without_text.lines();

    let numbers = lines_with_numbers.map(|line| convert_outer_digits_to_number(&line.to_string()));

    numbers.into_iter().sum::<i32>().to_string()
}

fn split_concatenated_numbers(line: &String) -> String {
    line.replace("oneight", "oneeight")
        .replace("fiveight", "fiveeight")
        .replace("nineight", "nineeight")
        .replace("twone", "twoone")
        .replace("sevenine", "sevennine")
        .replace("eightwo", "eighttwo")
}

fn transform_words_to_numbers(line: &String) -> String {
    let spelled_numbers_list: Vec<_> = [
        ["one", "1"],
        ["two", "2"],
        ["three", "3"],
        ["four", "4"],
        ["five", "5"],
        ["six", "6"],
        ["seven", "7"],
        ["eight", "8"],
        ["nine", "9"],
    ]
    .into();

    let mut transformed_line: String = line.to_string();

    for i in 0..line.len() {
        let mut has_replaced_first = false;
        let (first, _last) = line.split_at(i);

        for j in 0..spelled_numbers_list.len() {
            if let Some(_part) = first.find(spelled_numbers_list[j][0]) {
                transformed_line =
                    line.replace(spelled_numbers_list[j][0], spelled_numbers_list[j][1]);
                has_replaced_first = true;
            }
        }

        if has_replaced_first {
            break;
        }
    }

    let mut transformed_line2: String = transformed_line.to_string();

    for i in 0..transformed_line.len() {
        let mut has_replaced_last = false;
        let (_first, last) = transformed_line.split_at(transformed_line.len() - i - 1);

        for j in 0..spelled_numbers_list.len() {
            if let Some(_part) = last.find(spelled_numbers_list[j][0]) {
                transformed_line2 = transformed_line
                    .replace(spelled_numbers_list[j][0], spelled_numbers_list[j][1]);
                has_replaced_last = true;
            }
        }

        if has_replaced_last {
            break;
        }
    }

    transformed_line2.to_string()
}

fn part_2(input: &String) -> String {
    let lines = input.lines();

    let numbers = lines.map(|line| {
        let split_line = split_concatenated_numbers(&line.to_string());
        let transformed_line = transform_words_to_numbers(&split_line.to_string());
        let line_without_text = remove_alphabetical_characters(&transformed_line.to_string());
        let number = convert_outer_digits_to_number(&line_without_text);

        println!("{} -> {}", line, number.to_string());

        number
    });

    numbers.into_iter().sum::<i32>().to_string()
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("error reading file");

    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}
