use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn part_one_parse_line(line: String) -> i32 {
    let mut first_num = -1;
    let mut last_num = -1;
    for c in line.chars() {
        if c.is_digit(10) {
            // Print the character if it's a digit
            let num = (c.to_string()).parse::<i32>().unwrap();
            if first_num == -1 {
                first_num = num;
            }
            last_num = num;
        }
    }
    return first_num * 10 + (last_num);
}

fn part_one() -> io::Result<i32> {
    // Open a file in read-only mode
    let file = File::open("input.main.txt")?;
    let reader = BufReader::new(file);
    let mut sum = 0;

    // Iterate over lines and print each line
    for line in reader.lines() {
        let num = part_one_parse_line(line?);
        sum += num;
    }
    return Ok(sum);
}

fn part_two_parse_line(line: String, numbers: &HashMap<&str, u8>) -> u32 {
    let mut char_index: Vec<(u64, u8)> = Vec::new();
    let mut i = 0;

    for c in line.chars() {
        if c.is_digit(10) {
            // Print the character if it's a digit
            let num = (c.to_string()).parse::<u8>().unwrap();
            char_index.push((i, num));
        }
        i += 1;
    }

    for num_str in numbers.keys() {
        let found: Vec<u8> = line.match_indices(num_str).map(|(i, _)| i as u8).collect();
        for f in found {
            // push the index of the number and the converted value
            let val = (f as u64, *numbers.get(num_str).unwrap());
            char_index.push(val);
        }
    }

    char_index.sort_by(|a, b| a.0.cmp(&b.0));

    if char_index.len() == 0 {
        return 0;
    } else {
        let first_num = char_index[0].1;
        let last_num = char_index[char_index.len() - 1].1;
        println!("{} {}", first_num, last_num);
        return first_num as u32 * 10 + (last_num as u32);
    }
}

fn part_two() -> io::Result<u128> {
    let numbers: HashMap<&str, u8> = HashMap::from([
        ("zero", 0),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);

    // Open a file in read-only mode
    let file = File::open("input.part2.main.txt")?;
    let reader = BufReader::new(file);
    let mut sum: u128 = 0;

    // Iterate over lines and print each line
    for line in reader.lines() {
        let num = part_two_parse_line(line?, &numbers);
        sum += num as u128;
    }
    return Ok(sum);
}

fn main() -> io::Result<()> {
    // Open a file in read-only mode
    let sum = part_two();
    println!("Sum: {}", sum?);
    Ok(())
}
