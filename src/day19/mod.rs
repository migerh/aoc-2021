use std::cmp::max;
use std::cmp::min;
use std::collections::VecDeque;
use std::collections::HashMap;
use std::str::FromStr;
use std::num::ParseIntError;
use crate::utils::ParseError;
use itertools::Itertools;

type C = isize;
type Coords = [C; 3];

#[derive(Debug)]
pub struct Scanner {
    name: String,
    beacons: Vec<Coords>,
}

impl FromStr for Scanner {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let name = s.lines()
            .take(1)
            .next()
            .ok_or(ParseError::new("No name found"))?
            .to_owned();
        let beacons = s.lines()
            .skip(1)
            .filter(|s| *s != "")
            .map(|l| Scanner::parse_line(l))
            .collect::<Result<Vec<_>, ParseError>>()?;

        Ok(Self { name, beacons })
    }
}

impl Scanner {
    fn parse_line(s: &str) -> Result<Coords, <Self as FromStr>::Err> {
        let coords = s.split(",").map(|v| v.parse::<isize>()).collect::<Result<Vec<_>, ParseIntError>>()?;
        if coords.len() != 3 {
            return Err(ParseError::new("Invalid number of coordinates for beacon"));
        }

        Ok([coords[0], coords[1], coords[2]])
    }
}

#[aoc_generator(day19)]
pub fn input_generator(input: &str) -> Result<Vec<Scanner>, ParseError> {
    input
        .split("\n\n")
        .filter(|s| *s != "")
        .map(|s| Scanner::from_str(s))
        .collect::<Result<Vec<_>, ParseError>>()
}

fn permutations() -> Vec<[isize; 6]> {
    let mut result = vec![];

    let sign_perms = vec![
        vec![ 1,  1,  1],
        vec![-1,  1,  1],
        vec![ 1, -1,  1],
        vec![ 1,  1, -1],
        vec![-1, -1,  1],
        vec![ 1, -1, -1],
        vec![-1,  1, -1],
        vec![-1, -1, -1],
    ];

    for p in (0..=2).permutations(3) {
        for s in &sign_perms {
            result.push([p[0], p[1], p[2], s[0], s[1], s[2]]);
        }
    }

    result
}

fn transform(beacons: &Vec<Coords>, t: &[isize; 6]) -> Vec<Coords> {
    beacons.iter()
        .map(|b| {
            [t[3] * b[t[0] as usize], t[4] * b[t[1] as usize], t[5] * b[t[2] as usize]]
        })
        .collect::<Vec<_>>()
}

fn translate(beacons: &Vec<Coords>, t: Coords) -> Vec<Coords> {
    beacons.iter()
        .map(|b| {
            [b[0] + t[0], b[1] + t[1], b[2] + t[2]]
        })
        .collect::<Vec<_>>()
}

fn manhattan(a: Coords, b: Coords) -> u64 {
    let mut dist = 0;
    for i in 0..3 {
        dist += (b[i]-a[i]).abs() as u64
    }
    dist
}

fn dist(a: Coords, b: Coords) -> Coords {
    [(b[0] - a[0]).abs(), (b[1] - a[1]).abs(), (b[2] - a[2]).abs()]
}

fn sub(a: Coords, b: Coords) -> Coords {
    [(a[0] - b[0]), (a[1] - b[1]), (a[2] - b[2])]
}

#[derive(Debug)]
pub struct BeaconMap {
    map: HashMap<(usize, usize), [isize; 3]>,
    beacons: Vec<Coords>,
}

impl BeaconMap {
    fn new(b: Vec<Coords>) -> Self {
        let map = HashMap::new();
        let beacons = vec![];
        let mut bmap = BeaconMap { map, beacons };
        bmap.add(b.clone());
        bmap
    }

    fn add(&mut self, mut new_beacons: Vec<Coords>) {
        let len = self.beacons.len();
        self.beacons.append(&mut new_beacons);

        for i in 0..self.beacons.len() {
            for j in len..self.beacons.len() {
                if i >= j {
                    continue;
                }
                self.map.entry((i, j)).or_insert(dist(self.beacons[i], self.beacons[j]));
            }
        }
    }

    fn has_unique_distance(&self, distance: [isize; 3]) -> bool {
        self.map.iter().filter(|(_, v)| **v == distance).count() == 1
    }
}

fn trace(map: &BeaconMap, beacons: &Vec<Coords>) -> Option<(Vec<usize>, Vec<usize>)> {

    let mut pools = vec![];
    for t in permutations() {
        let points = transform(beacons, &t);
        let tmp_map = BeaconMap::new(points.clone());
        for (i, _) in points.iter().enumerate() {
            let mut pool = tmp_map.map.iter()
                .filter(|(k, v)| k.0 == i && map.has_unique_distance(**v))
                .map(|(k, _)| k.1)
                .collect::<Vec<_>>();
            pool.push(i);

            pools.push(pool);
        }

        pools.sort_by(|a, b| b.len().cmp(&a.len()));
        // println!("Pools: {:?}", pools);

        let pool = pools[0].clone();

        if pool.len() < 12 {
            continue;
        }

        let matched = points.iter().enumerate().filter(|(i, _)| pool.contains(i)).map(|(_, v)| v).cloned().collect::<Vec<_>>();
        let tm = BeaconMap::new(matched);

        let mut matchpools = vec![];
        for (j, _) in map.beacons.iter().enumerate() {
            let mut matchpool = map.map.iter()
                .filter(|(k, v)| k.0 == j && tm.has_unique_distance(**v))
                .map(|(k, _)| k.1)
                .collect::<Vec<_>>();
            matchpool.push(j);
            matchpools.push(matchpool);
        }
        matchpools.sort_by(|a, b| b.len().cmp(&a.len()));
        let matches = matchpools[0].clone();

        if matches.len() == pool.len() {
            return Some((pool, matches))
        }
    }

    None
}

