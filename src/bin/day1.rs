#![feature(stdin_forwarders)]

use std::io;

use advent_of_code_2021_rs::day1;

fn main() -> io::Result<()> {
    let ints: Vec<u32> = io::stdin()
        .lines()
        .filter_map(|s| s.ok())
        .filter_map(|s| s.parse().ok())
        .collect();

    let day1_result = day1::part1(&ints);
    let day2_result = day1::part2(&ints);

    println!("{}", day1_result);
    println!("{}", day2_result);

    Ok(())
}
