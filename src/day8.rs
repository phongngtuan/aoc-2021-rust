use std::collections::HashMap;

use itertools::*;

pub fn part1(lines: &[String]) -> usize {
    let mut segment_index: HashMap<usize, Vec<u8>> = HashMap::new();
    segment_index.insert(2, vec![1]);
    segment_index.insert(3, vec![7]);
    segment_index.insert(4, vec![4]);
    segment_index.insert(7, vec![8]);

    let mut ans = 0;
    for line in lines {
        let tokens: Vec<&str> = line.split("|").collect();
        let output = tokens[1];
        let digits: Vec<&str> = output.split(" ").collect();
        for digit in digits {
            let len = digit.len();
            if let Some(digit) = segment_index.get(&len) {
                if digit.len() == 1 {
                    ans += 1;
                }
            }
        }
    }

    ans
}

#[allow(dead_code)]
struct Mapping {
    forward_mapping: Vec<u8>,
    inverse_mapping: Vec<u8>,
    alphabets: Vec<u8>,
}

impl Mapping {

    fn from(forward_mapping: &[u8]) -> Self {
        let orig_segments: Vec<u8> = (0..=9).map(|d| digit_to_segment(d)).collect();

        let inverse_mapping: Vec<u8> = inverse(&forward_mapping);
        let forward_mapping: Vec<u8> = forward_mapping.iter().cloned().collect();
        let alphabets: Vec<u8> = orig_segments.iter().map(|segment| apply_mapping(*segment, &forward_mapping)).collect();
        let mapping = Mapping {
            forward_mapping, inverse_mapping, alphabets
        };
        mapping
    }
}

fn generate_all_mappings() -> Vec<Mapping> {
    let unit_mapping: Vec<u8> = vec![0,1,2,3,4,5,6];

    let mut all_mappings: Vec<Mapping> = Vec::new();
    for mapping in unit_mapping.iter().permutations(7) {
        let forward_mapping: Vec<u8> = mapping.iter().map(|&x| *x).collect();
        all_mappings.push(Mapping::from(&forward_mapping));
    }
    all_mappings
}

pub fn part2(lines: &[String]) -> u64 {
    let mut ans: u64 = 0;
    let all_mappings: Vec<Mapping> = generate_all_mappings();
    // let mapping: Vec<u8> = vec![2,5,6,0,1,3,4];
    // let all_mappings: Vec<Mapping> = vec![Mapping::from(&mapping)];

    for line in lines {
        let tokens: Vec<&str> = line.split("|").collect();
        let inputs: Vec<&str> = tokens[0].trim().split(" ").collect();
        let outputs: Vec<&str> = tokens[1].trim().split(" ").collect();

        let mut line_output: u64 = 0;

        'mapping: for mapping in &all_mappings {
            for &input in &inputs {
                let segment = bitmask(input);
                if !mapping.alphabets.contains(&segment) {
                    continue 'mapping
                }
            }

            for &output in &outputs {
                let segment = bitmask(output);
                if !mapping.alphabets.contains(&segment) {
                    continue 'mapping
                }
                let digit = segment_to_digit(apply_mapping(segment, &mapping.inverse_mapping));
                line_output = line_output * 10 + digit as u64;
            }
            ans += line_output;
        }
    }
    ans
}

fn bitmask(word: &str) -> u8 {
    let mut ans = 0;
    for char in word.chars() {
        let index = 'g' as u8 - char as u8;
        ans |= 1 << index;
    }
    ans
}

#[allow(dead_code)]
fn format_digit(digit: u8) -> String {
    let segment = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g'];
    let mut output: String = String::new();
    for (i, segment) in segment.iter().enumerate() {
        // index 0 = 'a', shift left 6
        if digit & (1 << (6-i)) > 0 {
            output.push(*segment);
        }
    }
    output
}

fn apply_mapping(input: u8, mapping: &[u8]) -> u8 {
    let mut output: u8 = 0;
    for &pos in mapping {
        output = output << 1;
        let bit = (input >> (7-1-pos)) % 2;
        output |= bit;
    }
    output
}