fn bounding_box(points: &Vec<Coords>) -> (Coords, Coords) {
    let mut mi = [isize::MAX, isize::MAX, isize::MAX];
    let mut ma = [isize::MIN, isize::MIN, isize::MIN];

    for p in points {
        mi = [min(mi[0], p[0]), min(mi[1], p[1]), min(mi[2], p[2])];
        ma = [max(ma[0], p[0]), max(ma[1], p[1]), max(ma[2], p[2])];
    }

    (mi, ma)
}

fn find_transformation(map: &BeaconMap, beacons: &Vec<Coords>, existing: &Vec<usize>) -> Option<([isize; 6], Coords)> {
    let trafos = permutations();

    let original = map.beacons.iter().enumerate().filter(|(i, _)| existing.contains(i)).map(|(_, v)| v).cloned().collect::<Vec<_>>();
    let fixpoint = original[0];
    let box2 = bounding_box(&original);

    for t in trafos {
        let points = transform(beacons, &t);
        let box1 = bounding_box(&points);

        let size1 = [box1.1[0] - box1.0[0], box1.1[1] - box1.0[1], box1.1[2] - box1.0[2]];
        let size2 = [box2.1[0] - box2.0[0], box2.1[1] - box2.0[1], box2.1[2] - box2.0[2]];

        if size1[0] == size2[0] && size1[1] == size2[1] && size1[2] == size2[2] {
            for p in &points {
                let translation = sub(fixpoint, *p);
                let translated = translate(&points, translation);

                let matches = translated.iter()
                    .filter(|p| original.contains(p))
                    .count() == original.len();

                if matches {
                    return Some((t, translation));
                }
            }
        }
    }

    None
}

#[aoc(day19, part1)]
pub fn solve_part1(input: &Vec<Scanner>) -> Result<usize, ParseError> {
    let mut queue = input.iter().skip(1).collect::<VecDeque<_>>();
    let mut map = BeaconMap::new(input[0].beacons.clone());

    let mut iterations = 0;
    while let Some(q) = queue.pop_front() {
        if let Some(t) = trace(&map, &q.beacons) {
            println!("matches found #{}: {}", iterations, t.0.len());
            if t.0.len() >= 12 {
                println!("Successful trace, adding untraced points - {} vs {}", t.0.len(), t.1.len());
                let matched = q.beacons.iter().enumerate().filter(|(i, _)| t.0.contains(i)).map(|(_, v)| v).cloned().collect::<Vec<_>>();
                if let Some(trans) = find_transformation(&map, &matched, &t.1) {
                    println!("trans: {:?}", trans);

                    let (rotation, translation) = trans;
                    let unmatched = q.beacons.iter().enumerate().filter(|(i, _)| !t.0.contains(i)).map(|(_, v)| v).cloned().collect::<Vec<_>>();
                    let rotated = transform(&unmatched, &rotation);
                    let transformed = translate(&rotated, translation);

                    map.add(transformed);

                    continue;
                }
            }
        }

        queue.push_back(q);

        iterations += 1;
        if iterations > 1000 {
            break;
        }
    }

    if queue.len() > 0 {
        println!("Need another idea :(");
    }

    Ok(map.beacons.len())
}

#[aoc(day19, part2)]
pub fn solve_part2(input: &Vec<Scanner>) -> Result<u64, ParseError> {
    let mut queue = input.iter().skip(1).collect::<VecDeque<_>>();
    let mut map = BeaconMap::new(input[0].beacons.clone());

    let mut translations = vec![];
    let mut iterations = 0;
    while let Some(q) = queue.pop_front() {
        if let Some(t) = trace(&map, &q.beacons) {
            println!("matches found #{}: {}", iterations, t.0.len());
            if t.0.len() >= 12 {
                println!("Successful trace, adding untraced points - {} vs {}", t.0.len(), t.1.len());
                let matched = q.beacons.iter().enumerate().filter(|(i, _)| t.0.contains(i)).map(|(_, v)| v).cloned().collect::<Vec<_>>();
                if let Some(trans) = find_transformation(&map, &matched, &t.1) {
                    println!("trans: {:?}", trans);

                    let (rotation, translation) = trans;
                    let unmatched = q.beacons.iter().enumerate().filter(|(i, _)| !t.0.contains(i)).map(|(_, v)| v).cloned().collect::<Vec<_>>();
                    let rotated = transform(&unmatched, &rotation);
                    let transformed = translate(&rotated, translation);

                    map.add(transformed);

                    translations.push(translation);

                    continue;
                }
            }
        }

        queue.push_back(q);

        iterations += 1;
        if iterations > 1000 {
            break;
        }
    }

    if queue.len() > 0 {
        println!("Need another idea :(");
    }

    let mut distances = vec![];
    for i in 0..translations.len() {
        for j in 0..translations.len() {
            distances.push(manhattan(translations[i], translations[j]));
        }
    }
    distances.sort();

    Ok(*distances.iter().rev().next().unwrap())
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils::ParseError;

    fn sample() -> &'static str {
        "--- scanner 0 ---
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
30,-46,-14"
    }

    fn input() -> Result<Vec<Scanner>, ParseError> {
        Ok(input_generator(sample())?)
    }

    #[test]
    fn part1_sample() -> Result<(), ParseError> {
        let data = input()?;
        Ok(assert_eq!(79, solve_part1(&data)?))
    }

    #[test]
    fn part2_sample() -> Result<(), ParseError> {
        let data = input()?;
        Ok(assert_eq!(3621, solve_part2(&data)?))
    }
}
