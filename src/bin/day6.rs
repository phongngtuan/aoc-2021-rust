#![feature(stdin_forwarders)]

use std::io;

use advent_of_code_2021_rs::day6::*;

fn main() -> io::Result<()> {

    let input: Vec<u64> = io::stdin()
        .lines()
        .filter_map(|x| x.ok())
        .next()
        .unwrap()
        .split(",")
        .filter_map(|x| x.parse().ok())
        .collect();
    let part1_result = part1(&input);
    println!("{}", part1_result);

    let part2_result = part2(&input);
    println!("{}", part2_result);
    Ok(())
}
