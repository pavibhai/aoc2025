pub fn part1(battery_banks: &[Vec<u8>]) -> u64 {
    compute_total_joltage(battery_banks, 2)
}

pub fn part2(battery_banks: &[Vec<u8>]) -> u64 {
    compute_total_joltage(battery_banks, 12)
}

fn compute_total_joltage(battery_banks: &[Vec<u8>], select: usize) -> u64 {
    let mut lefts: Vec<Vec<usize>> = vec![Vec::new(); 10];
    battery_banks
        .iter()
        .map(|b| {
            let mut joltage = 0u64;
            let mut start_pos = 0;
            lefts.iter_mut().for_each(|l| l.clear());
            b.iter().enumerate().skip(start_pos).for_each(|(i, c)| {
                lefts[*c as usize].push(i);
            });
            for r in (0..select).rev() {
                let first = lefts.iter()
                    .enumerate()
                    .filter_map(|(j, i)| {
                        i.iter().find(|idx| idx >= &&start_pos && idx < &&(b.len() - r))
                            .map(|idx| (j, idx))
                    })
                    .next_back().unwrap();
                joltage = joltage * 10 + first.0 as u64;
                start_pos = first.1 + 1;
            }
            joltage
        }).sum()
}

pub fn generator(input: &str) -> Vec<Vec<u8>> {
    input.lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as u8).collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{generator, part1, part2};

    const INPUT: &str = "987654321111111
811111111111119
234234234234278
818181911112111";

    const INPUT_2: &str = "4575437413355455333663534375455245465427458234151534536655743664553344545864333436556593466693743454";

    #[test]
    fn test_generator() {
        let battery_banks = generator(INPUT);
        assert_eq!(battery_banks.len(), 4);
    }

    #[test]
    fn test_part_1() {
        let battery_banks = generator(INPUT);
        assert_eq!(part1(&battery_banks), 357);

        let battery_banks = generator(INPUT_2);
        assert_eq!(part1(&battery_banks), 99);
    }

    #[test]
    fn test_part_2() {
        let battery_banks = generator(INPUT);
        assert_eq!(part2(&battery_banks), 3121910778619);
    }
}
