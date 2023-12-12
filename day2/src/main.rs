use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

const INPUT_PT1: &str = "input.main.part1.txt";
const INPUT_PT2: &str = "input.main.part2.txt";
const MAX_RED: u128 = 12;
const MAX_GREEN: u128 = 13;
const MAX_BLUE: u128 = 14;

pub struct Set {
    red: u128,
    green: u128,
    blue: u128,
    is_possible: bool,
}

pub struct Game {
    sets: Vec<Set>,
    content: String,
    id: u128,
    red: u128,
    green: u128,
    blue: u128,
    max_red: u128,
    max_green: u128,
    max_blue: u128,
    power: u128,
    is_possible: bool,
}

fn parse_game_string(content_string: &str) -> io::Result<Game> {
    let mut sets = Vec::new();
    let game_split = content_string.split(": ").collect::<Vec<&str>>();
    let sets_str: Vec<&str> = game_split[1].split("; ").collect();
    let mut red_count_t = 0;
    let mut green_count_t = 0;
    let mut blue_count_t = 0;
    let mut game_is_possible = true;
    let mut max_red = 0;
    let mut max_green = 0;
    let mut max_blue = 0;

    for set_str in sets_str {
        let mut red_count = 0;
        let mut green_count = 0;
        let mut blue_count = 0;

        let items: Vec<&str> = set_str.split(',').collect();

        for item in items {
            let parts: Vec<&str> = item.trim().split_whitespace().collect();
            if let Some(quantity) = parts.get(0).and_then(|s| s.parse::<u128>().ok()) {
                if let Some(color) = parts.get(1) {
                    match *color {
                        "red" => {
                            red_count += quantity;
                            red_count_t += quantity;
                            if quantity > max_red {
                                max_red = quantity;
                            }
                        }
                        "green" => {
                            green_count += quantity;
                            green_count_t += quantity;
                            if quantity > max_green {
                                max_green = quantity;
                            }
                        }
                        "blue" => {
                            blue_count += quantity;
                            blue_count_t += quantity;
                            if quantity > max_blue {
                                max_blue = quantity;
                            }
                        }
                        _ => {}
                    }
                }
            }
        }

        let not_possible = red_count > MAX_RED || green_count > MAX_GREEN || blue_count > MAX_BLUE;
        if not_possible {
            game_is_possible = false;
        }
        sets.push(Set {
            red: red_count,
            green: green_count,
            blue: blue_count,
            is_possible: !not_possible,
        });
    }

    let id = game_split[0].split(" ").collect::<Vec<&str>>()[1]
        .parse::<u128>()
        .unwrap();
    return Ok(Game {
        id: id,
        sets: sets,
        content: content_string.to_string(),
        red: red_count_t,
        green: green_count_t,
        blue: blue_count_t,
        max_red: max_red,
        max_green: max_green,
        max_blue: max_blue,
        power: max_red * max_green * max_blue,
        is_possible: game_is_possible,
    });
}

fn parse_games(input_file: &str) -> io::Result<Vec<Game>> {
    // Sample input:
    // Game 1: 1 green, 2 red, 6 blue; 4 red, 1 green, 3 blue; 7 blue, 5 green; 6 blue, 2 red, 1 green
    // Open a file in read-only mode
    let mut games = Vec::new();
    let file = File::open(input_file)?;
    let reader = BufReader::new(file);
    // Iterate over lines and print each line
    for content in reader.lines() {
        games.push(parse_game_string(&content.unwrap()).unwrap());
    }
    return Ok(games);
}

fn part1() {
    let mut sum = 0;
    for game in parse_games(INPUT_PT1).unwrap() {
        if game.is_possible {
            println!("\nGame {} is possible.", game.id);
            println!(
                "Red: {}, Green: {}, Blue: {}",
                game.red, game.green, game.blue
            );
            println!("{}\n", game.content);
            sum += game.id;
        }
    }
    println!("Sum: {}", sum);
}

fn part2() {
    let mut sum = 0;
    for game in parse_games(INPUT_PT2).unwrap() {
        println!("\nGame {} power: {}.", game.id, game.power);
        println!(
            "Red: {}, Green: {}, Blue: {}\n",
            game.max_red, game.max_green, game.max_blue
        );
        sum += game.power;
    }
    println!("Sum: {}", sum);
}

fn main() {
    part2();
}
