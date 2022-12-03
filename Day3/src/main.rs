use std::{fs, io};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_input() -> BufReader<File> {
    let file = fs::File::open("./src/input").unwrap();
    let reader = io::BufReader::new(file);

    reader
}

fn part_one() {
    let input = read_input();

    for line in input.lines() {
        if let Ok(line) = line {

        }
    }
}

fn part_two() {
    let input = read_input();

    for line in input.lines() {
        if let Ok(line) = line {

        }
    }
}

fn main() {
    let part_one = part_one();
    println!("{:#?}", part_one);

    let part_two = part_two();
    println!("{:#?}", part_two);
}
