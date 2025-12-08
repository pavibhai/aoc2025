use itertools::Itertools;

pub fn part1(playground: &Playground) -> u64 {
    playground.connect_boxes(Some(1000))
}

pub fn part2(playground: &Playground) -> u64 {
    playground.connect_boxes(None)
}

pub fn generator(input: &str) -> Playground {
    Playground::new(input)
}

#[derive(Debug, Eq, PartialEq)]
struct Coord {
    x: u64,
    y: u64,
    z: u64,
}

impl Coord {
    fn distance_between(&self, other: &Coord) -> u64 {
        (self.x.abs_diff(other.x)).pow(2) + (self.y.abs_diff(other.y)).pow(2) + (self.z.abs_diff(other.z).pow(2))
    }
}

pub struct Playground {
    junction_boxes: Vec<Coord>,
    distances: Vec<(usize, usize, u64)>,
}

impl Playground {
    fn new(input: &str) -> Playground {
        let junction_boxes = input.lines()
            .map(|line| {
                let mut values = line.split(',').map(|i| i.parse().unwrap());
                let c = Coord { x: values.next().unwrap(), y: values.next().unwrap(), z: values.next().unwrap() };
                match values.next() {
                    Some(_) => unreachable!("Unexpected line with {line}"),
                    None => c
                }
            }).collect();

        let mut p = Playground { junction_boxes, distances: vec![] };
        p.compute_distances();
        p
    }

    fn compute_distances(&mut self) {
        self.distances = self.junction_boxes.iter().enumerate().combinations(2)
            .map(|combination| {
                let (idx1, c1) = combination[0];
                let (idx2, c2) = combination[1];
                let d = c1.distance_between(&c2);
                (idx1, idx2, d)
            }).collect();
        self.distances.sort_unstable_by(|item1, item2| item2.2.cmp(&item1.2));
    }

    fn connect_boxes(&self, times: Option<usize>) -> u64 {
        let mut next_id = 0u32;
        let mut connections: Vec<Option<u32>> = vec![None; self.junction_boxes.len()];
        let mut distances = self.distances.clone();
        let mut idx = 0usize;
        let mut last_connection = None;

        while times.is_none() || times.is_some_and(|v| idx < v) {
            idx += 1;
            let (idx_1, idx_2, _) = distances.pop().unwrap();
            match (connections[idx_1], connections[idx_2]) {
                (Some(c1), Some(c2)) if c1 == c2 => {
                    continue;
                }
                (Some(c1), Some(c2)) => {
                    connections.iter_mut()
                        .filter(|item| item.is_some_and(|v| v == c2))
                        .for_each(|item| *item = Some(c1));
                }
                (Some(c1), None) => {
                    connections[idx_2] = Some(c1);
                }
                (None, Some(c2)) => {
                    connections[idx_1] = Some(c2);
                }
                (None, None) => {
                    connections[idx_1] = Some(next_id);
                    connections[idx_2] = Some(next_id);
                    next_id += 1;
                }
            }
            if connections.iter().all_equal() {
                last_connection = Some((idx_1, idx_2));
                break;
            }
        }
        if times.is_some() {
            let mut circuit_lengths = vec![0; next_id as usize];
            connections.iter().filter(|item| item.is_some())
                .for_each(|item| {circuit_lengths[item.unwrap() as usize] += 1;});
            circuit_lengths.sort_unstable();
            circuit_lengths.iter().rev().take(3)
                .product::<u64>()
        } else {
            let (idx1, idx2) = last_connection.unwrap();
            self.junction_boxes[idx1].x * self.junction_boxes[idx2].x
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{generator, part2, Coord};

    const INPUT: &str = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";

    #[test]
    fn test_generator() {
        let p = generator(INPUT);
        assert_eq!(p.junction_boxes.len(), 20);
        assert_eq!(p.junction_boxes[0], Coord { x: 162, y: 817, z: 812 });
        assert_eq!(p.junction_boxes.last().unwrap(), &Coord { x: 425, y: 690, z: 689 });
    }

    #[test]
    fn test_part_1() {
        let p = generator(INPUT);
        assert_eq!(p.connect_boxes(Some(10)), 40);
    }

    #[test]
    fn test_part_2() {
        let p = generator(INPUT);
        assert_eq!(part2(&p), 25272);
    }
}
