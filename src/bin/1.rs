use std::collections::HashSet;

const INPUT: &'static str = include_str!("../input/1.in");

fn main() {
    part_one();
    part_two();
}

fn part_one() {
    let freq = INPUT.lines()
         .map(|line| line.parse::<i64>().unwrap());
         .sum::<i64>()

    println!("{}", freq);
}

fn part_two() {
    let change_iter = INPUT.lines()
         .map(|line| line.parse::<i64>().unwrap())
         .cycle();

    let mut freq = 0;
    let mut seen = HashSet::new();

    for change in change_iter {
        freq += change;

        if !seen.insert(freq) {
            break;
        }
    }

    println!("{}", freq);
}
