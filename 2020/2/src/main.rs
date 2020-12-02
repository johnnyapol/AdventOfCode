use std::env;
use std::fs;



fn main() {
    println!("Reading input");

    let contents = fs::read_to_string("input.txt").expect("Failed to read input");
    let mut answer1 = 0;
    let mut answer2 = 0;

    // Part1
    for input in contents.lines()
    {
        println!("Input: {}", input);
        let split: Vec<&str> = input.split(" ").collect();

        // split[0] = min-max
        // split[1] = char:
        // split[2] = passwd

        let minmax: Vec<&str> = split[0].split("-").collect();
        let min = minmax[0].parse::<usize>().unwrap();
        let max = minmax[1].parse::<usize>().unwrap();

        println!("Min: {}, Max: {}", min, max);

        let character = split[1].chars().nth(0).unwrap();
        println!("Target char: {}", character);

        let mut count = 0;
        for c in split[2].chars()
        {
            if (c == character)
            {
                count += 1;
            }
        }

        if (count >= min && count <= max)
        {
            answer1 += 1;
        }

        let first_char = split[2].chars().nth(min-1).unwrap() == character;
        let second_char = match split[2].chars().nth(max-1)
        {
            Some(c2) => c2 == character,
            None => false,
        };
        
        if (first_char ^ second_char)
        {
            answer2 += 1;
        }
    }

    println!("Part 1: {}\nPart 2: {}", answer1, answer2);
}
