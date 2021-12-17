use std::collections::HashMap;

pub type Board = Vec<Vec<i32>>;

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }
}

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Line {
    start: Point,
    end: Point
}

impl Line {
    fn is_diagonal(&self) -> bool {
        (self.start.x != self.end.x) && (self.start.y != self.end.y)
    }

    fn points(&self) -> LineIter {
        let dx = if self.end.x == self.start.x {
            0
        } else if self.end.x > self.start.x {
            1
        } else {
            -1
        };

        let dy = if self.end.y == self.start.y {
            0
        } else if self.end.y > self.start.y {
            1
        } else {
            -1
        };

        LineIter {
            current: self.start,
            end: self.end,
            d: Point::new(dx, dy),
            done: false,
        }
    }
}

struct LineIter {
    current: Point,
    end: Point,
    d: Point,
    done: bool,
}

impl Iterator for LineIter {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            None
        } else {
            let point = self.current;
            self.current.x += self.d.x;
            self.current.y += self.d.y;
            if point == self.end {
                self.done = true;
            }
            Some(point)
        }
    }
}

fn parse_point(s: &str) -> Point {
    let tokens: Vec<String> = s.split(',').map(String::from).collect();
    let x: i32 = tokens[0].parse().unwrap();
    let y: i32 = tokens[1].parse().unwrap();
    Point { x, y }
}

fn parse_line(s: &str) -> Line {
    let tokens: Vec<String> = s.split(" -> ").map(String::from).collect();
    let start = parse_point(&tokens[0]);
    let end: Point = parse_point(&tokens[1]);
    Line { start, end }
}

pub fn part1(lines: &[String]) -> i32 {
    let mut map: HashMap<Point, i32> = HashMap::new();
    let mut result = 0;

    for input in lines {
        let line = parse_line(input);
        if !line.is_diagonal() {
            for point in line.points() {
                let intersect_count= map.entry(point).or_insert(0);
                if *intersect_count == 1 {
                    result += 1;
                }
                *intersect_count += 1;
            }
        }
    }

    result
}

pub fn part2(lines: &[String]) -> i32 {
    let mut map: HashMap<Point, i32> = HashMap::new();
    let mut result = 0;

    for input in lines {
        let line = parse_line(input);
        for point in line.points() {
            let intersect_count= map.entry(point).or_insert(0);
            if *intersect_count == 1 {
                result += 1;
            }
            *intersect_count += 1;
        }
    }

    result
}
