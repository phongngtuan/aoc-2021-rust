use std::collections::VecDeque;
use std::fmt::{Display, Formatter};
use itertools::{enumerate, Itertools};
use crate::day18::Num::Pair;


enum Num {
    Pair(Box<Num>, Box<Num>),
    Literal(i64)
}

enum Action {
    Exploded(Explosion),
    Splitted(Splitting)
}

struct Explosion(Option<Box<Num>>, Option<i64>, Option<i64>);
struct Splitting(Option<Box<Num>>);

impl Num {
    fn magnitude(&self) -> i64 {
        match self {
            Num::Literal(num) => *num,
            Num::Pair(left, right) => {
                left.magnitude() * 3 + right.magnitude() * 2
            },
        }
    }

    fn add(self, other: Box<Self>) -> Self {
        let left = Box::new(self);
        let mut tree = Num::Pair(left, other);
        tree.reduce();
        tree
    }

    fn explode(&mut self) -> Option<Explosion> {
        self.explode_depth(0)
    }

    fn add_left(&mut self, value: i64) {
        match self {
            Num::Pair(left, _) => left.add_left(value),
            Num::Literal(num) => *num += value,
        }
    }

    fn add_right(&mut self, value: i64) {
        match self {
            Num::Pair(_, right) => right.add_right(value),
            Num::Literal(num) => *num += value,
        }
    }

    fn reduce(&mut self) {
        while let Some(_) = self.action() {
            ()
        }
    }

    fn action(&mut self) -> Option<Action> {
        self.explode().map(Action::Exploded).or_else(|| self.split_node().map(Action::Splitted))
    }

    fn split_node(&mut self) -> Option<Splitting> {
        match self {
            Num::Literal(num) if *num >= 10 => {
                let first = *num / 2;
                let second = *num - first;
                Some(Splitting(Some(Box::new(Num::Pair(Box::new(Num::Literal(first)), Box::new(Num::Literal(second)))))))
            },
            Num::Pair(left, right) => {
                match left.split_node() {
                    Some(Splitting(Some(new_node))) => {
                        *left = new_node;
                        Some(Splitting(None))
                    },
                    None => {
                        // now go right
                        match right.split_node() {
                            Some(Splitting(Some(new_node))) => {
                                *right = new_node;
                                Some(Splitting(None))
                            },
                            none => none
                        }
                    },
                    x => x,
                }
            },
            _ => None
        }
    }

    fn explode_depth(&mut self, depth: usize) -> Option<Explosion> {
        // the first to explode is always a pair of number
        // the first one we find is to explode
        match self {
            Num::Pair(box left, box right) =>
            match (&left, &right) {
                (Num::Literal(left), Num::Literal(right)) => {
                    // left most pair
                    if depth >= 4 {
                        Some(Explosion(Some(Box::new(Num::Literal(0))), Some(*left), Some(*right)))
                    } else {
                        None
                    }
                }
                (_, _) => {
                    // first going left
                    let explosion = left.explode_depth(depth + 1);
                    match explosion {
                        Some(explosion) => {
                            let Explosion(mut new_node, mut left_cout, mut right_cout) = explosion;
                            match new_node {
                                Some(node) => {
                                    *left = *node;
                                    new_node = None;
                                },
                                _ => (),
                            }
                            match right_cout {
                                Some(val) => {
                                    right.add_left(val);
                                    right_cout = None;
                                },
                                _ => ()
                            }
                            return Some(Explosion(new_node, left_cout, right_cout))
                        },
                        None => {
                            // going right
                            let explosion = right.explode_depth(depth + 1);
                            match explosion {
                                Some(explosion) => {
                                    let Explosion(mut new_node, mut left_cout, mut right_cout) = explosion;
                                    match new_node {
                                        Some(node) => {
                                            *right = *node;
                                            new_node = None;
                                        },
                                        _ => (),
                                    }
                                    match left_cout {
                                        Some(val) => {
                                            left.add_right(val);
                                            left_cout = None;
                                        },
                                        _ => ()
                                    }
                                    Some(Explosion(new_node, left_cout, right_cout))
                                }
                                _ => None
                            }
                        }
                    }
                },
            }
            _ => None, // Can't explode a number, continue searching
        }
    }
}

impl ToString for Num {
    fn to_string(&self) -> String {
        match self {
            Num::Literal(num) => num.to_string(),
            Num::Pair(left, right) => format!("[{},{}]", left.to_string(), right.to_string())
        }
    }
}

impl Num {
    fn expect(s: &mut VecDeque<char>, c: char) {
        let peek = s.front();
        match peek {
            None => panic!("Unexpected end of input"),
            Some(x) if *x != c => panic!("Expected ',' but found {:?}", x),
            _ => s.pop_front(),
        };
    }

