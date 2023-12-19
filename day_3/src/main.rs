use std::{unimplemented, assert_eq, dbg, println, eprintln};

fn main() {
    const INPUT: &str = include_str!("../input.txt");
    let input_lines: Vec<&str> = INPUT.split_terminator("\n").collect();

    let score1 = part1(&input_lines);
    println!("Part 1: {}", score1);

    let score2 = part2(&input_lines);
    println!("Part 2: {}", score2);
}

fn should_sum(mut l: usize, r: usize, line: &[u8], prev_line: Option<&str>, next_line: Option<&str>) -> bool {
    if l > 0 {
        l -= 1;
    }

    if line[l] != b'.' && !line[l].is_ascii_digit() {
        return true;
    }

    if line[r] != b'.' && !line[r].is_ascii_digit() {
        return true;
    }

    if let Some(prev_line) = prev_line {
        for i in l..=r {
            let char = prev_line.as_bytes()[i];
            if !char.is_ascii_digit() && char != b'.' {
                return true;
            }
        }
    }

    if let Some(next_line) = next_line {
        for i in l..=r {
            let char = next_line.as_bytes()[i];
            if !char.is_ascii_digit() && char != b'.' {
                return true;
            }
        }
    }
    

    false
}

fn part1(input: &Vec<&str>) -> u32 {
    let mut score = 0;

    for row in 0..input.len() {
        let line = input[row].as_bytes();

        let mut l = 0;

        let mut is_curr_number = false;

        for r in 0..line.len() {
            let char = line[r];

            if char.is_ascii_digit() {
                is_curr_number = true;
                if r == line.len() - 1 {
                    let num = &input[row][l..=r].parse::<u32>().unwrap();
                    let prev_line = if row > 0 { Some(input[row - 1]) } else { None };
                    let next_line = if row < input.len() - 1 { Some(input[row + 1]) } else { None };

                    if should_sum(l, r, line, prev_line, next_line) {
                        score += num;
                    }
                }
                continue;
            }

            if is_curr_number {
                let num = &input[row][l..r].parse::<u32>().unwrap();
                let prev_line = if row > 0 { Some(input[row - 1]) } else { None };
                let next_line = if row < input.len() - 1 { Some(input[row + 1]) } else { None };

                if should_sum(l, r, line, prev_line, next_line) {
                    score += num;
                }
            }

            l = r + 1;

            is_curr_number = false;
        }
    }

    score
}

#[test]
fn test_part1() {
    let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
    let input = input.split("\n").collect();
    let score = part1(&input);
    assert_eq!(score, 4361);

    let input = "12......34
5*......$6
78......90".split_terminator("\n").collect();
    let score = part1(&input);
    assert_eq!(score, 225);

    let input = "..........
.10*10$10.
..........".split_terminator("\n").collect();
    let score = part1(&input);
    assert_eq!(score, 30);

    let input = ".10.10....
.10*10....
.10.10....".split_terminator("\n").collect();
    let score = part1(&input);
    assert_eq!(score, 60);

    let input = ".10.11....
.12*13....
.14.15....".split_terminator("\n").collect();
    let score = part1(&input);
    assert_eq!(score, 75);

    let input = "..........
.10*10$10.
.10*10$10.
.10*10$10.
..........".split_terminator("\n").collect();
    let score = part1(&input);
    assert_eq!(score, 90);

    let input = ".10..10.
.*....*.
.10..10.".split_terminator("\n").collect();
    let score = part1(&input);
    assert_eq!(score, 40);

    let input = "10..10
*.0..*
10..10".split_terminator("\n").collect();
    let score = part1(&input);
    assert_eq!(score, 40);

    let input = "....#234".split_terminator("\n").collect();
    let score = part1(&input);
    assert_eq!(score, 234);

    let input = "..11111...
..1...1...
..1.$.1...
..1...1...
..11111...".split_terminator("\n").collect();
    let score = part1(&input);
    assert_eq!(score, 0);

    let input = "........
.34.455
817#56..
.41.91..
........".split_terminator("\n").collect();
    let score = part1(&input);
    assert_eq!(score, 1494);

    let input = ".....234
....#...".split_terminator("\n").collect();
    let score = part1(&input);
    assert_eq!(score, 234);

    let input = "........
1.......
........
....#...
.......1".split_terminator("\n").collect();
    let score = part1(&input);
    assert_eq!(score, 0);

    const INPUT: &str = include_str!("../input.txt");
    let input: Vec<&str> = INPUT.split_terminator("\n").collect();
    assert_eq!(530495, part1(&input));

    // Correct answer for part 2
    // 80253814
}

fn part2(lines: &Vec<&str>) -> u32 {
    let gears = get_gears(&lines);
    let numbers = get_numbers(&lines);

    // (x, y)
    const DIRECTIONS: [(isize, isize); 8] = [
        (-1,  1),
        ( 0,  1),
        ( 1,  1),
        ( 1,  0),
        ( 1, -1),
        ( 0, -1),
        (-1, -1),
        (-1,  0),
    ];

    gears.iter().filter_map(|gear| {
        let part_numbers: Vec<&Number> = numbers.iter().filter(|number| {
            for (x, y) in DIRECTIONS {
                if number.row as isize == gear.row as isize + y &&
                    gear.col as isize + x >= number.col_from as isize &&
                    gear.col as isize + x <= number.col_to as isize {
                    return true;
                }
            }

            false
        }).collect();

        if part_numbers.len() == 2 {
            return Some(part_numbers.iter().map(|pn| pn.value).product::<u32>());
        }

        None
    }).sum()
}

#[derive(Debug)]
struct Gear {
    row: usize,
    col: usize,
}

#[derive(Debug)]
struct Number {
    row: usize,
    col_from: usize,
    col_to: usize,
    value: u32,
}

fn get_gears(lines: &Vec<&str>) -> Vec<Gear> {
    lines.iter().enumerate().flat_map(|(row, line)| {
        line.chars().enumerate().filter_map(|(col, char)| {
            match char {
                '*' => Some(Gear { row, col }),
                _ => None,
            }
        }).collect::<Vec<Gear>>()
    }).collect()
}

fn get_numbers(lines: &Vec<&str>) -> Vec<Number> {
    lines.iter().enumerate().flat_map(|(row, line)| {
        let mut numbers = vec![];

        let mut curr_num = None;

        line.chars().enumerate().for_each(|(col, char)| {
            match (char.is_ascii_digit(), &mut curr_num) {
                (true, None) => {
                    curr_num = Some(Number {
                        row,
                        col_from: col,
                        col_to: col,
                        value: 0,
                    });
                },
                (false, Some(num)) => {
                    num.col_to = col - 1;
                    num.value = line[num.col_from..=num.col_to].parse().unwrap();
                    numbers.push(curr_num.take().unwrap());
                },
                (true, Some(num)) => {
                    num.col_to = col;
                }
                _ => {},
            }
        });

        if let Some(mut curr_num) = curr_num {
            curr_num.col_to = line.len() - 1;
            curr_num.value = line[curr_num.col_from..=curr_num.col_to].parse().unwrap();
            numbers.push(curr_num);
        }

        numbers
    }).collect()
}

#[test]
fn test_part2() {
    let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..".split_terminator("\n").collect();

    let score = part2(&input);

    assert_eq!(467835, score);
}
