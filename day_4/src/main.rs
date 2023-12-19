use std::{collections::{HashSet, VecDeque}, println};

fn main() {
    const INPUT: &str = include_str!("../input.txt");

    // let score1 = part1(&INPUT);
    // println!("Part 1: {}", score1);

    let matches: Vec<usize> = INPUT.lines().map(|line| {
        let line = &line[(line.find(":").unwrap() + 2)..];
        let (winning_numbers, numbers) = line.split_once(" | ").unwrap();
        let (winning_numbers, numbers) = (parse_numbers(&winning_numbers), parse_numbers(&numbers));
        winning_numbers.intersection(&numbers).count()
    }).collect();


    
    println!("Part 2: {}", part2(&matches));
}

fn parse_numbers(line: &str) -> HashSet<u32> {
    line.split_whitespace().map(|num| {
        num.parse::<u32>().unwrap()
    }).collect()
}

fn part1(input: &str) -> u32 {
    input.lines().map(|line| {
        let line = &line[(line.find(":").unwrap() + 2)..];
        let (winning_numbers, numbers) = line.split_once(" | ").unwrap();
        let (winning_numbers, numbers) = (parse_numbers(&winning_numbers), parse_numbers(&numbers));
        let matches = winning_numbers.intersection(&numbers).count() as u32;

        if matches == 0 {
            return 0;
        }

        2u32.pow(matches - 1)
    }).sum()
}

#[test]
fn test_part1() {
    let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    assert_eq!(13, part1(&input));
}


fn part2(matches: &Vec<usize>) -> u32 {
    let mut copies = vec![1u32; matches.len()];

    for (i, number_of_matches) in matches.iter().enumerate() {
        println!("i = {}", i);
        for j in (i + 1)..=(i + number_of_matches) {
            println!("\tj = {}", j);
            copies[j] += copies[i];
            println!("\tcopies[j] = {} copies[i] = {}", copies[j], copies[i]);
        }
    }

    copies.iter().sum()
}

#[test]
fn test_part2() {
    let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    let matches: Vec<usize> = input.lines().map(|line| {
        let line = &line[(line.find(":").unwrap() + 2)..];
        let (winning_numbers, numbers) = line.split_once(" | ").unwrap();
        let (winning_numbers, numbers) = (parse_numbers(&winning_numbers), parse_numbers(&numbers));
        winning_numbers.intersection(&numbers).count()
    }).collect();

    assert_eq!(30, part2(&matches));
}
