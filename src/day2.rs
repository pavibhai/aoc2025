use itertools::Itertools;

pub fn part1(ranges: &[(u64, u64)]) -> u64 {
    find_invalid_ids(ranges, true).iter().sum()
}

pub fn part2(ranges: &[(u64, u64)]) -> u64 {
    find_invalid_ids(ranges, false).iter().sum()
}

fn find_invalid_ids(ranges: &[(u64, u64)], single_split: bool) -> Vec<u64> {
    let mut invalid_ids = Vec::new();
    for (l, h) in ranges {
        if h < &10 { continue; }
        let digits = h.checked_ilog10().unwrap_or(0) + 1;
        if single_split && digits % 2 == 1 { continue; }
        let max_digits = h.checked_ilog10().unwrap_or(0).div_ceil(2);
        let min_digits = if single_split { max_digits } else { 1 };
        for d in min_digits..=max_digits {
            if digits % d == 0 {
                let mul = 10u64.pow(digits - d);
                let l_value = l / mul;
                let h_value = h / mul;
                (l_value..=h_value)
                    .map(|v| (0..(digits / d))
                        .fold(0, |acc, _| (acc * 10u64.pow(d)) + v))
                    .filter(|&v| !(&v < l || &v > h))
                    .for_each(|v| { invalid_ids.push(v); });
            }
        }
    }
    if single_split {
        invalid_ids
    } else {
        invalid_ids.sort_unstable();
        invalid_ids.into_iter().dedup().collect()
    }
}

fn find_ranges(l: &u64, h: &u64) -> Vec<(u64, u64)> {
    let mut output = Vec::new();
    let l_digits = l.checked_ilog10().unwrap_or(0) + 1;
    let h_digits = h.checked_ilog10().unwrap_or(0) + 1;
    for d in l_digits..=h_digits {
        let l = if d == l_digits { *l } else { 10u64.pow(d - 1) };
        let h = if d == h_digits { *h } else { 10u64.pow(d) - 1 };
        output.push((l, h));
    }
    output
}

pub fn generator(input: &str) -> Vec<(u64, u64)> {
    let mut ranges: Vec<(u64, u64)> = input.trim().split(',')
        .flat_map(|i| {
            let range = i.split_once('-').unwrap();
            find_ranges(&range.0.parse().unwrap(), &range.1.parse().unwrap())
        }).collect();
    // We should not have any overlapping ranges
    ranges.sort_unstable_by_key(|r| r.0);
    if ranges.windows(2).all(|w| w[0].1 < w[1].0) == false {
        panic!("Overlapping ranges are not supported!!!")
    }
    ranges
}

#[cfg(test)]
mod tests {
    use super::{generator, part1, part2};

    const INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn test_generator() {
        let ranges = generator(INPUT);
        assert_eq!(13, ranges.len());
    }

    #[test]
    fn test_part_1() {
        let ranges = generator(INPUT);
        assert_eq!(1227775554, part1(&ranges));
    }

    #[test]
    fn test_part_2() {
        let ranges = generator(INPUT);
        assert_eq!(4174379265, part2(&ranges));
    }
}
