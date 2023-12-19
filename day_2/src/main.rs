use std::{str::FromStr, cmp::max};

#[derive(Debug)]
struct Set {
    red: u32,
    green: u32,
    blue: u32,
}

struct Game {
    sets: Vec<Set>,
}

impl FromStr for Game {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parsed_sets: Vec<Set> = Vec::with_capacity(3);

        let from = s.find(':').unwrap() + 2;
        let s = &s[from..];
        let sets = s.split("; ");
        for set in sets {
            let mut parsed_set = Set { red: 0, green: 0, blue: 0 };
            let cubes = set.split(", ");
            for cube in cubes {
                let (num, color) = cube.split_once(' ').unwrap();
                let num: u32 = num.parse().unwrap();
                match color {
                    "red" => parsed_set.red += num,
                    "green" => parsed_set.green += num,
                    "blue" => parsed_set.blue += num,
                    _ => panic!("Invalid color received while parsing: {}", color),
                }
            }
            parsed_sets.push(parsed_set);
        }

        Ok(Game { sets: parsed_sets })
    }
}

fn main() {
    const INPUT: &str = include_str!("../input.txt");
    let games = INPUT.lines().map(|line| line.parse().unwrap()).collect();

    let possible_games = part1(&games);
    let sum_of_power_sets = part2(&games);

    println!("Part 1: possible games = {}", possible_games);
    println!("Part 2: sum of power sets = {}", sum_of_power_sets);
}

fn part1(games: &Vec<Game>) -> u32 {
    games.iter().enumerate().filter_map(|(i, game)| {
        if game.sets.iter().all(|set| {
            set.red <= 12 && set.green <= 13 && set.blue <= 14
        }) {
            Some(i + 1)
        } else {
            None
        }
    }).sum::<usize>() as u32
}

fn part2(games: &Vec<Game>) -> u32 {
    games.iter().map(|game| {
        let mut minimum_set = Set{red: 0, green: 0, blue: 0 };

        for set in &game.sets {
            minimum_set.red = max(minimum_set.red, set.red);
            minimum_set.green = max(minimum_set.green, set.green);
            minimum_set.blue = max(minimum_set.blue, set.blue);
        }

        minimum_set.red * minimum_set.green * minimum_set.blue
    }).sum()
}

#[test]
fn test_part1() {
    let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    let possible_games = part1(&input.lines().map(|line| line.parse().unwrap()).collect());

    assert_eq!(possible_games, 8);
}

#[test]
fn test_part2() {
    let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    let possible_games = part2(&input.lines().map(|line| line.parse().unwrap()).collect());

    assert_eq!(possible_games, 2286);
}
