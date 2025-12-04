use crate::day4::Space::Paper;
use crate::day4::Space::Empty;

pub fn part1(diagram: &Diagram) -> u32 {
    diagram.accessible_rolls.len() as u32
}

pub fn part2(diagram: &Diagram) -> u32 {
    let mut diagram = diagram.clone();
    diagram.remove_rolls(true)
}

pub fn generator(input: &str) -> Diagram {
    Diagram::new(input)
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Space {
    Paper(u8),
    Empty,
}

impl Space {
    fn increase_adjacent_rolls(&mut self) {
        if let Paper(n) = self {
            *n += 1;
        }
    }


    fn decrease_adjacent_rolls(&mut self) -> Option<u8> {
        if let Paper(n) = self {
            *n -= 1;
            let r = *n;
            if *n == 0 {
                *self = Empty;
            }
            Some(r)
        } else {
            None
        }
    }
}

#[derive(Clone)]
pub struct Diagram {
    layout: Vec<Vec<Space>>,
    accessible_rolls: Vec<(usize, usize)>,
}

impl Diagram {
    fn new(input: &str) -> Self {
        let layout = input.lines()
            .map(|line| line.chars().map(|c| match c {
                '@' => Paper(0),
                '.' => Empty,
                _ => panic!("Invalid character {c} in input"),
            }).collect())
            .collect();

        let mut d = Diagram {
            layout,
            accessible_rolls: Vec::new(),
        };
        d.compute_adjacent_rolls();
        d.compute_accessible_rolls();
        d
    }

    fn width(&self) -> usize {
        self.layout.first().unwrap().len()
    }

    fn height(&self) -> usize {
        self.layout.len()
    }

    fn compute_adjacent_rolls(&mut self) {
        let width = self.width();
        let height = self.height();
        for y in 0..height {
            for x in 0..width {
                if self.layout[y][x] == Empty { continue; }
                for nx in [-1i32, 0, 1] {
                    for ny in [-1i32, 0, 1] {
                        if nx == 0 && ny == 0 {
                            continue;
                        }
                        match (x as i32 + nx, y as i32 + ny) {
                            (x, y) if x < 0 || x >= width as i32 || y < 0 || y >= height as i32 => {}
                            (x, y) => {
                                self.layout[y as usize][x as usize].increase_adjacent_rolls();
                            }
                        }
                    }
                }
            }
        }
    }

    fn compute_accessible_rolls(&mut self) {
        self.layout.iter_mut().enumerate().for_each(|(y, row)| {
            row.iter_mut().enumerate().for_each(|(x, s)| {
                match s {
                    Paper(n) if n < &mut 4 => {
                        *s = Empty;
                        self.accessible_rolls.push((x, y));
                    }
                    _ => {}
                }
            })
        });
    }

    fn remove_rolls(&mut self, with_update: bool) -> u32 {
        let mut removed = 0;
        let width = self.width();
        let height = self.height();
        while !self.accessible_rolls.is_empty() {
            removed += 1;
            let (x, y) = self.accessible_rolls.pop().unwrap();
            for nx in [-1i32, 0, 1] {
                for ny in [-1i32, 0, 1] {
                    if nx == 0 && ny == 0 {
                        continue;
                    }
                    match (x as i32 + nx, y as i32 + ny) {
                        (x, y) if x < 0 || x >= width as i32 || y < 0 || y >= height as i32 => {}
                        (x, y) => {
                            if with_update
                                && self.layout[y as usize][x as usize].decrease_adjacent_rolls()
                                .is_some_and(|v| v < 4) {
                                self.layout[y as usize][x as usize] = Empty;
                                self.accessible_rolls.push((x as usize, y as usize));
                            }
                        }
                    }
                }
            }
        }
        removed
    }
}

#[cfg(test)]
mod tests {
    use super::{generator, part1, part2};

    const INPUT: &str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

    #[test]
    fn test_generator() {
        let diagram = generator(INPUT);
        assert_eq!(diagram.width(), 10);
        assert_eq!(diagram.height(), 10);
    }

    #[test]
    fn test_part_1() {
        let diagram = generator(INPUT);
        assert_eq!(part1(&diagram), 13);
    }

    #[test]
    fn test_part_2() {
        let diagram = generator(INPUT);
        assert_eq!(part2(&diagram), 43);
    }
}
