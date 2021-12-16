use std::collections::HashSet;

pub fn part1(input: &str) -> u64 {
    let mut lines = input.lines();
    let mut graph: HashSet<(i64, i64)> = HashSet::new();
    while let Some(line) = lines.next() {
        if line.trim().is_empty() {
            break;
        }

        let (x, y) = line.split_once(",").unwrap();
        // println!("{}, {}", x, y);
        let x = x.parse().unwrap();
        let y = y.parse().unwrap();
        graph.insert((x, y));
    }

    let mut fold_count = 0;
    while let Some(line) = lines.next() {
        fold_count += 1;
        if fold_count == 2 {
            break;
        }
        let (instruction, coordinate) = line.split_once("=").unwrap();
        let fold_line: i64 = coordinate.parse().unwrap();
        match instruction {
            "fold along y" => {
                println!("folding y = {}", fold_line);

                let mut folded = HashSet::new();
                for (x, y) in graph {
                    if y > fold_line {
                        let y = fold_line - (y - fold_line);
                        folded.insert((x, y));
                    } else {
                        folded.insert((x, y));
                    }
                }
                graph = folded;
            },

            "fold along x" => {
                println!("folding y = {}", fold_line);
                let mut folded = HashSet::new();
                for (x, y) in graph {
                    if x > fold_line {
                        let x = fold_line - (x - fold_line);
                        folded.insert((x, y));
                    } else {
                        folded.insert((x, y));
                    }
                }
                graph = folded;
            },
            _ => panic!(),
        }
    }
    graph.len() as u64
}

pub fn part2(input: &str) -> u64 {
    let mut lines = input.lines();
    let mut graph: HashSet<(i64, i64)> = HashSet::new();
    let mut max_x = 0;
    let mut max_y = 0;
    while let Some(line) = lines.next() {
        if line.trim().is_empty() {
            break;
        }

        let (x, y) = line.split_once(",").unwrap();
        // println!("{}, {}", x, y);
        let x = x.parse().unwrap();
        let y = y.parse().unwrap();
        max_x = max_x.max(x);
        max_y = max_y.max(y);
        graph.insert((x, y));
    }

    while let Some(line) = lines.next() {
        let (instruction, coordinate) = line.split_once("=").unwrap();
        let fold_line: i64 = coordinate.parse().unwrap();
        match instruction {
            "fold along y" => {
                println!("folding y = {}", fold_line);
                max_y = fold_line;

                let mut folded = HashSet::new();
                for (x, y) in graph {
                    if y > fold_line {
                        let y = fold_line - (y - fold_line);
                        folded.insert((x, y));
                    } else {
                        folded.insert((x, y));
                    }
                }
                graph = folded;
            },

            "fold along x" => {
                println!("folding x = {}", fold_line);
                max_x = fold_line;
                let mut folded = HashSet::new();
                for (x, y) in graph {
                    if x > fold_line {
                        let x = fold_line - (x - fold_line);
                        folded.insert((x, y));
                    } else {
                        folded.insert((x, y));
                    }
                }
                graph = folded;
            },
            _ => panic!(),
        }
    }

    for y in 0..=max_y {
        for x in 0..=max_x {
            if graph.contains(&(x, y)) {
                print!("#")
            } else {
                print!(".")
            }
        }
        println!();
    }

    graph.len() as u64
}

#[cfg(test)]
mod tests {
    #[test]
    fn example1() {
        assert_eq!(
            super::part1(
                "\
6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5"
            ), 0
        )
    }
}
