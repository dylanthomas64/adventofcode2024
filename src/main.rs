use std::fs;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

fn main() {
    println!("day5 == {}", day5::run(parse_text()))
}

fn parse_text() -> String {
    fs::read_to_string("input.txt").expect("couldn't read text input")
}
