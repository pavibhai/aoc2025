use std::mem::swap;

pub fn part1(manifold: &Lab) -> u64 {
    manifold.active_splitters
}

pub fn part2(manifold: &Lab) -> u64 {
    manifold.timelines
}

pub fn generator(input: &str) -> Lab {
    Lab::new(input)
}

#[derive(Debug, Eq, PartialEq)]
enum Space {
    Empty,
    Splitter,
}

pub struct Lab {
    start: (u64, u64),
    manifold: Vec<Vec<Space>>,
    timelines: u64,
    active_splitters: u64,
}

impl Lab {
    fn width(&self) -> usize {
        self.manifold[0].len()
    }
    fn new(input: &str) -> Self {
        let mut start: Option<(u64, u64)> = None;
        let manifold = input.trim().lines().enumerate()
            .map(|(y, line)| line.chars().enumerate()
                .map(|(x, char)| match char {
                    '.' => Space::Empty,
                    '^' => Space::Splitter,
                    'S' => {
                        start = Some((y as u64, x as u64));
                        Space::Empty
                    }
                    _ => {
                        unreachable!()
                    }
                }).collect()
            ).collect();
        let mut l = Lab { start: start.unwrap(), manifold, timelines: 0, active_splitters: 0 };
        l.identify_timelines();
        l
    }

    fn identify_timelines(&mut self) {
        let mut current = vec![0u64; self.width()];
        let mut next = vec![0u64; self.width()];
        current[self.start.1 as usize] += 1;
        let width = self.width();
        let mut active = Vec::new();

        for y in (self.start.0 + 1)..self.manifold.len() as u64 {
            for (x, v) in current.iter().enumerate().filter(|&(_, v)| v > &0) {
                if self.manifold[y as usize][x] == Space::Splitter {
                    active.push((y, x));
                    if x > 0 {
                        next[x - 1] += v;
                    }
                    if x + 1 < width {
                        next[x + 1] += v;
                    }
                } else {
                    next[x] += v;
                }
            }
            swap(&mut current, &mut next);
            next.iter_mut().for_each(|x| *x = 0);
        }
        active.sort_unstable();
        active.dedup();
        self.active_splitters = active.len() as u64;
        self.timelines = current.iter().sum();
    }
}

#[cfg(test)]
mod tests {
    use super::{generator, part1, part2};

    const INPUT: &str = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

    #[test]
    fn test_generator() {
        let lab = generator(INPUT);
        assert_eq!(lab.start, (0, 7));
        assert_eq!(lab.manifold.len(), 16);
        assert_eq!(lab.manifold.first().unwrap().len(), 15);
    }

    #[test]
    fn test_part_1() {
        let lab = generator(INPUT);
        assert_eq!(part1(&lab), 21);
    }

    #[test]
    fn test_part_2() {
        let lab = generator(INPUT);
        assert_eq!(part2(&lab), 40);
    }
}
