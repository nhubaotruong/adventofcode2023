use std::collections::HashMap;

#[test]
fn test() {
    let result = run(String::from(
        "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
    ));
    assert_eq!(result, 2286);
}

pub fn run(input: String) -> usize {
    let max_red: usize = 12;
    let max_green: usize = 13;
    let max_blue: usize = 14;
    let filtered_input = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| part_2(&line))
        .collect::<Vec<usize>>();

    let result = filtered_input.iter().sum::<usize>();
    println!("Result: {}", result);
    return result;
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
