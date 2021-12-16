fn simulation(initial_fish: &[u64], simulation_days: u64) -> u64 {
    let mut simulation: Vec<u64> = vec![0; 9];
    for fish in initial_fish {
        simulation[*fish as usize] += 1;
    }

    for _ in 0..simulation_days {
        let reproduce_count = simulation[0];
        simulation[0] = 0; // clear this bit because we don't want to wrap around
        simulation.rotate_left(1);
        // there are some more new fish
        simulation[8] = reproduce_count;
        simulation[6] += reproduce_count;
    }

    simulation.iter().sum()
}

pub fn part1(initial_fish: &[u64]) -> u64 {
    simulation(initial_fish, 80)
}

pub fn part2(initial_fish: &[u64]) -> u64 {
    simulation(initial_fish, 256)
}
