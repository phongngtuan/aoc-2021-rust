use std::collections::{HashMap, HashSet, VecDeque};
use std::ops::{Add, Mul, Sub};
use itertools::{Itertools};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Point([i64; 3]);

impl ToString for Point {
    fn to_string(&self) -> String {
        let points = self.0;
        format!("({},{},{})", points[0], points[1], points[2])
    }
}

impl Point {
    fn from(x: i64, y: i64, z: i64) -> Self {
        Point([x, y, z])
    }

    fn zero() -> Self {
        Point([0, 0, 0])
    }

    fn manhattan_dist(&self, rhs: &Point) -> u64 {
        let a = self.0;
        let b = rhs.0;
        let dx = (a[0] - b[0]).abs() as u64;
        let dy = (a[1] - b[1]).abs() as u64;
        let dz = (a[2] - b[2]).abs() as u64;
        dx + dy + dz
    }
}

impl Mul for Point {
    type Output = Point;
    /// Cross product
    fn mul(self, rhs: Point) -> Self::Output {
        let a = self.0;
        let b = rhs.0;
        let x = a[1] * b[2] - a[2] * b[1];
        let y = -(a[0] * b[2] - a[2] * b[0]);
        let z = a[0] * b[1] - a[1] * b[0];
        Point([x, y, z])
    }
}

impl Add for Point {
    type Output = Point;
    fn add(self, rhs: Self) -> Self::Output {
        let a = self.0;
        let b = rhs.0;
        Point([a[0] + b[0], a[1] + b[1], a[2] + b[2]])
    }
}

