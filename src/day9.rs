use std::collections::{HashMap};
use itertools::Itertools;

pub fn part1(floor: &Floor) -> u64 {
    floor.find_largest_area()
}

pub fn part2(floor: &Floor) -> u64 {
    floor.red_tiles.iter().combinations(2)
        .filter(|p| p[0].x != p[1].x || p[0].y != p[1].y)
        .map(|p| floor.compute_enclosed_area(p[0], p[1]))
        .max().unwrap()
}

pub fn generator(input: &str) -> Floor {
    Floor::new(input)
}

pub struct Floor {
    red_tiles: Vec<Coord>,
    x_values: Vec<u64>,
    y_values: Vec<u64>,
    x_lines: HashMap<(u64, u64), Vec<u64>>,
    y_lines: HashMap<(u64, u64), Vec<u64>>,
}

impl Floor {
    fn new(input: &str) -> Floor {
        let red_tiles: Vec<Coord> = input.lines()
            .map(|line| {
                let (x, y) = line.split_once(',').unwrap();
                Coord::new(x.parse().unwrap(), y.parse().unwrap())
            }).collect();
        let x_values: Vec<u64> = red_tiles.iter().map(|coord| coord.x)
            .sorted_unstable()
            .dedup().collect();
        let y_values: Vec<u64> = red_tiles.iter().map(|coord| coord.y)
            .sorted_unstable()
            .dedup().collect();

        let (x_lines, y_lines) = Floor::make_lines(&red_tiles, &x_values, &y_values);
        Floor { red_tiles, x_values, y_values, x_lines, y_lines }
    }

    fn is_xline_enclosing(&self, x_pos: &(usize, usize), y: &(u64, u64)) -> bool {
        Floor::is_line_enclosing(x_pos, y, &self.y_lines, &self.x_values)
    }

    fn is_yline_enclosing(&self, y_pos: &(usize, usize), x: &(u64, u64)) -> bool {
        Floor::is_line_enclosing(y_pos, x, &self.x_lines, &self.y_values)
    }

    fn is_line_enclosing(pos: &(usize, usize), value: &(u64, u64), lines: &HashMap<(u64, u64), Vec<u64>>, values: &[u64]) -> bool {
        for v in values[pos.0..=pos.1].windows(2) {
            let search_space = lines.get(&(v[0], v[1])).unwrap();
            let min_pos = match search_space.binary_search(&value.0) {
                Ok(idx) if idx % 2 == 1 => return false,
                Ok(idx) => idx,
                Err(idx) if idx == 0 || idx >= search_space.len() || idx % 2 == 0 => return false,
                Err(idx) => idx - 1,
            };

            let max_pos = min_pos + 1;
            if max_pos >= search_space.len() || search_space[max_pos] < value.1 {
                return false;
            }
        }
        true
    }

    fn find_largest_area(&self) -> u64 {
        self.red_tiles.iter().combinations(2)
            .map(|combination| combination[0].area(combination[1]))
            .max().unwrap()
    }

    fn make_lines(red_tiles: &[Coord], x_values: &[u64], y_values: &[u64]) -> (HashMap<(u64, u64), Vec<u64>>, HashMap<(u64, u64), Vec<u64>>) {
        let mut x_lines: HashMap<(u64, u64), Vec<u64>> = HashMap::new();
        let mut y_lines: HashMap<(u64, u64), Vec<u64>> = HashMap::new();

        red_tiles.iter().circular_tuple_windows()
            .filter(|(a, b)| a.x == b.x)
            .for_each(|(a, b)| {
                Self::compute_lines(&a.y, &b.y, &a.x, y_values, &mut x_lines);
            });
        red_tiles.iter().circular_tuple_windows()
            .filter(|(a, b)| a.y == b.y)
            .for_each(|(a, b)| {
                Self::compute_lines(&a.x, &b.x, &a.y, x_values, &mut y_lines);
            });

        x_lines.iter_mut().for_each(|(_, values)| { values.sort_unstable() });
        y_lines.iter_mut().for_each(|(_, values)| { values.sort_unstable() });
        (x_lines, y_lines)
    }

    fn compute_lines(v1: &u64, v2: &u64, o: &u64, values: &[u64], lines: &mut HashMap<(u64, u64), Vec<u64>>) {
        let r = v1.min(v2)..=v1.max(v2);

        values.iter()
            .tuple_windows()
            .filter(|(x1, x2)| r.contains(x1) && r.contains(x2))
            .for_each(|(x1, x2)| {
                lines.entry((*x1, *x2))
                    .and_modify(|y| y.push(*o))
                    .or_insert(vec![*o]);
            });
    }

    fn compute_enclosed_area(&self, p1: &Coord, p2: &Coord) -> u64 {
        let x = (p1.x.min(p2.x), p1.x.max(p2.x));
        let y = (p1.y.min(p2.y), p1.y.max(p2.y));
        let x_pos = (self.x_values.binary_search(&x.0).unwrap(),
                     self.x_values.binary_search(&x.1).unwrap());
        let y_pos = (self.y_values.binary_search(&y.0).unwrap(),
                     self.y_values.binary_search(&y.1).unwrap());
        if self.is_xline_enclosing(&x_pos, &y)
            && self.is_yline_enclosing(&y_pos, &x) {
            (x.1 - x.0 + 1) * (y.1 - y.0 + 1)
        } else {
            0
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
#[derive(Hash)]
#[derive(Clone)]
struct Coord {
    x: u64,
    y: u64,
}

impl Coord {
    fn new(x: u64, y: u64) -> Coord {
        Coord { x, y }
    }

    fn area(&self, other: &Coord) -> u64 {
        (self.x.abs_diff(other.x) + 1) * (self.y.abs_diff(other.y) + 1)
    }
}

#[cfg(test)]
mod tests {
    use super::{generator, part1, part2, Coord};

    const INPUT: &str = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";

    #[test]
    fn test_generator() {
        let f = generator(INPUT);
        assert_eq!(f.red_tiles.len(), 8);
    }

    #[test]
    fn test_part_1() {
        let f = generator(INPUT);
        assert_eq!(part1(&f), 50);
    }

    #[test]
    fn test_make_lines() {
        let f = generator(INPUT);
        assert_eq!(f.x_lines.values().map(|v| v.len()).sum::<usize>(), 6);
        assert_eq!(f.y_lines.values().map(|v| v.len()).sum::<usize>(), 6);
    }

    #[test]
    fn test_enclosed_area() {
        // (7,1) -> (9,5)
        let f = generator(INPUT);
        let c1 = Coord { x: 7, y: 1 };
        let c2 = Coord { x: 9, y: 5 };
        assert_eq!(f.compute_enclosed_area(&c1, &c2), 15);
    }

    #[test]
    fn test_part_2() {
        let f = generator(INPUT);
        assert_eq!(part2(&f), 24);
    }
}