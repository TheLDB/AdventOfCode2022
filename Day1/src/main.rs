use std::fs;

fn part_one() -> i32 {
    // Steps to solution:
    // Parse the file into a string
    // Split it by \n\n, i.e. a line break from value 1 -> 2, then a full line break, resembling an empty line
    // Loop through the Vec, add up the values, and if the total is higher than the last total stored, store the value & index somewhere

    let input = fs::read_to_string("./src/input").unwrap();

    let input_arr = input.split("\n\n");

    let mut current_cal_chunk: i32 = 0;
    let mut highest_cal_chunk: i32 = 0;

    for (index, element) in input_arr.enumerate() {

        for (index, element) in element.split("\n").enumerate() {
            let num_element: i32 = element.parse().unwrap();

            current_cal_chunk = num_element + current_cal_chunk;
        }

        if current_cal_chunk > highest_cal_chunk {
            highest_cal_chunk = current_cal_chunk;

            current_cal_chunk = 0;
        }
        else {
            current_cal_chunk = 0;
        }
    }

    highest_cal_chunk
}

fn part_two() -> i32 {
    // Steps to solution:
    // Parse the file into a string
    // Split it by \n\n
    // Loop through, add the calories, then push them to a Vec<i32>
    // Sort the Vec, then add the top 3 values and return them

    let input = fs::read_to_string("./src/input").unwrap();

    let input_arr = input.split("\n\n");

    let mut cal_vec: Vec<i32> = Vec::new();

    for (_, element) in input_arr.enumerate() {

        let mut total_cals_for_chunk: i32 = 0;

        for(_, element) in element.split("\n").enumerate() {
            let num_element: i32 = element.parse().unwrap();
            total_cals_for_chunk += num_element;
        }

        cal_vec.push(total_cals_for_chunk);
    }

    cal_vec.sort_by(|a, b| b.cmp(a));

    let mut top_3: i32 = 0;

    for i in 0..3 {
        top_3 += cal_vec.get(i).unwrap();
    }

    top_3
}
fn main() {
    let part_one = part_one();
    let part_two = part_two();

    println!("Part One: {}", part_one);
    println!("Part Two: {}", part_two);
}
