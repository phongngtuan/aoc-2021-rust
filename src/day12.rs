use std::collections::HashMap;

use itertools::Itertools;

fn is_large(cave: &str) -> bool {
    cave.chars().all(|c| c.is_uppercase())
}

fn is_small(cave: &str) -> bool {
    cave.chars().all(|c| c.is_lowercase())
}

#[allow(dead_code)]
fn print_path(path: &Vec<String>) {
    for node in path {
        print!("{},", node);
    }
    println!();
}

fn dfs(graph: &HashMap<String, Vec<String>>, node: String, path: &mut Vec<String>, result: &mut u64) {
    if node == "end" {
        *result += 1;
        return
    }

    path.push(node.to_string());
    if let Some(neis) = graph.get(&node) {
        for nei in neis {
            if is_large(nei) || !path.contains(nei) {
                dfs(graph, nei.clone(), path, result);
            }
        }
    }
    path.pop();
}

pub fn part1(input: &str) -> u64 {
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();
    for line in input.lines() {
        let (a, b) = line.split_once("-").unwrap();
        if !graph.contains_key(a) {
            graph.insert(a.to_string(), Vec::new());
        }
        graph.get_mut(a).unwrap().push(b.to_string());
        if !graph.contains_key(b) {
            graph.insert(b.to_string(), Vec::new());
        }
        graph.get_mut(b).unwrap().push(a.to_string());
    }

    // dfs
    let mut ans = 0;
    let mut path = Vec::new();
    dfs(&mut graph, String::from("start"), &mut path, &mut ans);
    ans
}

fn is_legal_move(path: &Vec<String>, next_node: &str) -> bool {
    match next_node {
        "start" => false,
        "end" => true,
        node if is_large(node) => true,
        node if !path.contains(&node.to_string()) => true,
        _ => { // contains this node before, need to check if this is the only
            let small_cave = path.iter().filter(|node| is_small(*node));
            small_cave.clone().count() == small_cave.unique().count()
        }
    }
}

fn dfs2(graph: &HashMap<String, Vec<String>>, node: String, path: &mut Vec<String>, result: &mut u64) {
    if node == "end" {
        // print_path(path);
        *result += 1;
        return
    }

    path.push(node.to_string());
    if let Some(neis) = graph.get(&node) {
        for nei in neis {
            if is_legal_move(path, nei) {
                dfs2(graph, nei.clone(), path, result);
            }
        }
    }
    path.pop();
}

pub fn part2(input: &str) -> u64 {
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();
    for line in input.lines() {
        let (a, b) = line.split_once("-").unwrap();
        if !graph.contains_key(a) {
            graph.insert(a.to_string(), Vec::new());
        }
        graph.get_mut(a).unwrap().push(b.to_string());
        if !graph.contains_key(b) {
            graph.insert(b.to_string(), Vec::new());
        }
        graph.get_mut(b).unwrap().push(a.to_string());
    }

    // dfs
    let mut ans = 0;
    let mut path = Vec::new();
    dfs2(&mut graph, String::from("start"), &mut path, &mut ans);
    ans
}

#[cfg(test)]
mod tests {
    #[test]
    fn example1a() {
        assert_eq!(
            super::part1(
                "\
dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc"
            ), 19
        )
    }

    #[test]
    fn example1b() {
        assert_eq!(
            super::part1(
                "\
fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW"
            ), 226
        )
    }

    #[test]
    fn example2() {
        assert_eq!(
            super::part2(
                "\
start-A
start-b
A-c
A-b
b-d
A-end
b-end"
            ), 36
        )
    }

    #[test]
    fn example2a() {
        assert_eq!(
            super::part2(
                "\
dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc"
            ), 103
        )
    }

    #[test]
    fn example2b() {
        assert_eq!(
            super::part2(
                "\
fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW"
            ), 3509
        )
    }

//     #[test]
//     fn example2() {
//         assert_eq!(
//             super::part2(
//                 "\
// 5483143223
// 2745854711
// 5264556173
// 6141336146
// 6357385478
// 4167524645
// 2176841721
// 6882881134
// 4846848554
// 5283751526"
//             ), 195
//         )
//     }
}