    fn parse(s: &mut VecDeque<char>) -> Box<Self> {
        match s.front() {
            Some('[') => {
                Num::expect(s, '[');
                let left = Num::parse(s);
                Num::expect(s, ',');
                let right = Num::parse(s);
                Num::expect(s, ']');
                Box::new(Num::Pair(left, right))
            },
            Some(x) if x.is_digit(10) => {
                let mut num = 0;
                while let Some(d) = s.front() {
                    match d.to_digit(10) {
                        Some(x) => {
                            num = num * 10 + x as i64;
                            s.pop_front();
                        },
                        _ => break,
                    }
                }
                Box::new(Num::Literal(num))
            }
            _ => panic!()
        }
    }
}

fn add(left: Box<Num>, right: Box<Num>) -> Box<Num> {
    let mut new_tree = Num::Pair(left, right);
    new_tree.reduce();
    Box::new(new_tree)
}

fn solve_magnitude(lines: &[&str]) -> i64 {
    let mut left: Option<Box<Num>> = None;
    for line in lines {
        let right = Num::parse(&mut line.chars().collect());
        if left.is_none() {
            left = Some(right);
            continue
        }
        left = Some(add(left.unwrap(), right));
        match &left {
            Some(tree) => println!("{}", tree.to_string()),
            _ => (),
        }
    }
    let ans = left.unwrap();
    println!("{}", ans.to_string());
    ans.magnitude()
}

pub fn part1(input: &str) -> i64 {
    solve_magnitude(&input.lines().collect_vec())
}

pub fn part2(input: &str) -> i64 {
    input.lines().tuple_combinations().map(|(a, b)| solve_magnitude(&[a, b])).max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing() {
        let input = "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]";
        let mut queue = input.chars().collect();
        assert_eq!(
            Num::parse(&mut queue).to_string(),
            input
        )
    }

    #[test]
    fn test_explode() {
        for (input, output) in [
            ("[[[[[9,8],1],2],3],4]", "[[[[0,9],2],3],4]"),
            ("[7,[6,[5,[4,[3,2]]]]]", "[7,[6,[5,[7,0]]]]"),
            ("[[6,[5,[4,[3,2]]]],1]", "[[6,[5,[7,0]]],3]"),
            ("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]", "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]"),
            ("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]", "[[3,[2,[8,0]]],[9,[5,[7,0]]]]")
        ] {
            let mut queue = input.chars().collect();
            let mut tree = Num::parse(&mut queue);
            assert!(tree.explode().is_some());
            assert_eq!(tree.to_string(), output)
        }
    }

    #[test]
    fn test_split() {
        for (input, output) in [
            ("[[[[0,7],4],[15,[0,13]]],[1,1]]", "[[[[0,7],4],[[7,8],[0,13]]],[1,1]]"),
            ("[[[[0,7],4],[[7,8],[0,13]]],[1,1]]", "[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]"),
        ] {
            let mut queue = input.chars().collect();
            let mut tree = Num::parse(&mut queue);
            assert!(tree.split_node().is_some());
            assert_eq!(tree.to_string(), output)
        }
    }

    #[test]
    fn test_reduce() {
        for (input, output) in [
            ("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]", "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"),
        ] {
            let mut queue = input.chars().collect();
            let mut tree = Num::parse(&mut queue);
            tree.reduce();
            assert_eq!(tree.to_string(), output)
        }
    }

    #[test]
    fn test_add() {
        for (a, b, output) in [
            ("[[[[4,3],4],4],[7,[[8,4],9]]]", "[1,1]", "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"),
        ] {
            let mut tree_a = Num::parse(&mut a.chars().collect());
            let mut tree_b = Num::parse(&mut b.chars().collect());
            let tree = tree_a.add(tree_b);
            assert_eq!(tree.to_string(), output)
        }
    }

//     #[test]
//     fn example1a() {
//         let input = "\
// [[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
// [7,[[[3,7],[4,3]],[[6,3],[8,8]]]]
// [[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]
// [[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]
// [7,[5,[[3,8],[1,4]]]]
// [[2,[2,2]],[8,[8,1]]]
// [2,9]
// [1,[[[9,3],9],[[9,0],[0,7]]]]
// [[[5,[7,4]],7],1]
// [[[[4,2],2],6],[8,7]]";
//         part1(input);
//     }

    #[test]
    fn example1() {
        let input = "\
[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]";
        // assert_eq!(part1(input), 4140);
    }

    #[test]
    fn part1() {
        assert_eq!(
            super::part1(
                include_str!("input/day18.txt")
            ), 4391
        )
    }

    #[test]
    fn example2() {
        assert_eq!(
            super::part2("\
[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]"
            ), 3993
        )
    }

    #[test]
    fn part2() {
        assert_eq!(
            super::part2(
                include_str!("input/day18.txt")
            ), 1031
        )
    }
}
