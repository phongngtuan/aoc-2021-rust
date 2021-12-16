use std::cmp::Reverse;
use std::collections::HashMap;
use std::collections::BinaryHeap;

pub fn part1(input: &str) -> u64 {
    let mut matrix: HashMap<(i64, i64), u64> = HashMap::new();
    let mut max_row: i64 = 0;
    let mut max_col: i64 = 0;
    for (row, line) in input.lines().enumerate() {
        for (col, char) in line.chars().enumerate() {
            let row = row as i64;
            let col = col as i64;
            matrix.insert((row, col), char.to_digit(10).unwrap() as u64);
            max_row = max_row.max(row);
            max_col = max_col.max(col);
        }
    }

    solve_dijkstra(&matrix, (0, 0), (max_col, max_row))
}

pub fn part2(input: &str) -> u64 {
    let mut matrix: HashMap<(i64, i64), u64> = HashMap::new();
    let mut max_row: i64 = 0;
    let mut max_col: i64 = 0;
    for (row, line) in input.lines().enumerate() {
        for (col, char) in line.chars().enumerate() {
            let row = row as i64;
            let col = col as i64;
            matrix.insert((row, col), char.to_digit(10).unwrap() as u64);
            max_row = max_row.max(row);
            max_col = max_col.max(col);
        }
    }

    let row_count = max_row + 1;
    let col_count = max_col + 1;

    for tile_row in 0..5 {
        for tile_col in 0..5 {
            for row in 0..row_count {
                for col in 0..col_count {
                    let mut val = *matrix.get(&(row, col)).unwrap() + tile_row as u64 + tile_col as u64;
                    if val > 9 {
                        val -= 9;
                    }
                    let actual_row: i64 = tile_row * row_count + row;
                    let actual_col: i64 = tile_col * col_count + col;
                    matrix.insert((actual_row, actual_col), val);
                }
            }
        }
    }

    // Dijkstra's algorithm
    solve_dijkstra(&matrix, (0,0), (col_count * 5 -1, row_count * 5 - 1))
}

fn solve_dijkstra(costs: &HashMap<(i64, i64), u64>, start: (i64, i64), dest: (i64, i64)) -> u64 {
    let mut best = HashMap::new();
    let mut heap = BinaryHeap::new();

    for x in 0..=dest.0 {
        for y in 0..=dest.1 {
            best.insert((x, y), u64::MAX);
        }
    }

    heap.push((Reverse(0), start));
    best.insert(start, 0);
    while let Some((Reverse(curr_cost), (x0, y0))) = heap.pop() {
        if (x0, y0) == dest {
            return curr_cost
        }
        if curr_cost <= best[&(x0, y0)] {
            for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let (x, y) = (x0 + dx, y0 + dy);
                if let Some(&cost) = costs.get(&(x, y)) {
                    if curr_cost + cost < best[&(x, y)] {
                        best.insert((x, y), curr_cost + cost);
                        heap.push((Reverse(curr_cost + cost), (x, y)));
                    }
                }
            }
        }
    }

    0
}

#[cfg(test)]
mod tests {
    #[test]
    fn example1() {
        assert_eq!(
            super::part1(
                "\
1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581"
            ), 40
        )
    }

    #[test]
    fn example2() {
        assert_eq!(
            super::part2(
                "\
1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581"
            ), 315
        )
    }
}
