use std::collections::HashSet;
use std::env;
use std::fs;

fn part1(target: i32, v: &HashSet<i32>) -> Result<i32, &'static str> {
    match v.iter().find(|&&x| v.contains(&(target - x))) {
        Some(num) => Ok(num * (target - num)),
        None => Err("Failed to find an answer!"),
    }
}

fn part2(v: &HashSet<i32>) -> Result<i32, &'static str> {
    match v.iter().find(|&&x| part1(2020 - x, v).is_ok()) {
        Some(num) => Ok(num * part1(2020 - num, v).unwrap()),
        None => Err("Failed to find an answer!"),
    }
}

fn main() {
    println!("Reading input");

    let contents = fs::read_to_string("input.txt").expect("Failed to read input");

    let mut v: HashSet<i32> = HashSet::new();
    let input_data = contents.split("\n");

    for input in input_data {
        match input.parse::<i32>() {
            Ok(num2) => {
                v.insert(num2);
                ()
            }
            Err(error) => println!("Error {:?}", error),
        }
    }

    let answer1 = part1(2020, &v).unwrap();
    let answer2 = part2(&v).unwrap();
    println!("Part 1: {}\nPart 2: {}", answer1, answer2);
}
