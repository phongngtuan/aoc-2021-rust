pub fn part1(input: &[String]) -> usize {
    let sample_count = input.len();
    let majority_threshold = sample_count / 2;

    if sample_count <= 0 {
        return 0;
    }

    let bit_count = input[0].len();
    let mut one_freq: Vec<usize> = vec![0; bit_count];

    for line in input {
        for (index, bit) in line.chars().enumerate() {
            if bit == '1' {
                one_freq[index] += 1;
            }
        }
    }

    //epsilon = most commmon
    //gamme = least common
    let mut epsilon: usize = 0;
    let mut gamma: usize = 0;
    for freq in one_freq {
        epsilon *= 2;
        gamma *= 2;
        if freq > majority_threshold {
            epsilon += 1;
        } else {
            gamma += 1;
        }
    }

    epsilon * gamma
}

fn choosing<'a, F>(input: &'a [String], bit_criteria: F) -> usize
where
    F: Fn(Vec<&'a str>, Vec<&'a str>) -> Vec<&'a str>,
{
    let mut input: Vec<&str> = input.iter().map(|x| x.as_str()).collect();
    let mut index: usize = 0;
    let mut counts: (Vec<&str>, Vec<&str>) = input
        .iter()
        .partition(|x| x.chars().nth(index) == Some('0'));
    while counts.0.len() + counts.1.len() > 1 {
        input = bit_criteria(counts.0, counts.1);
        index += 1;
        counts = input
            .iter()
            .partition(|x| x.chars().nth(index) == Some('0'));
    }

    let result = counts.0.last().or(counts.1.last()).unwrap();
    usize::from_str_radix(result, 2).unwrap()
}

fn find_oxygen_generator<'a>(zeros: Vec<&'a str>, ones: Vec<&'a str>) -> Vec<&'a str> {
    if zeros.len() > ones.len() {
        zeros
    } else {
        ones
    }
}

fn find_co2_scrubber<'a>(zeros: Vec<&'a str>, ones: Vec<&'a str>) -> Vec<&'a str> {
    if zeros.len() <= ones.len() {
        zeros
    } else {
        ones
    }
}

pub fn part2(input: &[String]) -> usize {
    choosing(input, find_co2_scrubber) * choosing(input, find_oxygen_generator)
}

mod tests {
    use crate::day3::*;

    #[test]
    fn test_part1() {
        let input: Vec<String> = "00100
      11110
      10110
      10111
      10101
      01111
      00111
      11100
      10000
      11001
      00010
      01010"
            .lines()
            .map(|x| x.trim().to_string())
            .collect();
        let part1_result = part1(&input);
        assert_eq!(part1_result, 198);
    }

    #[test]
    fn test_part2() {
        let input: Vec<String> = "00100
      11110
      10110
      10111
      10101
      01111
      00111
      11100
      10000
      11001
      00010
      01010"
            .lines()
            .map(|x| x.trim().to_string())
            .collect();
        let part2_result = part2(&input);
        assert_eq!(part2_result, 230);
    }
}
