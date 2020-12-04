use std::fs;
#[macro_use]
extern crate lazy_static;
extern crate regex;
use regex::Regex;

fn in_range(num: i32, lower: i32, upper: i32) -> bool {
    return lower <= num && num <= upper;
}

fn is_valid(passport: &str) -> bool {
    lazy_static! {
        static ref BYR_RE: Regex = Regex::new(r"byr:\d{4}").unwrap();
        static ref IYR_RE: Regex = Regex::new(r"iyr:\d{4}").unwrap();
        static ref EYR_RE: Regex = Regex::new(r"eyr:\d{4}").unwrap();
        static ref HGT_RE: Regex = Regex::new(r"hgt:\d*(in|cm)").unwrap();
        static ref HCL_RE: Regex = Regex::new(r"hcl:#[0-9a-f]{6}").unwrap();
        static ref EYE_RE: Regex = Regex::new(r"ecl:(amb|blu|brn|gry|grn|hzl|oth)").unwrap();
        static ref PID_RE: Regex = Regex::new(r"pid:\b\d{9}\b").unwrap();
    }

    let validators = [
        match BYR_RE.find(passport) {
            Some(m) => in_range(
                passport[m.start() + 4..m.end()].parse::<i32>().unwrap(),
                1920,
                2002,
            ),
            None => false,
        },
        match IYR_RE.find(passport) {
            Some(m) => in_range(
                passport[m.start() + 4..m.end()].parse::<i32>().unwrap(),
                2010,
                2020,
            ),
            None => false,
        },
        match EYR_RE.find(passport) {
            Some(m) => in_range(
                passport[m.start() + 4..m.end()].parse::<i32>().unwrap(),
                2020,
                2030,
            ),
            None => false,
        },
        match HGT_RE.find(passport) {
            Some(m) => {
                let val = passport[m.start() + 4..m.end() - 2].parse::<i32>().unwrap();
                let unit = &passport[m.end() - 2..m.end()];

                if unit.eq("cm") {
                    in_range(val, 150, 193)
                } else {
                    in_range(val, 59, 76)
                }
            }
            None => false,
        },
        HCL_RE.is_match(passport),
        EYE_RE.is_match(passport),
        PID_RE.is_match(passport),
    ];

    return validators.iter().all(|&x| x == true);
}
fn main() {
    let contents = fs::read_to_string("input.txt").expect("Failed to read input");
    let input_data = contents.split("\n\n");
    // println!("{:#?}", input_data)

    let mut valid = 0;

    for passport in input_data {
        let pass = String::from(passport).replace("\n", " ");

        if is_valid(&pass) {
            valid += 1;
        }
    }

    println!("Valid: {}", valid);
}
