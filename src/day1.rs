use std::io::{stdin, BufRead};

pub fn day1() -> u32 {
    let std_in = stdin();
    let mut calibration_value: Vec<u32> = vec![];
    for line in std_in.lock().lines() {
        match line {
            Ok(line) => {
                if line.is_empty() {
                    break;
                } else {
                    calibration_value.push(find_first_and_last_digit(&line));
                }
            }
            Err(error) => println!("Error: {}", error),
        }
    }

    return calibration_value.iter().sum::<u32>();
}

fn find_first_and_last_digit(input: &str) -> u32 {
    let input_substr: Vec<&str> = input.split_inclusive(|c: char| c.is_digit(10)).collect();
    let mut digit_chars: Vec<char> = vec![];
    let digit_mapping = [
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ];
    let min_length = digit_mapping
        .iter()
        .map(|(word, _)| word.len())
        .min()
        .unwrap();
    let max_length = digit_mapping
        .iter()
        .map(|(word, _)| word.len())
        .max()
        .unwrap();
    for sub_str in input_substr {
        if sub_str.is_empty() {
            continue;
        }
        let mut start: usize = 0;
        while start < sub_str.len() {
            let max_loop_length = [sub_str.len() - start, max_length]
                .iter()
                .min()
                .unwrap()
                .clone()
                + 1;
            for sub_sub_str_len in 0..max_loop_length {
                let sub_sub_str = &sub_str[start..start + sub_sub_str_len];
                if sub_sub_str.is_empty() || sub_sub_str.len() < min_length {
                    continue;
                }
                for (word, digit) in digit_mapping {
                    if sub_sub_str == word {
                        digit_chars.push(digit.chars().next().unwrap());
                        break;
                    }
                }
            }
            start += 1;
        }

        let last_char = sub_str.chars().last().unwrap();
        if last_char.is_digit(10) {
            digit_chars.push(last_char);
            continue;
        }
    }
    return [digit_chars[0], digit_chars[digit_chars.len() - 1]]
        .iter()
        .collect::<String>()
        .parse::<u32>()
        .unwrap();
}
