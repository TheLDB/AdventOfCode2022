use std::collections::HashMap;
use std::fs;

fn part_one() -> i32 {
    // A = Rock
    // B = Paper
    // C = Scissors

    // X = Rock = 1 point
    // Y = Paper = 2 points
    // Z = Scissors = 3 points

    // Loss = 0 points
    // Draw = 3 points
    // Win = 6 points

    let input = fs::read_to_string("./src/input").unwrap();
    let input = input.split("\n");

    let mut total_score: i32 = 0;

    for i in input {
        let mut round_score: i32 = 0;

        let parts = i.split(" ");

        let mut part_one: &str = "";
        let mut part_two: &str = "";

        for (index, el) in parts.enumerate() {
            if index == 0 {
                part_one = el;
            }
            else {
                part_two = el;
            }
        }

        if part_two == "X" {
            round_score += 1;

            if part_one == "A" {
                round_score += 3;
            }
            else if part_one == "C" {
                round_score += 6;
            }
        }
        else if part_two == "Y" {
            round_score += 2;

            if part_one == "A" {
                round_score += 6;
            }
            else if part_one == "B" {
                round_score += 3;
            }
        }
        else if part_two == "Z" {
            round_score += 3;

            if part_one == "B" {
                round_score += 6;
            }
            else if part_one == "C" {
                round_score +=3
            }
        }

        total_score += round_score;
    }

    total_score
}

fn part_two() -> i32 {
    let input = fs::read_to_string("./src/input").unwrap();
    let input = input.split("\n");

    let mut total_score: i32 = 0;

    for i in input {
        let mut round_score: i32 = 0;

        let parts = i.split(" ");

        let mut part_one: &str = "";
        let mut part_two: &str = "";

        for (index, el) in parts.enumerate() {
            if index == 0 {
                part_one = el;
            }
            else {
                part_two = el;
            }
        }

        if part_two == "Z" {
            if part_one == "A" {
                round_score += 8
            }
            else if part_one == "B" {
                round_score += 9
            }
            else if part_one == "C" {
                round_score += 7
            }
        }
        else if part_two == "Y" {
            if part_one == "A" {
                round_score += 4;
            }
            else if part_one == "B" {
                round_score += 5;
            }
            else if part_one == "C" {
                round_score += 6;
            }
        }
        else if part_two == "X" {
            if part_one == "A" {
                round_score += 3;
            }
            else if part_one == "B" {
                round_score += 1;
            }
            else if part_one == "C" {
                round_score += 2;
            }
        }

        total_score += round_score;
    }

    total_score
}

fn main() {
    // let part_one = part_one();
    // println!("{:#?}", part_one);

    let part_two = part_two();
    println!("{}", part_two);
}