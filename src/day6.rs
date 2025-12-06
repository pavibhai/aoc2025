pub fn part1(homework: &Homework) -> u64 {
    homework.solve()
}

pub fn part2(homework: &Homework) -> u64 {
    homework.solve_cephalopod()
}

pub fn generator(input: &str) -> Homework {
    Homework::new(input)
}

pub struct Homework {
    operators: Vec<Operator>,
    operands: Vec<String>,
}

impl Homework {
    fn new(input: &str) -> Homework {
        let mut lines = input.lines().rev();
        let operators: Vec<Operator> = lines.next().unwrap()
            .split_whitespace()
            .map(|o| match o {
                "+" => Operator::Add,
                "*" => Operator::Multiply,
                _ => unreachable!(),
            }).collect();
        let operands = lines.rev()
            .map(|o| o.to_string())
            .collect();

        Homework { operators, operands }
    }

    fn solve(&self) -> u64 {
        let operands: Vec<Vec<u64>> = self.operands.iter().map(|l| l.split_whitespace()
            .map(|o| o.parse::<u64>().unwrap()).collect())
            .collect();

        (0..self.operators.len()).map(|i| {
            match self.operators[i] {
                Operator::Add => operands.iter().map(|r| r[i]).sum::<u64>(),
                Operator::Multiply => operands.iter().map(|r| r[i]).product(),
            }
        }).sum::<u64>()
    }

    fn solve_cephalopod(&self) -> u64 {
        let width = self.operands.first().unwrap().chars().count();

        let operands: Vec<Vec<u64>> = self.operands.iter()
            .map(|l| l.chars()
                .map(|c| c.to_digit(10).unwrap_or(0) as u64)
                .collect())
            .collect();

        let operands: Vec<u64> = (0..width)
            .map(|i| { operands.iter().map(move |r| r[i]) })
            .map(|c| {
                c.filter(|v| v > &0).fold(0, |acc, x| (acc * 10) + x)
            }).collect();
        let operands: Vec<Vec<u64>> = operands.split(|v| v == &0).map(|i| i.to_vec())
            .collect();

        (0..self.operators.len()).map(|i| {
            match self.operators[i] {
                Operator::Add => operands[i].iter().sum::<u64>(),
                Operator::Multiply => operands[i].iter().product(),
            }
        }).sum::<u64>()
    }
}

enum Operator {
    Add,
    Multiply,
}

#[cfg(test)]
mod tests {
    use super::{generator, part1, part2};

    const INPUT: &str = "123 328  51 64 \n 45 64  387 23 \n  6 98  215 314\n*   +   *   +  ";

    #[test]
    fn test_generator() {
        let h = generator(INPUT);
        assert_eq!(h.operators.len(), 4);
        assert_eq!(h.operands.len(), 3);
    }

    #[test]
    fn test_part_1() {
        let h = generator(INPUT);
        assert_eq!(part1(&h), 4277556);
    }

    #[test]
    fn test_part_2() {
        let h = generator(INPUT);
        assert_eq!(part2(&h), 3263827);
    }
}