impl Sub for Point {
    type Output = Point;
    fn sub(self, rhs: Self) -> Self::Output {
        let a = self.0;
        let b = rhs.0;
        Point([a[0] - b[0], a[1] - b[1], a[2] - b[2]])
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
struct Matrix([[i64; 3]; 3]);

impl Matrix {
    fn zero() -> Self {
        Matrix([[0; 3]; 3])
    }

    fn identity() -> Self { Matrix([[1, 0, 0], [0, 1, 0], [0, 0, 1]]) }
}

impl Matrix {
    fn rows(&self) -> Vec<Point> {
        let rows = self.0;
        vec![Point(rows[0]), Point(rows[1]), Point(rows[2])]
    }
}

impl Mul<Point> for Matrix {
    type Output = Point;
    fn mul(self, rhs: Point) -> Self::Output {
        let mut coordinates = [0, 0, 0];
        for i in 0..3 {
            for j in 0..3 {
                coordinates[i] += self.0[i][j] * rhs.0[i];
            }
        }
        Point(coordinates)
    }
}

#[derive(Copy, Clone, Debug)]
struct Alignment {
    orientation: [i64; 3],
    translation: Point,
}

impl Alignment {
    fn identity() -> Self {
        let translation = Point::zero();
        Alignment { orientation: [1, 2, 3], translation }
    }

    fn apply(&self, point: Point) -> Point {
        orient(point, &self.orientation) + self.translation
    }

    fn from(a: &[Point], b: &[Point]) -> Option<Self> {
        // find the transformation a to b
        let mut max_score = 0;
        for i in 0..a.len() {
            for j in 0..b.len() {
                // find the transformation that turn a[i] into b[j]
                'orientation: for orientation in ORIENTATIONS {
                    let translation = b[j] - orient(a[i], &orientation);
                    let alignment = Alignment { orientation, translation };
                    // find the score of this alignment
                    let score = alignment.score(a, b);
                    // println!("translation: {}", translation.to_string());
                    max_score = max_score.max(score);
                    if score >= 12 {
                        return Some(alignment);
                    }
                }
            }
        }
        // println!("No alignment found, max score = {}", max_score);
        None
    }

    fn score(&self, a: &[Point], b: &[Point]) -> usize {
        let a: HashSet<_> = a.iter().map(|p| self.apply(*p)).collect();
        let b: HashSet<_> = b.iter().copied().collect();
        a.intersection(&b).count()
    }
}

fn test_orientation(matrix: Matrix) -> bool {
    let rows = matrix.rows();
    rows[0] * rows[1] == rows[2]
}

const ORIENTATIONS: [[i64; 3]; 24] = [
    [1, 2, 3],
    [2, -1, 3],
    [-1, -2, 3],
    [-2, 1, 3],
    [2, 3, 1],
    [-1, 3, 2],
    [-2, 3, -1],
    [1, 3, -2],
    [3, 1, 2],
    [3, 2, -1],
    [3, -1, -2],
    [3, -2, 1],
    [2, 1, -3],
    [-1, 2, -3],
    [-2, -1, -3],
    [1, -2, -3],
    [1, -3, 2],
    [2, -3, -1],
    [-1, -3, -2],
    [-2, -3, 1],
    [-3, 2, 1],
    [-3, -1, 2],
    [-3, -2, -1],
    [-3, 1, -2],
];

fn orient(point: Point, orientation: &[i64; 3]) -> Point {
    let mut result = [0, 0, 0];
    let p = point.0;
    for (write, read) in orientation.iter().enumerate() {
        result[write] = p[read.abs() as usize - 1];
        if *read < 0 {
            result[write] = -result[write];
        }
    }
    Point(result)
}

// try all combinations and filter out those illegal
fn all_orientations() -> Vec<Matrix> {
    let mut ans = Vec::new();
    for permutation in [0, 1, 2].iter().permutations(3) {
        let x = *permutation[0];
        let y = *permutation[1];
        let z = *permutation[2];
        for a in [-1, 1] {
            for b in [-1, 1] {
                for c in [-1, 1] {
                    let mut matrix = Matrix::zero();
                    matrix.0[0][x] = a;
                    matrix.0[1][y] = b;
                    matrix.0[2][z] = c;
                    if test_orientation(matrix) {
                        ans.push(matrix);
                    }
                }
            }
        }
    }
    ans
}

// pub fn dfs(i: usize, j: usize, visited: &mut HashSet<(usize, usize)>, scanners: &Vec<Vec<Point>>, alignments: &mut HashSet<usize, Alignment>) {
//     if visited.contains(&(i, j)) {
//         return;
//     }
//
//     if let Some(alignment_i_j) = Alignment::from(&scanners[j], &scanners[i]) {
//         println!("Found alignment between {} {}, {:?}", i, j, alignment_i_j.translation);
//         for point in &scanners[j] {
//             // find the coordinate of this point in the base orientation
//             let point0 = alignment_i_j.apply(*point);
//
//             // println!("{},{},{}", point0.0[0], point0.0[1], point0.0[2]);
//             all_points.insert(point0);
//         }
//     }
//
// }

fn solve(input: &str) -> (HashSet<Point>, HashMap<usize, Alignment>) {
    let mut scanners: Vec<Vec<Point>> = Vec::new();
    let mut current: Vec<Point> = Vec::new();
    for line in input.lines() {
        if line.starts_with("---") {
            current = Vec::new();
        } else if line.is_empty() {
            scanners.push(current);
            current = Vec::new();
        } else {
            let coordinates: Vec<i64> = line.split(',').map(|x| x.parse().unwrap()).collect();
            current.push(Point([coordinates[0], coordinates[1], coordinates[2]]));
        }
    }
    scanners.push(current);

    let mut all_points = HashSet::new();
    for point in &scanners[0] {
        all_points.insert(*point);
    }

    let mut alignments: HashMap<usize, Alignment> = HashMap::new();
    alignments.insert(0, Alignment::identity());

    let mut queue = VecDeque::new();
    queue.push_back(0);

    while let Some(i) = queue.pop_front() {
        for j in 0..scanners.len() {
            if j == i || alignments.contains_key(&j) {
                continue;
            }

            if let Some(alignment_i_j) = Alignment::from(&scanners[j], &scanners[i]) {
                println!("Found alignment between {} {}, {:?}", i, j, alignment_i_j.translation);
                let mut oriented_scanners = Vec::new();
                for point in &scanners[j] {
                    // find the coordinate of this point in the base orientation
                    let point0 = alignment_i_j.apply(*point);
                    // println!("{},{},{}", point0.0[0], point0.0[1], point0.0[2]);
                    all_points.insert(point0);
                    oriented_scanners.push(point0);
                }

                scanners[j] = oriented_scanners;
                queue.push_back(j);
                alignments.insert(j, alignment_i_j);
            }
        }
    }
    (all_points, alignments)
}

pub fn part1(input: &str) -> i64 {
    let (points, _) = solve(input);
    points.len() as i64
}

pub fn part2(input: &str) -> i64 {
    let (_, alignments) = solve(input);
    alignments
        .values()
        .combinations(2)
        .map(|comb| {
            let a = comb[0];
            let b = comb[1];
            let dist = a.translation.manhattan_dist(&b.translation);
            println!("{:?} {:?} {}", a.translation, b.translation, dist);
            dist
        })
        .max()
        .unwrap() as i64
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_manhattan_distance() {
        let a = Point([1105, -1205, 1229]);
        let b = Point([-92, -2380, -20]);
        assert_eq!(a.manhattan_dist(&b), 3621);
    }

    #[test]
    fn matrix_mul() {
        let matrix = Matrix([
            [1, 0, 0],
            [0, 1, 0],
            [0, 0, 1],
        ]);
        let point = Point([1, 2, 3]);

        assert_eq!(matrix * point, point);
    }

    #[test]
    fn cross_product_unit() {
        let x = Point([1, 0, 0]);
        let y = Point([0, 1, 0]);
        let z = Point([0, 0, 1]);

        assert_eq!(x * y, z);
    }

    #[test]
    fn cross_product_example() {
        let x = Point([-4, 3, 0]);
        let y = Point([2, 0, 0]);
        let z = Point([0, 0, -6]);

        assert_eq!(x * y, z);
    }

    #[test]
    fn test_unit_vector_orientation() {
        let matrix = Matrix([
            [1, 0, 0],
            [0, 1, 0],
            [0, 0, 1],
        ]);
        assert!(test_orientation(matrix))
    }

    #[test]
    fn test_all_orientations() {
        assert_eq!(all_orientations().len(), 24);
    }

    fn test_all_orientations2() {
        let p = Point([1, 2, 3]);
        let mut a = HashSet::new();
        let mut b = HashSet::new();
        for orientation in all_orientations() {
            a.insert(orientation * p);
        }

        for o in &ORIENTATIONS {
            b.insert(orient(p, o));
        }

        assert_eq!(a.len(), b.len());
        assert_eq!(a, b);
    }

    #[test]
    fn test_alignment() {
        let a = vec![
            Point([-618, -824, -621]),
            Point([-537, -823, -458]),
            Point([-447, -329, 318]),
            Point([404, -588, -901]),
            Point([544, -627, -890]),
            Point([528, -643, 409]),
            Point([-661, -816, -575]),
            Point([390, -675, -793]),
            Point([423, -701, 434]),
            Point([-345, -311, 381]),
            Point([459, -707, 401]),
            Point([-485, -357, 347]),
        ];
        let b = vec![
            Point([686, 422, 578]),
            Point([605, 423, 415]),
            Point([515, 917, -361]),
            Point([-336, 658, 858]),
            Point([-476, 619, 847]),
            Point([-460, 603, -452]),
            Point([729, 430, 532]),
            Point([-322, 571, 750]),
            Point([-355, 545, -477]),
            Point([413, 935, -424]),
            Point([-391, 539, -444]),
            Point([553, 889, -390]),
        ];
        let alignment = Alignment::from(&b, &a).unwrap();

        // assert_eq!(alignment.translation, Point([68, -1246, -43]));
        let c = b.iter().map(|point| alignment.apply(*point)).collect_vec();
        assert_eq!(a, c);
    }

    #[test]
    fn part1_small_example() {
        let input = "\
--- scanner 0 ---
-618,-824,-621
-537,-823,-458
-447,-329,318
404,-588,-901
544,-627,-890
528,-643,409
-661,-816,-575
390,-675,-793
423,-701,434
-345,-311,381
459,-707,401
-485,-357,347

--- scanner 1 ---
686,422,578
605,423,415
515,917,-361
-336,658,858
-476,619,847
-460,603,-452
729,430,532
-322,571,750
-355,545,-477
413,935,-424
-391,539,-444
553,889,-390";
        assert_eq!(part1(input), 12);
    }

    #[test]
    fn part1_align_0_1() {
        let input = "\
--- scanner 0 ---
404,-588,-901
528,-643,409
-838,591,734
390,-675,-793
-537,-823,-458
-485,-357,347
-345,-311,381
-661,-816,-575
-876,649,763
-618,-824,-621
553,345,-567
474,580,667
-447,-329,318
-584,868,-557
544,-627,-890
564,392,-477
455,729,728
-892,524,684
-689,845,-530
423,-701,434
7,-33,-71
630,319,-379
443,580,662
-789,900,-551
459,-707,401

--- scanner 1 ---
686,422,578
605,423,415
515,917,-361
-336,658,858
95,138,22
-476,619,847
-340,-569,-846
567,-361,727
-460,603,-452
669,-402,600
729,430,532
-500,-761,534
-322,571,750
-466,-666,-811
-429,-592,574
-355,545,-477
703,-491,-529
-328,-685,520
413,935,-424
-391,539,-444
586,-435,557
-364,-763,-893
807,-499,-711
755,-354,-619
553,889,-390";
        assert_eq!(part1(input), 38);
    }

    #[test]
    fn part1_align_1_4() {
        let input = "\
--- scanner 1 ---
686,422,578
605,423,415
515,917,-361
-336,658,858
95,138,22
-476,619,847
-340,-569,-846
567,-361,727
-460,603,-452
669,-402,600
729,430,532
-500,-761,534
-322,571,750
-466,-666,-811
-429,-592,574
-355,545,-477
703,-491,-529
-328,-685,520
413,935,-424
-391,539,-444
586,-435,557
-364,-763,-893
807,-499,-711
755,-354,-619
553,889,-390

--- scanner 4 ---
727,592,562
-293,-554,779
441,611,-461
-714,465,-776
-743,427,-804
-660,-479,-426
832,-632,460
927,-485,-438
408,393,-506
466,436,-512
110,16,151
-258,-428,682
-393,719,612
-211,-452,876
808,-476,-593
-575,615,604
-485,667,467
-680,325,-822
-627,-443,-432
872,-547,-609
833,512,582
807,604,487
839,-516,451
891,-625,532
-652,-548,-490
30,-46,-14";
        assert_eq!(part1(input), 39);
    }

    //
//     #[test]
//     fn part1_example2() {
//         let input = "\
// --- scanner 0 ---
// 404,-588,-901
// 528,-643,409
// -838,591,734
// 390,-675,-793
// -537,-823,-458
// -485,-357,347
// -345,-311,381
// -661,-816,-575
// -876,649,763
// -618,-824,-621
// 553,345,-567
// 474,580,667
// -447,-329,318
// -584,868,-557
// 544,-627,-890
// 564,392,-477
// 455,729,728
// -892,524,684
// -689,845,-530
// 423,-701,434
// 7,-33,-71
// 630,319,-379
// 443,580,662
// -789,900,-551
// 459,-707,401
//
// --- scanner 1 ---
// 686,422,578
// 605,423,415
// 515,917,-361
// -336,658,858
// 95,138,22
// -476,619,847
// -340,-569,-846
// 567,-361,727
// -460,603,-452
// 669,-402,600
// 729,430,532
// -500,-761,534
// -322,571,750
// -466,-666,-811
// -429,-592,574
// -355,545,-477
// 703,-491,-529
// -328,-685,520
// 413,935,-424
// -391,539,-444
// 586,-435,557
// -364,-763,-893
// 807,-499,-711
// 755,-354,-619
// 553,889,-390";
//         assert_eq!(part1(input), 38);
//     }
//
//
    const EXAMPLE: &str = "\
--- scanner 0 ---
404,-588,-901
528,-643,409
-838,591,734
390,-675,-793
-537,-823,-458
-485,-357,347
-345,-311,381
-661,-816,-575
-876,649,763
-618,-824,-621
553,345,-567
474,580,667
-447,-329,318
-584,868,-557
544,-627,-890
564,392,-477
455,729,728
-892,524,684
-689,845,-530
423,-701,434
7,-33,-71
630,319,-379
443,580,662
-789,900,-551
459,-707,401

--- scanner 1 ---
686,422,578
605,423,415
515,917,-361
-336,658,858
95,138,22
-476,619,847
-340,-569,-846
567,-361,727
-460,603,-452
669,-402,600
729,430,532
-500,-761,534
-322,571,750
-466,-666,-811
-429,-592,574
-355,545,-477
703,-491,-529
-328,-685,520
413,935,-424
-391,539,-444
586,-435,557
-364,-763,-893
807,-499,-711
755,-354,-619
553,889,-390

--- scanner 2 ---
649,640,665
682,-795,504
-784,533,-524
-644,584,-595
-588,-843,648
-30,6,44
-674,560,763
500,723,-460
609,671,-379
-555,-800,653
-675,-892,-343
697,-426,-610
578,704,681
493,664,-388
-671,-858,530
-667,343,800
571,-461,-707
-138,-166,112
-889,563,-600
646,-828,498
640,759,510
-630,509,768
-681,-892,-333
673,-379,-804
-742,-814,-386
577,-820,562

--- scanner 3 ---
-589,542,597
605,-692,669
-500,565,-823
-660,373,557
-458,-679,-417
-488,449,543
-626,468,-788
338,-750,-386
528,-832,-391
562,-778,733
-938,-730,414
543,643,-506
-524,371,-870
407,773,750
-104,29,83
378,-903,-323
-778,-728,485
426,699,580
-438,-605,-362
-469,-447,-387
509,732,623
647,635,-688
-868,-804,481
614,-800,639
595,780,-596

--- scanner 4 ---
727,592,562
-293,-554,779
441,611,-461
-714,465,-776
-743,427,-804
-660,-479,-426
832,-632,460
927,-485,-438
408,393,-506
466,436,-512
110,16,151
-258,-428,682
-393,719,612
-211,-452,876
808,-476,-593
-575,615,604
-485,667,467
-680,325,-822
-627,-443,-432
872,-547,-609
833,512,582
807,604,487
839,-516,451
891,-625,532
-652,-548,-490
30,-46,-14";

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE), 79)
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE), 3621)
    }
}