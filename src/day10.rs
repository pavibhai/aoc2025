use std::time::SystemTime;
use std::cmp::Ordering;
use std::collections::{BinaryHeap};
use std::mem::swap;
use itertools::Itertools;

pub fn part1(machines: &[Machine]) -> u32 {
    machines.iter().map(|m| m.steps_to_target())
        .sum()
}

pub fn part2(machines: &[Machine]) -> u32 {
    machines.iter().enumerate().map(|(idx, m)| {
        println!("{:?}: {}", SystemTime::now(), idx);
        m.steps_to_joltage()
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
    buttons: Vec<Vec<usize>>,
    joltage: Vec<u32>,
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
        let joltage = splits.next_back().unwrap();
        let joltage = joltage.strip_prefix('{').unwrap()
            .strip_suffix('}').unwrap()
            .split(',')
            .map(|s| s.parse::<u32>().unwrap())
            .collect();

        let buttons = splits.map(|b| {
            b.strip_prefix('(').unwrap()
                .strip_suffix(')').unwrap()
                .split(',')
                .map(|s| s.parse::<usize>().unwrap())
                .sorted()
                .collect()
        }).collect();

        Machine { target, start: 0, joltage, buttons }
    }

    fn steps_to_target(&self) -> u32 {
        let buttons = self.buttons.iter().map(|b| {
            b.iter().map(|b| 1 << (self.joltage.len() - 1 - b))
                .fold(0u32, |acc, v| acc ^ v)
        }).collect::<Vec<_>>();
        let mut values = vec![self.start];
        let mut next: Vec<u32> = Vec::new();
        let mut steps = 0;
        while !values.contains(&self.target) {
            next.clear();
            steps += 1;
            for c in values.iter() {
                for b in buttons.iter() {
                    next.push(c ^ *b);
                }
            }
            swap(&mut values, &mut next);
        }
        steps
    }

    fn steps_to_joltage(&self) -> u32 {
        let mut e = Evaluator::new(self);
        e.solve()
    }
}

#[derive(Debug, Clone)]
struct Joltage {
    target: u32,
    buttons: Vec<usize>,
}

#[derive(Debug, Clone)]
struct State {
    joltage: Vec<u32>,
    steps: u32,
}

impl State {
    fn speed(&self) -> u32 {
        self.joltage.iter().sum::<u32>() * 1000000 / self.steps
    }
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other).is_eq()
    }
}

impl Eq for State {}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.steps.cmp(&self.steps).then(self.speed().cmp(&other.speed()))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone)]
struct Evaluator {
    buttons: Vec<Vec<usize>>,
    joltage: Vec<Joltage>,
    skip: Vec<bool>,
    skip_next: Vec<bool>,
    common_buttons: Vec<Vec<Option<Vec<usize>>>>,
}

impl Evaluator {
    fn new(machine: &Machine) -> Self {
        let mut joltage = vec![Vec::new(); machine.joltage.len()];
        machine.buttons.iter().enumerate()
            .for_each(|(b_idx, b)| {
                b.iter().for_each(|j_idx| {
                    joltage[*j_idx].push(b_idx)
                });
            });
        let joltage: Vec<Joltage> = joltage.into_iter().enumerate()
            .map(|(j_idx, mut buttons)| {
                buttons.sort();
                Joltage { target: machine.joltage[j_idx], buttons }
            })
            .collect();
        let common_buttons = Self::find_common_buttons(&joltage);

        Evaluator {
            joltage,
            buttons: machine.buttons.clone(),
            skip: vec![false; machine.buttons.len()],
            skip_next: vec![false; machine.buttons.len()],
            common_buttons,
        }
    }

    fn find_common_buttons(joltage: &[Joltage]) -> Vec<Vec<Option<Vec<usize>>>> {
        let mut common_buttons = vec![vec![None; joltage.len()]; joltage.len()];
        for i in 0..joltage.len() {
            for j in 0..joltage.len() {
                if i == j {
                    continue;
                }
                common_buttons[i][j] = Some(joltage[i].buttons
                    .iter()
                    .filter(|b_idx| joltage[j].buttons.contains(b_idx))
                    .cloned()
                    .collect_vec());
            }
        }
        common_buttons
    }

    fn no_of_combinations(buttons: usize, value: u32) -> u64 {
        //println!("buttons {}, value {}", buttons, value);
        let mut result = 1;
        for d in 0..value {
            result *= (buttons as u64 - 1 + (value - d) as u64) / (value - d) as u64;
        }
        result
    }

    fn press(&self, b_idx: &usize, joltage: &mut [u32], times: u32) {
        self.buttons[*b_idx].iter().for_each(|j_idx| { joltage[*j_idx] += times })
    }

    fn un_press(&self, b_idx: &usize, joltage: &mut [u32], times: u32) {
        self.buttons[*b_idx].iter().for_each(|j_idx| { joltage[*j_idx] -= times })
    }

    fn initialize_skips(&mut self, state: &State) {
        self.skip.iter_mut().for_each(|skip| *skip = false);
        self.skip_next.iter_mut().for_each(|skip| *skip = false);
        self.joltage.iter()
            .zip(state.joltage.iter())
            .filter(|(t, c)| &&t.target == c)
            .for_each(|(t, _)| {
                t.buttons.iter().for_each(|b_idx| {
                    self.skip[*b_idx] = true;
                    self.skip_next[*b_idx] = true;
                })
            });
    }

