pub fn part1(input: &[u32]) -> usize {
  input
    .windows(2)
    .filter(|w| w[0] < w[1])
    .count()
}

pub fn part2(input: &[u32]) -> usize {
  let windows: Vec<u32> = input
    .windows(3)
    .map(|w| w.iter().sum())
    .collect();

  windows
    .windows(2)
    .filter(|w| w[0] < w[1])
    .count()
}