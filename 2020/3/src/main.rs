use std::env;
use std::fs;

fn get_slope_amount(slope_y : usize, slope_x : usize, input : &Vec<&str>) -> usize
{
    let mut answer = 0;
    let mut pos_x = 0;
    let mut pos_y = 0;

    let x_size = input.len();
    let y_size = input[0].chars().count();

    loop
    {
        pos_x += slope_x;
        pos_y += slope_y;

        if pos_x >= x_size
        {
            break;
        }

        if input[pos_x].chars().nth(pos_y % y_size).unwrap() == '#'
        {
            answer += 1;
        }
    }
    
    return answer;
}

fn main() {
    println!("Reading input");

    let contents = fs::read_to_string("input.txt").expect("Failed to read input");
    let input = contents.lines().collect();
    let answer1 = get_slope_amount(3,1,&input);
    let answer2 = vec![(1,1), (3,1), (5,1), (7,1), (1,2)].iter().fold(1, |mut product, &x| product * get_slope_amount(x.0, x.1, &input));
    println!("Part 1: {}\nPart 2: {}", answer1, answer2);
}
