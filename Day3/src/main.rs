use std::{fs, io};
use std::collections::HashMap;
use std::fs::File;
use std::hash::Hash;
use std::io::{BufRead, BufReader};

fn read_input() -> BufReader<File> {
    let file = fs::File::open("./src/input").unwrap();
    let reader = io::BufReader::new(file);

    reader
}

fn part_one() -> i32 {
    let input = read_input();

    let mut shared_items: Vec<String> = Vec::new();

    for line in input.lines() {
        if let Ok(line) = line {
            // line = 1 Rucksack
            // need to split the Rucksack in half to get both compartments
            let line_length = line.len() / 2;
            let (left, right) = line.split_at(line_length);

            let left_split = left.split("");
            let left_split = left_split.collect::<Vec<&str>>();

            let mut blacklisted_items: Vec<&str> = Vec::new();

            for i in left_split {
                if right.contains(i) && i != "" && !blacklisted_items.contains(&i) {
                    shared_items.push(i.to_string());
                    blacklisted_items.push(i);
                }
            }
        }

    }

    // Prioritize the lines
    let mut priority_sum: i32 = 0;

    let alphabet: Vec<&str> = vec!["a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z",
                                   "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z"];

    for i in shared_items {
        let priority_index = alphabet.iter().position(|&r| r == i).unwrap() + 1;
        println!("Letter: {}, Index: {}", i, priority_index);
        priority_sum += priority_index as i32;
    }

    priority_sum
}

fn part_two() -> i32 {
    let input = read_input();

    let mut all_chunks: Vec<String> = Vec::new();

    let mut chunk: String = "".to_string();

    let mut iterator: i32 = 0;

    for (index, element) in input.lines().enumerate() {
        // Split the lines up into 3 chunks
        if let Ok(line) = element {
            if iterator == 2 {
                chunk.push_str(format!("{}\n", line).as_str());
                all_chunks.push(chunk.clone());
                chunk = "".to_string();
                iterator = 0;
            }
            else {
                chunk.push_str(format!("{}\n", line).as_str());
                iterator += 1;
            }
        }
    }

    let mut shared_items: Vec<String> = Vec::new();

    for i in all_chunks {
        // Split chunk up into 3 lines
        let chunks = i.split("\n");

        let mut chunk_one: String = "".to_string();
        let mut chunk_two: String = "".to_string();
        let mut chunk_three: String = "".to_string();

        for (index, element) in chunks.enumerate() {
            if index == 0 {
                chunk_one = element.to_string();
            }
            else if index == 1 {
                chunk_two = element.to_string()
            }
            else if index == 2 {
                chunk_three = element.to_string()
            }
        }

        let mut blacklisted_items: Vec<&str> = Vec::new();

        for i in chunk_one.split("") {
            if chunk_two.contains(i) && chunk_three.contains(i) && i != "" && !blacklisted_items.contains(&i) {
                shared_items.push(i.to_string());
                blacklisted_items.push(i);
            }
        }
    }

    let mut priority_sum: i32 = 0;

    let alphabet: Vec<&str> = vec!["a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z",
                                   "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z"];

    for i in shared_items {
        let priority_index = alphabet.iter().position(|&r| r == i).unwrap() + 1;
        println!("Letter: {}, Index: {}", i, priority_index);
        priority_sum += priority_index as i32;
    }

    priority_sum
}

fn main() {
    // let part_one = part_one();
    // println!("{:#?}", part_one);

    let part_two = part_two();
    println!("{:#?}", part_two);
}
