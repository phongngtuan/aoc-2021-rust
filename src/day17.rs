fn hit_target(x_min: i64, x_max: i64, y_min: i64, y_max: i64, mut dx: i64, mut dy: i64) -> Option<i64> {
    let (mut x, mut y) = (0, 0);
    let mut top_y = 0;
    loop {
        x += dx;
        y += dy;
        top_y = top_y.max(y);

        if x_min <= x && x <= x_max && y_min <= y && y <= y_max {
            return Some(top_y)
        }

        if y < y_min || x > x_max {
            return None
        }

        if dx > 0 {
            dx -= 1;
        } else if dx < 0 {
            dx += 1;
        }
        dy -= 1;
    }
}

pub fn part1(input: &str) -> i64 {
    let (x_bounds, y_bounds) = input.trim_start_matches("target area: ").split_once(',').unwrap();
    let (x_min, x_max) = x_bounds.trim().trim_start_matches("x=").split_once("..").unwrap();
    let (y_min, y_max) = y_bounds.trim().trim_start_matches("y=").split_once("..").unwrap();
    let x_min: i64 = x_min.parse().unwrap();
    let x_max: i64 = x_max.parse().unwrap();
    let y_min: i64 = y_min.parse().unwrap();
    let y_max: i64 = y_max.parse().unwrap();
    println!("{} {} {} {}", x_min, x_max, y_min, y_max);

    let y_bound = y_min.abs().max(y_max.abs());

    let mut ans = 0;
    for dx in 0..=x_max {
        for dy in -y_bound..=y_bound {
            if let Some(top_y) = hit_target(x_min, x_max, y_min, y_max, dx, dy) {
                ans = ans.max(top_y);
            }
        }
    }
    ans
}

pub fn part2(input: &str) -> i64 {
    let (x_bounds, y_bounds) = input.trim_start_matches("target area: ").split_once(',').unwrap();
    let (x_min, x_max) = x_bounds.trim().trim_start_matches("x=").split_once("..").unwrap();
    let (y_min, y_max) = y_bounds.trim().trim_start_matches("y=").split_once("..").unwrap();
    let x_min: i64 = x_min.parse().unwrap();
    let x_max: i64 = x_max.parse().unwrap();
    let y_min: i64 = y_min.parse().unwrap();
    let y_max: i64 = y_max.parse().unwrap();
    println!("{} {} {} {}", x_min, x_max, y_min, y_max);

    let y_bound = y_min.abs().max(y_max.abs());

    let mut ans = 0;
    for dx in 0..=x_max {
        for dy in -y_bound..=y_bound {
            if hit_target(x_min, x_max, y_min, y_max, dx, dy).is_some() {
                ans += 1;
            }
        }
    }
    ans
}

#[cfg(test)]
mod tests {
    #[test]
    fn example1() {
        assert_eq!(
            super::part1(
                "target area: x=20..30, y=-10..-5"
            ), 45
        )
    }

    #[test]
    fn day1() {
        assert_eq!(
            super::part1(
                include_str!("input/day17.txt")
            ), 13041
        )
    }

    #[test]
    fn example2() {
        assert_eq!(
            super::part2(
                "target area: x=20..30, y=-10..-5"
            ), 112
        )
    }

    #[test]
    fn day2() {
        assert_eq!(
            super::part2(
                include_str!("input/day17.txt")
            ), 1031
        )
    }
}
