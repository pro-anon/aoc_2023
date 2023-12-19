use std::println;

fn main() {
    const INPUT: &str = include_str!("../input.txt");

    let input: Vec<Vec<i32>> = INPUT.lines()
        .map(|line| line.split(" ").map(|n| n.parse().unwrap()).collect())
        .collect();

    let (score1, score2) = input.iter().map(|l| solve(l.to_vec())).reduce(|acc, (p1, p2)| {
        (
            acc.0 + p1,
            acc.1 + p2
        )
    }).unwrap();

    println!("Part 1: {}\nPart 2: {}", score1, score2);

}

fn solve(nums: Vec<i32>) -> (i32, i32) {
    let mut diffs = vec![nums];

    let mut curr = diffs.last().unwrap();

    while !curr.iter().all(|x| *x == 0) {
        let new_curr: Vec<i32> = curr.iter().zip(curr.iter().skip(1)).map(|(l, r)| r - l).clone().collect();

        diffs.push(new_curr);

        curr = diffs.last().unwrap();
    }

    (
        diffs.iter().map(|diff| diff.last().unwrap()).sum(),
        diffs.iter().rev().map(|diff| *diff.first().unwrap()).reduce(|acc, x| x - acc).unwrap(),
    )
}

#[test]
fn test() {
    let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
    let input: Vec<Vec<i32>> = input.lines()
        .map(|line| line.split(" ").map(|n| n.parse().unwrap()).collect())
        .collect();

    let (score1, score2) = input.iter().map(|l| solve(l.to_vec())).reduce(|acc, (p1, p2)| {
        (
            acc.0 + p1,
            acc.1 + p2
        )
    }).unwrap();

    assert_eq!(114, score1);
    assert_eq!(2, score2);
}
