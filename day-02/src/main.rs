use regex::Regex;
use std::fs;

fn find_maximum(line: String, color: String) -> i32 {
    Regex::new(&[r"\s[0-9]+\s", &color.as_str()].concat())
        .unwrap()
        .find_iter(&line)
        .map(|m| {
            m.as_str()
                .replace(&color, "")
                .trim()
                .parse::<i32>()
                .unwrap()
        })
        .max()
        .unwrap()
}

fn get_game_id(line: String) -> i32 {
    Regex::new(r"Game\s[0-9]+")
        .unwrap()
        .find(&line)
        .unwrap()
        .as_str()
        .replace("Game", "")
        .trim()
        .parse::<i32>()
        .unwrap()
}

fn part_1(input: &String) -> i32 {
    let mut sum = 0;

    for line in input.lines() {
        if find_maximum(line.to_string(), "red".to_string()) <= 12
            && find_maximum(line.to_string(), "green".to_string()) <= 13
            && find_maximum(line.to_string(), "blue".to_string()) <= 14
        {
            sum += get_game_id(line.to_string());
        }
    }

    sum
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("error reading file");

    println!("Part 1: {}", part_1(&input));
}
