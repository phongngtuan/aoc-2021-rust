use std::str::FromStr;

enum Command {
  Down(i32),
  Up(i32),
  Forward(i32),
}

struct DirectionParseError;

impl FromStr for Command {
  type Err = DirectionParseError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let tokens: Vec<&str> = s.split(' ').collect();
    if tokens.len() != 2 {
      Err(DirectionParseError)
    } else {
      let d = tokens[0];
      let u = tokens[1];

      if let Ok(unit) = u.parse() {
        match d {
          "down" => Ok(Command::Down(unit)),
          "up" => Ok(Command::Up(unit)),
          "forward" => Ok(Command::Forward(unit)),
          _ => Err(DirectionParseError)
        }
      } else {
        Err(DirectionParseError)
      }
    }
  }
}

#[derive(Default)]
struct Submarine {
  horizontal: i32,
  depth: i32,
  aim: i32,
}

pub fn part1(input: &[String]) -> i32 {
  fn transform1(sub: &mut Submarine, command: &Command) {
    match command {
      Command::Down(x) => sub.depth += x,
      Command::Up(x) => sub.depth -= x,
      Command::Forward(x) => sub.horizontal += x,
    }
  }

  let commands = input
    .iter()
    .filter_map(|s| Command::from_str(s).ok());

  let mut sub = Submarine::default();
  for command in commands {
    transform1(&mut sub, &command);
  }

  sub.horizontal * sub.depth
}

pub fn part2(input: &[String]) -> i32 {
  fn transform(sub: &mut Submarine, command: &Command) {
    match command {
      Command::Down(x) => sub.aim += x,
      Command::Up(x) => sub.aim -= x,
      Command::Forward(x) => {
        sub.horizontal += x;
        sub.depth += x * sub.aim;
      },
    }
  }

  let commands = input
    .iter()
    .filter_map(|s| Command::from_str(s).ok());

  let mut sub = Submarine::default();
  for command in commands {
    transform(&mut sub, &command);
  }

  sub.horizontal * sub.depth
}