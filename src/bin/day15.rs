use std::io;
use std::io::{Read, stdin};

use advent_of_code_2021_rs::day15::*;

fn main() -> io::Result<()> {
    let mut buf: String = String::new();
    stdin().read_to_string(&mut buf)?;
    let part1_result = part1(&buf);
    println!("{}", part1_result);
    let part2_result = part2(&buf);
    println!("{}", part2_result);
    Ok(())
}
