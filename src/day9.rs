use std::cmp::Reverse;
use std::collections::{BinaryHeap, VecDeque};

pub fn part1(lines: &[String]) -> u64 {
    let mut height_map: Vec<Vec<u64>> = Vec::new();
    for line in lines {
        let row: Vec<u64> = line.chars().filter_map(|c| c.to_digit(10)).map(|d| d as u64).collect();
        height_map.push(row);
    }

    let row_count = height_map.len();
    let col_count = height_map[0].len();

    let mut total_risk_level: u64 = 0;
    for row in 0..row_count {
        for col in 0..col_count {
            let is_low_point =
                ((col == 0) || (height_map[row][col] < height_map[row][col - 1])) &&
                    ((col + 1 == col_count) || (height_map[row][col] < height_map[row][col + 1])) &&
                    ((row == 0) || (height_map[row][col] < height_map[row - 1][col])) &&
                    ((row + 1 == row_count) || (height_map[row][col] < height_map[row + 1][col]));

            if is_low_point {
                total_risk_level += height_map[row][col] + 1;
            }
        }
    }

    total_risk_level
}

// struct Matrix {
//     row_count: usize,
//     col_count: usize,
//     elements: Vec<Vec<u>>
// }
//
// struct
//
// impl Matrix {
// }

fn neighbors(row: usize, col: usize, row_count: usize, col_count: usize) -> Vec<(usize, usize)> {
    let mut ans: Vec<(usize, usize)> = Vec::new();
    if col > 0 { ans.push((row, col - 1)); }
    if col + 1 < col_count { ans.push((row, col + 1)); }
    if row > 0 { ans.push((row - 1, col)); }
    if row + 1 < row_count { ans.push((row + 1, col)); }
    ans
}

pub fn part2(lines: &[String]) -> u64 {
    let mut global_basins = BinaryHeap::new();
    let mut height_map: Vec<Vec<u64>> = Vec::new();
    for line in lines {
        let row: Vec<u64> = line.chars().filter_map(|c| c.to_digit(10)).map(|d| d as u64).collect();
        height_map.push(row);
    }

    let row_count = height_map.len();
    let col_count = height_map[0].len();

    // BFS
    let mut visited: Vec<Vec<bool>> = vec![vec![false; col_count]; row_count];
    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();

    for row in 0..row_count {
        for col in 0..col_count {
            let is_low_point = neighbors(row, col, row_count, col_count)
                .iter()
                .all(|(row_nei, col_nei)| height_map[row][col] < height_map[*row_nei][*col_nei]);

            if is_low_point {
                let mut local_basin: u64 = 0;
                // BFS loop
                queue.push_back((row, col));
                visited[row][col] = true;

                while let Some((row, col)) = queue.pop_front() {
                    local_basin += 1;
                    for (row_nei, col_nei) in neighbors(row, col, row_count, col_count) {
                        if height_map[row_nei][col_nei] > height_map[row][col]
                            && height_map[row_nei][col_nei] < 9
                            && !visited[row_nei][col_nei] {
                            visited[row_nei][col_nei] = true;
                            queue.push_back((row_nei, col_nei));
                        }
                    }
                }
                if global_basins.len() < 3 {
                    global_basins.push(Reverse(local_basin));
                } else if let Some(min_basin) = global_basins.peek() {
                    if min_basin.0 < local_basin {
                        global_basins.pop();
                        global_basins.push(Reverse(local_basin));
                    }
                }
            }
        }
    }

    global_basins.iter().map(|x| x.0).product()
}