use std::collections::HashMap;
use itertools::{Itertools, MinMaxResult};

fn simulation(input: &str, count: usize) -> u64 {
    let mut input = input.lines();
    let s: Vec<char> = input.next().unwrap().chars().collect();
    let first = s[0];
    let last = s[s.len()-1];

    let mut rules: HashMap<(char, char), char> = HashMap::new();
    while let Some(rule) = input.next() {
        let rule = rule.trim();
        if !rule.is_empty() {
            let (from, to) = rule.split_once(" -> ").unwrap();
            let key: Vec<char> = from.chars().collect();
            let val: char = to.chars().next().unwrap();
            rules.insert((key[0], key[1]), val);
        }
    }

    let mut state: HashMap<(char, char), usize> = HashMap::new();
    for idx in 0..s.len()-1 {
        let pair = (s[idx], s[idx+1]);
        *state.entry(pair).or_default() += 1;
    }

    for _ in 0..count {
        let mut new_state: HashMap<(char, char), usize> = HashMap::new();
        for ((a, b), count) in state {
            if let Some(insert) = rules.get(&(a, b)) {
                *new_state.entry((a, *insert)).or_default() += count;
                *new_state.entry((*insert, b)).or_default() += count;
            } else {
                *new_state.entry((a, b)).or_default() += count;
            }
        }
        state = new_state;

    }

    let mut freqs: HashMap<char, usize> = HashMap::new();
    for ((a, b), count) in &state {
        *freqs.entry(*a).or_default() += count;
        *freqs.entry(*b).or_default() += count;
    }
    *freqs.entry(first).or_default() += 1;
    *freqs.entry(last).or_default() += 1;
    for val in freqs.values_mut() {
        *val = *val / 2;
    }
    match freqs.values().minmax() {
        MinMaxResult::MinMax(a, b) => (*b-*a) as u64,
        _ => 0
    }
}

pub fn part1(input: &str) -> u64 {
    simulation(input, 10)
}


pub fn part2(input: &str) -> u64 {
    simulation(input, 40)
}

#[cfg(test)]
mod tests {
    #[test]
    fn example1() {
        assert_eq!(
            super::part1(
                "\
NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C"
            ), 1588
        )
    }

    #[test]
    fn example2() {
        assert_eq!(
            super::part2(
                "\
NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C"
            ), 2188189693529
        )
    }
}
