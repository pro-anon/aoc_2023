use day_7::{part1::part1, part2::part2};

fn main() {
    const INPUT: &str = include_str!("../input.txt");

    let score1 = part1(INPUT);
    println!("Part 1: {}", score1);

    let score2 = part2(INPUT);
    println!("Part 2: {}", score2);
}
