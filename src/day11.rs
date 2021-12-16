use std::collections::{HashSet, VecDeque};

fn neighbors(row: usize, col: usize, row_count: usize, col_count: usize) -> Vec<(usize, usize)> {
    let row = row as i32;
    let col = col as i32;
    let row_count = row_count as i32;
    let col_count = col_count as i32;
    let offsets = vec![(-1, -1), (0, -1), (1, -1), (-1, 0), (1, 0), (-1, 1), (0, 1), (1, 1)];
    offsets.iter()
        .map( |(r, c)| (*r + row, *c + col))
        .filter( |(r, c)| (0 <= *r) && (*r < row_count) && (0 <= *c) && (*c < col_count))
        .map(|(r, c)| (r as usize, c as usize))
        .collect()
}

#[allow(dead_code)]
fn print_step(current_state: &Vec<Vec<u32>>) {
    let row_count =  current_state.len();
    let col_count = current_state[0].len();
    println!();
    for row in 0..row_count {
        for col in 0..col_count {
            print!("{}", current_state[row][col]);
        }
        println!();
    }
}

fn step(current_state: &mut Vec<Vec<u32>>) -> usize {
    let row_count =  current_state.len();
    let col_count = current_state[0].len();

    let mut all_flashes: HashSet<(usize, usize)> = HashSet::new();
    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();

    // first pass, increment everything by 1, record everything that flashes
    for row in 0..row_count {
        for col in 0..col_count {
            current_state[row][col] += 1;
            if current_state[row][col] == 10 {
                all_flashes.insert((row, col));
                queue.push_back((row, col));
            }
        }
    }

    // now simulate all the flashes
    while let Some((row, col)) = queue.pop_front() {
        // flash
        for (r, c) in neighbors(row, col, row_count, col_count) {
            current_state[r][c] += 1;
            // does this also flash?
            if current_state[r][c] == 10 {
                queue.push_back((r, c));
                all_flashes.insert((r, c));
            }
        }
    }

    // now clear all the flashes
    for (r, c) in &all_flashes {
        current_state[*r][*c] = 0;
    }

    // print_step(current_state);
    all_flashes.len()
}

pub fn part1(input: &str) -> u64 {
    let mut octopus: Vec<Vec<u32>> = Vec::new();

    for (_, line) in input.lines().enumerate() {
        let mut row = Vec::new();
        for (_, digit) in line.chars().enumerate() {
            if let Some(energy_level) = digit.to_digit(10) {
                row.push(energy_level);
            }
        }
        octopus.push(row);
    }

    let mut ans: u64 = 0;
    for _ in 0..100 {
        ans += step(&mut octopus) as u64;

    }
    ans
}


pub fn part2(input: &str) -> u64 {
    let mut octopus: Vec<Vec<u32>> = Vec::new();

    for (_, line) in input.lines().enumerate() {
        let mut row = Vec::new();
        for (_, digit) in line.chars().enumerate() {
            if let Some(energy_level) = digit.to_digit(10) {
                row.push(energy_level);
            }
        }
        octopus.push(row);
    }

    let mut step_count = 0;

    loop {
        step_count += 1;
        if step(&mut octopus) == 100 {
            return step_count;
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn example1() {
        assert_eq!(
            super::part1(
                "\
5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526"
            ), 1656
        )
    }

    #[test]
    fn example2() {
        assert_eq!(
            super::part2(
                "\
5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526"
            ), 195
        )
    }
}
