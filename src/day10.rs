use z3::ast::Int;
use z3::Optimize;
use std::collections::{HashSet};
use std::mem::swap;

pub fn part1(machines: &[Machine]) -> u32 {
    machines.iter().map(|m| m.steps_to_target())
        .sum()
}

pub fn part2(machines: &[Machine]) -> u32 {
    machines.iter().map(|m| {
        m.steps_to_joltage() as u32
    })
        .sum()
}

pub fn generator(input: &str) -> Vec<Machine> {
    input.lines()
        .map(|line| { Machine::new(line) })
        .collect()
}

#[derive(Debug)]
pub struct Machine {
    target: u32,
    start: u32,
    buttons: Vec<Vec<bool>>,
    counters: Vec<Counter>,
}

impl Machine {
    fn new(input: &str) -> Self {
        let mut splits = input.split(' ');
        let mut bits = 0;
        let target = splits.next().unwrap()
            .chars()
            .filter(|c| c != &'[' && c != &']')
            .fold(0, |acc, c| {
                bits += 1;
                (acc << 1) + if c == '.' { 0 } else { 1 }
            });
        let counters = splits.next_back().unwrap();
        let counters: Vec<u32> = counters.strip_prefix('{').unwrap()
            .strip_suffix('}').unwrap()
            .split(',')
            .map(|s| s.parse::<u32>().unwrap())
            .collect();

        let buttons: Vec<Vec<bool>> = splits.map(|b| {
            let mut joltages = vec![false; counters.len()];
            b.strip_prefix('(').unwrap()
                .strip_suffix(')').unwrap()
                .split(',')
                .for_each(|s| joltages[s.parse::<usize>().unwrap()] = true);
            joltages
        }).collect();
        let counters: Vec<Counter> = counters.into_iter().enumerate()
            .map(|(c_idx, target)| {
                Counter {
                    target,
                    buttons: buttons.iter()
                        .map(|counters| counters[c_idx]).collect(),
                }
            }).collect();

        Machine { target, start: 0, counters, buttons }
    }

    fn steps_to_target(&self) -> u32 {
        let mut seen = HashSet::new();
        seen.insert(self.start);
        let buttons = self.buttons.iter().map(|b| {
            b.iter().enumerate()
                .filter(|(_, v)| **v)
                .map(|(idx, _)| 1 << (self.counters.len() - 1 - idx))
                .fold(0u32, |acc, v| acc ^ v)
        }).collect::<Vec<_>>();
        let mut values = vec![self.start];
        let mut next: Vec<u32> = Vec::new();
        let mut steps = 0;
        'main: loop {
            next.clear();
            steps += 1;
            for c in values.iter() {
                for b in buttons.iter() {
                    let next_value = c ^ *b;
                    match c ^ *b {
                        v if v == self.target => break 'main,
                        v if seen.contains(&v) => continue,
                        _ => {
                            next.push(next_value);
                            seen.insert(next_value);
                        }
                    }
                }
            }
            swap(&mut values, &mut next);
        }
        steps
    }

    fn steps_to_joltage(&self) -> u64 {
        let opt = Optimize::new();
        let buttons = self.buttons.iter().enumerate().map(|(idx, _)| {
            Int::new_const(format!("b{idx}").to_string())
        }).collect::<Vec<_>>();
        buttons.iter().for_each(|b| opt.assert(b.ge(0)));

        for c in self.counters.iter() {
            let _value = Int::from_u64(c.target as u64);
            let _sum: Vec<&Int> = c.buttons.iter().enumerate()
                .filter(|(_, v)| **v)
                .map(|(idx, _)| {
                    &buttons[idx]
                })
                .collect();
            opt.assert(Int::add(&_sum).eq(c.target));
        }
        let _button_sum: Vec<&Int> = buttons.iter().collect();
        opt.minimize(&Int::add(&_button_sum));

        match opt.check(&[]) {
            z3::SatResult::Sat => {
                let model = opt.get_model().unwrap();
                buttons.iter().map(|b| {
                    let times = match model.eval(b, false) {
                        Some(val) => val.as_u64().unwrap_or(0),
                        None => 0,
                    };
                    times
                })
                    .sum()
            }
            _ => {
                unreachable!();
            }
        }
    }
}

#[derive(Debug)]
struct Counter {
    target: u32,
    buttons: Vec<bool>,
}

#[cfg(test)]
mod tests {
    use z3::ast::Int;
    use z3::{Optimize};
    use super::{generator, part1, part2, Machine};

    const INPUT: &str = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";

    #[test]
    fn test_generator() {
        let machines = generator(INPUT);
        assert_eq!(3, machines.len());

        assert_eq!(6, machines[0].target);
        assert_eq!(0, machines[0].start);
        assert_eq!(machines[0].buttons[0], vec![false, false, false, true]);
        assert_eq!(machines[0].buttons[1], vec![false, true, false, true]);
        assert_eq!(machines[0].counters[0].target, 3);
        assert_eq!(machines[0].counters[0].buttons, vec![false, false, false, false, true, true]);
        assert_eq!(machines[0].counters[1].buttons, vec![false, true, false, false, false, true]);
    }

    #[test]
    fn test_part_1() {
        let machines = generator(INPUT);
        assert_eq!(2, machines[0].steps_to_target());
        assert_eq!(7, part1(&machines));
    }

    #[test]
    fn test_part_2() {
        let machines = generator(INPUT);
        assert_eq!(10, part2(&machines[0..1]));
        assert_eq!(12, part2(&machines[1..2]));
        assert_eq!(11, part2(&machines[2..3]));
        assert_eq!(33, part2(&machines));
    }

    #[test]
    fn test_sample1() {
        let m = Machine::new("[...##..##] (1,2,3,5,6) (0,1,3,4,5,7) (1,2,3,4,5,7,8) (1,2,3,6,8) (0,1,6,7,8) (0,1,2,3,4,5,7) (0,2,3,4,8) (0,1) (1,2,3,4,5,7) (0,3,4,5,6) {75,122,93,109,78,80,57,77,68}");
        assert_eq!(136, part2(&[m]));
    }

    #[test]
    fn test_sample2() {
        let m = Machine::new("[.....#..##] (0,1,2,3,4,7,8,9) (4,5,7,9) (1,2,3,5,6,7,8,9) (0,2,6) (1,4,6,9) (0,7) (1,2,5,6,8,9) (1,2,4,8) (2,3,4,7,8) (0,1,3,5,7,8) (0,1,2,3,5,7,8,9) (0,1,2,3,4,5,6,7,8) (1,5,6,7,8,9) {61,127,89,74,48,100,78,106,123,85}");
        assert_eq!(131, part2(&[m]));
    }
}
