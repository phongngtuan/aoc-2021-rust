fn error_score(c: &char) -> u64 {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _   => 0,
    }
}

fn complete_score(c: &char) -> u64 {
    match c {
        '(' => 1,
        '[' => 2,
        '{' => 3,
        '<' => 4,
        _   => 0,
    }
}

fn complete_score_string(s: &[char]) -> u64 {
    let mut score = 0;
    for c in s.iter().rev() {
        print!("{} ", c);
        score = score * 5 + complete_score(c);
    }
    score
}

fn check_corrupt(line: &str) -> Result<Vec<char>, char> {
    let mut stack = Vec::new();
    for char in line.chars() {
        match char {
            ')' => if stack.pop() != Some('(') {
                return Err(char);
            },
            ']' => if stack.pop() != Some('[') {
                return Err(char);
            },
            '}' => if stack.pop() != Some('{') {
                return Err(char);
            },
            '>' => if stack.pop() != Some('<') {
                return Err(char);
            },
            _ => stack.push(char),
        }
    }
    Ok(stack)
}

pub fn part1(lines: &[String]) -> u64 {
    let mut errors: Vec<char> = Vec::new();
    for line in lines {
        match check_corrupt(line) {
            Err(corruption) => errors.push(corruption),
            _ => (),
        }
    }
    errors.iter().map(error_score).sum()
}

pub fn part2(lines: &[String]) -> u64 {
    let mut complete_score: Vec<u64> = Vec::new();
    for line in lines {
        match check_corrupt(line) {
            Ok(stack) => {
                let score = complete_score_string(&stack);
                println!("{}", score);
                complete_score.push(score);
            },
            _ => (),
        }
    }
    complete_score.sort_unstable();
    complete_score[complete_score.len() / 2]
}
