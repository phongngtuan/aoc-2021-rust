#![feature(stdin_forwarders)]

use std::io;

use advent_of_code_2021_rs::day4::*;

fn main() -> io::Result<()> {

    let mut input = io::stdin()
        .lines()
        .filter_map(|x| x.ok());

    let numbers: Vec<u32> = input.next().unwrap().trim().split(',').filter_map(|s| s.parse::<u32>().ok()).collect();
    let mut boards: Vec<Board> = Vec::new();
    for line in input {
        if line.is_empty() {
            boards.push(Vec::new())
        } else {
            let row: Vec<u32> = line.trim().split(' ').filter_map(|s| s.parse::<u32>().ok()).collect();
            boards.last_mut().unwrap().push(row);
        }
    }

    println!("# numbers: {}, # boards: {}", numbers.len(), boards.len());

    let part1_result = part1(&numbers, &boards);
    println!("{}", part1_result);

    let part2_result = part2(&numbers, &boards);
    println!("{}", part2_result);
    Ok(())
}
