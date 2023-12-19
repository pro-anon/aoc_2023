fn main() {
    let times = vec![51, 69, 98, 78];
    let distances = vec![377, 1171, 1224, 1505];

    let score1 = part1(&times, &distances);

    println!("Part 1: {}", score1);

    let time = 51699878;
    let distance = 377117112241505;

    let score2 = part2(time, distance);

    println!("Part 2: {}", score2);
}

fn part1(times: &Vec<u32>, distances: &Vec<u32>) -> u32 {

    times.iter().zip(distances.iter()).filter_map(|(&time, &distance)| {
        let mut count = 0;

        for x in 0..=(time as usize) {
            let x = x as u32;
            if (time - x) * x > distance {
                count += 1
            }
        }

        if count > 0 {
            Some(count)
        } else {
            None
        }

    }).product()

}

#[test]
fn test_part1() {
    let times = vec![7, 15, 30];
    let distances = vec![9, 40, 200];

    assert_eq!(288, part1(&times, &distances));
}

fn part2(time: u64, distance: u64) -> u64 {
    let mut count = 0;

    for x in 0..=(time as usize) {
        let x = x as u64;
        if (time - x) * x > distance {
            count += 1
        }
    }

    count
}

#[test]
fn test_part2() {
    let time     = 71530;
    let distance = 940200;

    assert_eq!(71503, part2(time, distance));
}
