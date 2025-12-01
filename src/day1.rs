const START: i32 = 50;
const DIAL_SIZE: i32 = 100;

pub fn part1(rotations: &[i32]) -> u32 {
    let mut pos = START;
    let mut count = 0;
    for r in rotations.iter() {
        pos = (pos + r).rem_euclid(DIAL_SIZE);
        if pos == 0 {
            count += 1;
        }
    }
    count
}

pub fn part2(rotations: &[i32]) -> u32 {
    let mut pos = START;
    let mut count = 0u32;
    for r in rotations.iter() {
        if pos > 0 && pos + r <= 0 {
            count += 1;
        }
        count += ((pos + r) / DIAL_SIZE).unsigned_abs();
        pos = (pos + r).rem_euclid(DIAL_SIZE);
    }
    count
}

pub fn generator(input: &str) -> Vec<i32> {
    input.lines()
        .map(|x| {
            if let Some(stripped) = x.strip_prefix('L') {
                -stripped.parse::<i32>().unwrap()
            } else if let Some(stripped) = x.strip_prefix('R') {
                stripped.parse::<i32>().unwrap()
            } else {
                panic!("Invalid input: {}", x);
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{generator, part1, part2};

    const INPUT: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

    #[test]
    fn test_generator() {
        let rotations = generator(INPUT);
        assert_eq!(10, rotations.len());
        assert_eq!(vec![-68, -30, 48, -5, 60, -55, -1, -99, 14, -82], rotations);
    }

    #[test]
    fn test_part_1() {
        let rotations = generator(INPUT);
        assert_eq!(3, part1(&rotations));
    }

    #[test]
    fn test_part_2() {
        let rotations = generator(INPUT);
        assert_eq!(6, part2(&rotations));

        let rotations = generator("R1000");
        assert_eq!(10, part2(&rotations));

        let rotations = generator("R1000\nR1000");
        assert_eq!(20, part2(&rotations));

        let rotations = generator("R1000\nR1000\nL1000");
        assert_eq!(30, part2(&rotations));

        let rotations = generator("L48\nL3\nR5\nL106");
        assert_eq!(4, part2(&rotations));

        let rotations = generator("L48\nL102");
        assert_eq!(2, part2(&rotations));
    }
}