    fn find_next_joltage(&self, state: &State) -> Option<usize> {
        state.joltage.iter()
            .zip(self.joltage.iter())
            .enumerate()
            .filter(|&(_, (c, t))| c < &t.target)
            .map(|(j_idx, (c, t))| {
                (j_idx, (c, t), t.buttons.iter().filter(|b_idx| !self.skip[**b_idx]).count())
            })
            .filter(|(_, _, bsize)| bsize > &0)
            .min_by_key(|(_, (c, t), bsize)| Self::no_of_combinations(*bsize, t.target - **c))
            .map(|(j_idx, _, _)| j_idx)
    }

    fn solve(&mut self) -> u32 {
        let mut to_process = BinaryHeap::new();
        to_process.push(State { steps: 0, joltage: vec![0u32; self.joltage.len()] });
        let target: Vec<u32> = self.joltage.iter().map(|j| j.target).collect();

        while let Some(mut state) = to_process.pop() {
            //writeln!(&mut output, "Processing {:?}/{}", state.joltage, state.steps).unwrap();
            if state.joltage == target {
                return state.steps;
            }

            self.initialize_skips(&state);

            let j_idx = self.find_next_joltage(&state);
            if j_idx.is_none() { continue; }
            let j_idx = j_idx.unwrap();
            let current_value = &state.joltage[j_idx];
            let joltage = &self.joltage[j_idx];

            for b_idx in joltage.buttons.iter() {
                self.skip_next[*b_idx] = true;
            }

            for new_presses in joltage.buttons.iter()
                .filter(|b_idx| !self.skip[**b_idx])
                .combinations_with_replacement((joltage.target - current_value) as usize)
                .map(|c| c.into_iter().dedup_with_count().collect::<Vec<(usize, &usize)>>()) {
                new_presses.iter().for_each(|(presses, b_idx)| {
                    self.press(b_idx, &mut state.joltage, *presses as u32);
                    state.steps += *presses as u32;
                });

                if self.is_valid(&state.joltage) {
                    to_process.push(state.clone());
                }

                new_presses.iter().for_each(|(presses, b_idx)| {
                    self.un_press(b_idx, &mut state.joltage, *presses as u32);
                    state.steps -= *presses as u32;
                });
            }
        }
        unreachable!()
    }

    fn is_valid(&self, joltage: &[u32]) -> bool {
        let mut button_max = vec![(0usize, u32::MAX); self.buttons.len()];
        for ((j_idx, c), t) in joltage.iter().enumerate().zip(self.joltage.iter()) {
            if c > &t.target {
                return false;
            }
            for b_idx in t.buttons.iter() {
                match button_max.get_mut(*b_idx) {
                    Some((min_j_idx, v)) if *v > t.target - c => {
                        *min_j_idx = j_idx;
                        *v = t.target - c;
                    }
                    _ => {}
                }
            }
        }
        'joltage: for ((j_idx, c), t) in joltage.iter().enumerate().zip(self.joltage.iter()) {
            let mut remaining = t.buttons.clone();
            remaining.retain(|b_idx| button_max[*b_idx].1 > 0);
            let mut max_value = 0u32;
            while !remaining.is_empty() {
                let (s_j_idx, count, _) = (0..joltage.len())
                    .filter(|s_j_idx| s_j_idx != &j_idx)
                    .map(|s_j_idx| {
                        (s_j_idx,
                         self.common_buttons[j_idx][s_j_idx].as_ref().unwrap()
                             .iter()
                             .filter(|b_idx| remaining.binary_search(b_idx).is_ok() && button_max[**b_idx].1 > 0)
                             .count(),
                         1000000 - self.joltage[s_j_idx].target - joltage[s_j_idx]
                        )
                    })
                    .max_by_key(|(_, count, value)| (*count, *value)).unwrap();
                if count == 0 {
                    continue 'joltage;
                }
                max_value += self.joltage[s_j_idx].target - joltage[s_j_idx];
                remaining.retain(|b_idx| self.joltage[s_j_idx].buttons.binary_search(b_idx).is_err());
            }

            if t.target - c > max_value {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::{generator, part1, part2, Evaluator, Machine};

    const INPUT: &str = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";

    #[test]
    fn test_generator() {
        let machines = generator(INPUT);
        assert_eq!(3, machines.len());

        assert_eq!(6, machines[0].target);
        assert_eq!(0, machines[0].start);
        assert_eq!(vec![3, 5, 4, 7], machines[0].joltage);
    }

    #[test]
    fn test_part_1() {
        let machines = generator(INPUT);
        assert_eq!(2, machines[0].steps_to_target());
        assert_eq!(7, part1(&machines));
    }

    #[test]
    fn test_evaluator() {
        let m = Machine::new("[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}");
        let e = Evaluator::new(&m);
        assert_eq!(e.joltage.len(), m.joltage.len());
        assert_eq!(e.joltage[0].target, 3);
        assert_eq!(e.joltage[0].buttons, vec![4, 5]);

        let m = Machine::new("[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}");
        let e = Evaluator::new(&m);
        assert_eq!(e.joltage.len(), m.joltage.len());
        assert_eq!(e.joltage[0].target, 7);
        assert_eq!(e.joltage[0].buttons, vec![0, 2, 3, ]);
        assert_eq!(e.joltage[1].target, 5);
        assert_eq!(e.joltage[1].buttons, vec![3, 4]);
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
