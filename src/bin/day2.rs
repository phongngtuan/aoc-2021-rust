#![feature(stdin_forwarders)]

use advent_of_code_2021_rs::day2;
use std::io;

fn main() -> io::Result<()> {
    let ints: Vec<String> = io::stdin()
        .lines()
        .filter_map(|x| x.ok())
        .collect();

    let part1_result = day2::part1(&ints);
    println!("{}", part1_result);

    let part2_result = day2::part2(&ints);
    println!("{}", part2_result);
    Ok(())
}
