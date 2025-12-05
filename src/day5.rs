use std::ops::{RangeInclusive};

pub fn part1(inventory: &Inventory) -> u64 {
    inventory.count_fresh_ingredients()
}

pub fn part2(inventory: &Inventory) -> u64 {
    inventory.fresh.iter().map(|r| r.end() + 1 - r.start()).sum()
}

pub fn generator(input: &str) -> Inventory {
    Inventory::new(input)
}

pub struct Inventory {
    fresh: Vec<RangeInclusive<u64>>,
    ingredients: Vec<u64>,
}

impl Inventory {
    fn new(input: &str) -> Inventory {
        let (fresh_ranges, ingredients) = input.split_once("\n\n").unwrap();
        let mut fresh_ranges: Vec<(u64, u64)> = fresh_ranges.lines()
            .map(|line| {
                let (low, high) = line.split_once("-").unwrap();
                (low.parse().unwrap(), high.parse().unwrap())
            }).collect();
        fresh_ranges.sort();
        let mut fresh: Vec<RangeInclusive<u64>> = Vec::new();
        for (s, e) in fresh_ranges.iter() {
            match fresh.last() {
                None => fresh.push(*s..=*e),
                Some(r) if r.end() < s => fresh.push(*s..=*e),
                Some(r) if r.end() < e => {
                    let s = r.end() + 1;
                    fresh.push(s..=*e);
                }
                _ => {}
            }
        }
        let ingredients = ingredients.lines()
            .map(|line| { line.parse().unwrap() }).collect();

        Inventory { fresh, ingredients }
    }

    fn is_included(&self, v: &u64) -> bool {
        let result = self.fresh.binary_search_by(|range| {
            if range.contains(v) {
                std::cmp::Ordering::Equal
            } else if range.start() > v {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Less
            }
        });
        result.is_ok()
    }

    fn count_fresh_ingredients(&self) -> u64 {

        self.ingredients.iter().filter(|i| {
            self.is_included(i)
        }).count() as u64
    }
}

#[cfg(test)]
mod tests {
    use super::{generator, part1, part2};

    const INPUT: &str = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";
    const INPUT_2: &str = "3-5
10-14
16-20
3-5
3-3

1
5
8
11
17
32";

    #[test]
    fn test_generator() {
        let i = generator(INPUT);
        assert_eq!(i.ingredients.len(), 6);
        assert_eq!(i.fresh.len(), 4);
    }

    #[test]
    fn test_part_1() {
        let i = generator(INPUT);
        assert_eq!(part1(&i), 3);
    }

    #[test]
    fn test_part_2() {
        let i = generator(INPUT);
        assert_eq!(part2(&i), 14);

        let i = generator(INPUT_2);
        assert_eq!(part2(&i), 13);
    }
}
