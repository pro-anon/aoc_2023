use std::{println, collections::HashMap};

fn main() {
    const INPUT: &str = include_str!("../HxMf.txt");

    let score1 = part1(INPUT);
    let score2 = part2(INPUT);

    println!("Part 1 = {}", score1);
    println!("Part 2 = {}", score2);
}

fn part1(input: &str) -> u32 {
    input.lines().map(|line| {
        let mut nums = line.chars().filter(|c| c.is_ascii_digit());

        let tens = nums.next().unwrap().to_digit(10).unwrap() * 10;
        let ones = if let Some(ones) = nums.next_back() {
            ones.to_digit(10).unwrap()
        } else {
            tens / 10
        };

        tens + ones
    }).sum()
}

#[test]
fn test_part1() {
    let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

    let score = part1(input);

    assert_eq!(score, 142);
}

fn part2(input: &str) -> u32 {
    let num_literals_original = HashMap::from([
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

    input.lines().map(|line| {
        let mut tens = 0;
        let mut left = 0;
        let mut num_literals = num_literals_original.clone();

        for (right, char) in line.chars().enumerate() {
            if char.is_ascii_digit() {
                tens = char.to_digit(10).unwrap() * 10;
                break;
            }

            num_literals.retain(|key, _| {
                if key.len() < right - left { return false };
                key.starts_with(&line[left..=right])
            });

            while left < right && num_literals.is_empty() {
                num_literals = num_literals_original.clone();
                left += 1;
                num_literals.retain(|key, _| {
                    if key.len() < right - left { return false };
                    key.starts_with(&line[left..=right])
                });
            }

            if let Some(num) = num_literals.get(&line[left..=right]) {
                tens = num * 10;
                break;
            }
        }

        let mut ones = 0;
        let mut num_literals = num_literals_original.clone();
        let mut right = line.len() - 1;

        for (left, char) in line.chars().rev().enumerate() {
            let left = line.len() - 1 - left;

            if char.is_ascii_digit() {
                ones = char.to_digit(10).unwrap();
                break;
            }

            num_literals.retain(|key, _| {
                if key.len() < right - left { return false };
                key.ends_with(&line[left..=right])
            });

            while right > left && num_literals.is_empty() {
                num_literals = num_literals_original.clone();
                right -= 1 ;
                num_literals.retain(|key, _| {
                    key.ends_with(&line[left..=right])
                });
            }

            if let Some(num) = num_literals.get(&line[left..=right]) {
                ones = *num;
                break;
            }
        }

        tens + ones
    }).sum()
}

#[test]
fn test_part2() {
    let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

    let score = part2(input);

    assert_eq!(score, 281);
}
