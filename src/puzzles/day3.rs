#[test]
fn test() {
    let result = run(String::from(
        "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..",
    ));
    assert_eq!(result, 4361);
}

pub fn run(input: String) -> usize {
    let result = part_1(input);
    println!("Result: {}", result);
    return result;
}

fn part_1(input: String) -> usize {
    let mut valid_digits: Vec<usize> = vec![];
    let lines: Vec<&str> = input.lines().collect();
    for (line_index, line) in lines.iter().enumerate() {
        let mut digit_chars: Vec<(usize, char)> = vec![];
        let mut is_auto_passed = false;
        for char in line.chars().enumerate() {
            if char.1.is_digit(10) {
                // Appending to digits
                digit_chars.push(char)
            } else if char.1 == '.' {
                if is_auto_passed && digit_chars.len() > 0 {
                    valid_digits.push(
                        String::from_iter(digit_chars.iter().map(|x| x.1))
                            .parse::<usize>()
                            .unwrap(),
                    );
                    digit_chars.clear();
                } else if digit_chars.len() > 0 {
                    // Check diagonally and vertically
                    let mut digit_chars_pos: Vec<usize> = digit_chars.iter().map(|x| x.0).collect();
                    if !digit_chars_pos.starts_with(&[0]) {
                        digit_chars_pos.insert(0, digit_chars_pos[0] - 1);
                    }
                    if !digit_chars_pos.ends_with(&[line.len() - 1]) {
                        digit_chars_pos.push(digit_chars_pos[digit_chars_pos.len() - 1] + 1);
                    }
                    let prev_line = match line_index.checked_sub(1) {
                        Some(x) => lines.get(x),
                        None => None,
                    };
                    let next_line = match line_index + 1 {
                        x if x < lines.len() => lines.get(x),
                        _ => None,
                    };
                    let is_adjacent_prev = match prev_line {
                        Some(l) => digit_chars_pos.iter().any(|pos| {
                            let c = l.chars().nth(*pos).unwrap();
                            return !c.is_digit(10) && c != '.';
                        }),
                        None => false,
                    };
                    let is_adjacent_next = match next_line {
                        Some(l) => digit_chars_pos.iter().any(|pos| {
                            let c = l.chars().nth(*pos).unwrap();
                            return !c.is_digit(10) && c != '.';
                        }),
                        None => false,
                    };
                    if is_adjacent_prev || is_adjacent_next {
                        valid_digits.push(
                            String::from_iter(digit_chars.iter().map(|x| x.1))
                                .parse::<usize>()
                                .unwrap(),
                        );
                        digit_chars.clear();
                    }
                }
                is_auto_passed = false;
            } else {
                // Passed by horizontally
                is_auto_passed = true;
                if digit_chars.len() > 0 {
                    valid_digits.push(
                        String::from_iter(digit_chars.iter().map(|x| x.1))
                            .parse::<usize>()
                            .unwrap(),
                    );
                    digit_chars.clear();
                }
            }
        }
    }
    return valid_digits.iter().sum();
}
