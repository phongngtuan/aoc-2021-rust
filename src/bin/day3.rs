#![feature(stdin_forwarders)]

use advent_of_code_2021_rs::day3::*;
use std::io;

fn main() -> io::Result<()> {
    let input: Vec<String> = io::stdin()
        .lines()
        .filter_map(|x| x.ok())
        .collect();

    let part1_result = part1(&input);
    println!("{}", part1_result);

    let part2_result = part2(&input);
    println!("{}", part2_result);
    Ok(())
}
