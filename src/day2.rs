use std::{
    collections::HashMap,
    io::{stdin, BufRead},
};

pub fn day2() -> usize {
    let max_red: usize = 12;
    let max_green: usize = 13;
    let max_blue: usize = 14;
    let std_in = stdin();
    let mut filted_input: Vec<usize> = vec![];
    for line in std_in.lock().lines() {
        match line {
            Ok(line) => {
                if line.is_empty() {
                    break;
                } else {
                    filted_input.push(part_2(&line));
                }
            }
            Err(error) => println!("Error: {}", error),
        }
    }

    return filted_input.iter().sum::<usize>();
}

fn part_1(input: &str, max_red: usize, max_green: usize, max_blue: usize) -> usize {
    let (game_num, game_detail) = input.split_once(": ").unwrap();
    let games: Vec<&str> = game_detail.split("; ").collect();
    let is_possible = games.iter().all(|g| {
        let game_result: Vec<&str> = g.split(", ").collect();
        let mut game_color = HashMap::<&str, usize>::new();
        for gr in game_result {
            let (num, color) = gr.split_once(" ").unwrap();
            game_color.insert(color, num.parse::<usize>().unwrap());
        }
        return game_color.get("red").unwrap_or(&0) <= &max_red
            && game_color.get("green").unwrap_or(&0) <= &max_green
            && game_color.get("blue").unwrap_or(&0) <= &max_blue;
    });
    if is_possible {
        return game_num
            .trim_start_matches("Game ")
            .parse::<usize>()
            .unwrap();
    } else {
        return 0;
    }
}

fn part_2(input: &str) -> usize {
    let (_, game_detail) = input.split_once(": ").unwrap();
    let games: Vec<&str> = game_detail.split("; ").collect();
    let mut color_map = HashMap::<&str, usize>::new();
    for g in games {
        let game_result: Vec<&str> = g.split(", ").collect();
        for gr in game_result {
            let (num, color) = gr.split_once(" ").unwrap();
            let color_num = color_map.entry(color).or_insert(0);
            *color_num = [*color_num, num.parse::<usize>().unwrap()]
                .iter()
                .max()
                .unwrap()
                .clone();
        }
    }
    return color_map.values().cloned().reduce(|a, b| a * b).unwrap();
}