fn inverse(mapping: &[u8]) -> Vec<u8> {
    let mut inverse_mapping: Vec<u8> = vec![0; mapping.len()];
    for (index, pos) in mapping.iter().enumerate() {
        let x = *pos;
        inverse_mapping[x as usize] = index as u8;
    }
    inverse_mapping
}

fn digit_to_segment(digit: u8) -> u8 {
    match digit {
        0 => bitmask("abcefg"),
        1 => bitmask("cf"),
        2 => bitmask("acdeg"),
        3 => bitmask("acdfg"),
        4 => bitmask("bcdf"),
        5 => bitmask("abdfg"),
        6 => bitmask("abdefg"),
        7 => bitmask("acf"),
        8 => bitmask("abcdefg"),
        9 => bitmask("abcdfg"),
        _ => unreachable!()
    }
}

fn segment_to_digit(segment: u8) -> u8 {
    (0..=9).find(|digit| digit_to_segment(*digit) == segment).unwrap()
}

mod tests {
    #[test]
    fn test_format() {
        assert_eq!(super::format_digit(super::bitmask("cf")), "cf");
    }

    #[test]
    fn test_bitmask() {
        assert_eq!(super::bitmask("abcdefg"), 0b1111111);
        assert_eq!(super::bitmask("bcdefg"), 0b0111111);
    }

    #[test]
    fn test_bitmask_equality() {
        assert_eq!(super::bitmask("abc"), super::bitmask("bca"));
    }

    #[test]
    fn test_apply_mapping() {
        let mapping = vec![3, 0, 1, 2, 4, 6, 5];
        assert_eq!(super::apply_mapping(super::digit_to_segment(1), &mapping), 0b1001);
    }

    #[test]
    fn test_inverse_mapping() {
        let unit = vec![1, 2, 3, 0, 4, 6, 5];
        let mapping = vec![3, 0, 1, 2, 4, 6, 5];
        assert_eq!(super::inverse(&mapping), unit);
    }

    #[test]
    fn test_inverse_inverse() {
        let segment = super::digit_to_segment(1);
        let mapping = vec![3, 0, 1, 2, 4, 6, 5];
        let mapped_once = super::apply_mapping(segment, &mapping);
        let mapped_twice = super::apply_mapping(mapped_once, &super::inverse(&mapping));
        assert_eq!(segment, mapped_twice);
    }

    #[test]
    fn test_sample_mapping() {
        let mapping= vec![2,5,6,0,1,3,4];
        // acedgfb: 8
        // cdfbe: 5
        // gcdfa: 2
        // fbcad: 3
        // dab: 7
        // cefabd: 9
        // cdfgeb: 6
        // eafb: 4
        // cagedb: 0
        // ab: 1
        assert_eq!(super::apply_mapping(super::digit_to_segment(0), &mapping), super::bitmask("cagedb"));
        assert_eq!(super::apply_mapping(super::digit_to_segment(1), &mapping), super::bitmask("ab"));
        assert_eq!(super::apply_mapping(super::digit_to_segment(2), &mapping), super::bitmask("gcdfa"));
        assert_eq!(super::apply_mapping(super::digit_to_segment(3), &mapping), super::bitmask("fbcad"));
        assert_eq!(super::apply_mapping(super::digit_to_segment(4), &mapping), super::bitmask("eafb"));
        assert_eq!(super::apply_mapping(super::digit_to_segment(5), &mapping), super::bitmask("cdfbe"));
        assert_eq!(super::apply_mapping(super::digit_to_segment(6), &mapping), super::bitmask("cdfgeb"));
        assert_eq!(super::apply_mapping(super::digit_to_segment(7), &mapping), super::bitmask("dab"));

        let inverse_mapping = super::inverse(&mapping);
        assert_eq!(super::apply_mapping(super::bitmask("ab"), &inverse_mapping), super::digit_to_segment(1));
    }
}