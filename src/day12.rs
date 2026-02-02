use std::collections::HashMap;
use std::fmt::Display;
use crate::day12::Status::{Done, Impossible, Possible};

const WIDTH_LENGTH: usize = 3;
type Choices = u64;

fn set_bit(choices: &mut Choices, bit: u8) {
    *choices |= 1 << bit;
}

pub fn part1(situation: &Situation) -> u32 {
    situation.regions.iter()
        .filter(|&r| situation.place_presents(r))
        .count() as u32
}

pub fn part2(situation: &Situation) -> u32 {
    situation.regions.len() as u32
}

pub fn generator(input: &str) -> Situation {
    Situation::parse(input)
}

pub struct Situation {
    shapes: Vec<Shape>,
    all_shapes: Vec<Vec<Choices>>,
    regions: Vec<Region>,
    unique_choices: u64,
    ordering: Vec<usize>
}

impl Situation {
    fn parse(input: &str) -> Situation {
        let mut sections = input.split("\n\n").collect::<Vec<_>>().into_iter();
        let regions = sections.next_back().unwrap().lines()
            .map(Region::parse)
            .collect();
        let shapes: Vec<Shape> = sections.map(Shape::parse).collect();
        let shapes: Vec<Shape> = shapes.into_iter().flat_map(|s| {
            (0..3).fold(vec![s.flip_vertical(), s], |mut shapes, _| {
                let n = shapes.last().unwrap().rotate_left();
                shapes.push(n.flip_vertical());
                shapes.push(n);
                shapes
            })
        }).collect();

        let mut unique: HashMap<Vec<Vec<bool>>, Vec<usize>> = HashMap::new();
        shapes.iter().enumerate().for_each(|(o_idx, shape)| {
            let k = shape.layout.clone();
            unique.entry(k)
                .and_modify(|v| v.push(o_idx))
                .or_insert(vec![o_idx]);
        });
        let mut unique_choices = 0u64;
        for v in unique.values() {
            set_bit(&mut unique_choices, *v.first().unwrap() as u8);
        }

        let all_shapes: Vec<Vec<Choices>> = (0..WIDTH_LENGTH).map(|y| {
            (0..WIDTH_LENGTH).map(|x| {
                shapes.iter().enumerate().fold(0u64, |mut result, (shape_idx, shape)| {
                    if shape.layout[y][x] {
                        result |= 1 << shape_idx;
                    }
                    result
                }) & unique_choices
            }).collect()
        }).collect::<Vec<_>>();

        let bits: u64 = 0b11111111;
        let mut ordering: Vec<usize> = (0..shapes.len() / 8).collect();
        ordering.sort_by_key(|v| ((unique_choices >> (8 * v)) & bits).count_ones());

        Situation {
            shapes,
            all_shapes,
            regions,
            unique_choices,
            ordering
        }
    }

    fn place_presents(&self, region: &Region) -> bool {
        region.can_accommodate() && Attempt::new(region, self).place_presents()
    }
}

struct Placement {
    y: usize,
    x: usize,
    o_shape_idx: usize,
}

struct Attempt<'a> {
    choices: Vec<Vec<Choices>>,
    area: Vec<Vec<bool>>,
    placements: Vec<Placement>,
    all_shapes: &'a Vec<Vec<Choices>>,
    presents_to_place: &'a Vec<usize>,
    shapes: &'a Vec<Shape>,
    unique_choices: &'a u64,
    ordering: &'a Vec<usize>
}

impl Attempt<'_> {
    fn width(&self) -> usize {
        self.area.first().unwrap().len()
    }

    fn height(&self) -> usize {
        self.area.len()
    }

    fn new<'a>(region: &'a Region, situation: &'a Situation) -> Attempt<'a> {
        let mut choices = vec![
            vec![situation.unique_choices; region.width];
            region.length];
        // Set the possibilities as false for the last 2 rows
        for i in 1..3 {
            choices[region.length - i].iter_mut()
                .for_each(|col| *col = 0);
        }
        // Set the possibilities as false for the last 2 columns
        choices.iter_mut()
            .for_each(|row| {
                for i in 1..3 {
                    row[region.width - i] = 0;
                }
            });
        let area = vec![vec![false; region.width]; region.length];

        Attempt {
            choices,
            all_shapes: &situation.all_shapes,
            presents_to_place: &region.presents_to_place,
            shapes: &situation.shapes,
            placements: vec![],
            area,
            unique_choices: &situation.unique_choices,
            ordering: &situation.ordering
        }
    }

    fn determine_choices(&mut self, mark_y: &usize, mark_x: &usize) {
        if *mark_y + WIDTH_LENGTH > self.height() || *mark_x + WIDTH_LENGTH > self.width() {
            return;
        }

        let mut bits = *self.unique_choices;
        let mut filled = 0;

        (*mark_y..*mark_y + WIDTH_LENGTH).for_each(|y| {
            (*mark_x..*mark_x + WIDTH_LENGTH).for_each(|x| {
                if self.area[y][x] {
                    filled += 1;
                    bits &= !self.all_shapes[y - mark_y][x - mark_x];
                }
            })
        });
        if filled < 3 {
            self.choices[*mark_y][*mark_x] = bits;
        }
    }

    fn propagate_removal(&mut self, mark_y: &usize, mark_x: &usize) {
        let start_y = if *mark_y < WIDTH_LENGTH - 1 {
            0
        } else {
            mark_y + 1 - WIDTH_LENGTH
        };
        let start_x = if *mark_x < WIDTH_LENGTH - 1 {
            0
        } else {
            mark_x + 1 - WIDTH_LENGTH
        };

        (start_y..=*mark_y).for_each(|y| {
            (start_x..=*mark_x).for_each(|x| {
                self.determine_choices(&y, &x);
            })
        });
    }

    fn remove_last_placement(&mut self) -> Placement {
        // Remove last placement from area
        let placement = self.placements.pop().unwrap();
        self.shapes[placement.o_shape_idx].layout.iter().enumerate().for_each(|(y, row)| {
            row.iter().enumerate().for_each(|(x, v)| {
                if *v {
                    self.area[placement.y + y][placement.x + x] = false;
                }
            })
        });

        // Propagate the removal
        (0..WIDTH_LENGTH).for_each(|y| {
            (0..WIDTH_LENGTH).for_each(|x| {
                self.determine_choices(&(placement.y + y), &(placement.x + x));
                if self.shapes[placement.o_shape_idx].layout[y][x] {
                    self.propagate_removal(&(placement.y + y), &(placement.x + x));
                }
            })
        });
        placement
    }

    fn mark_placement(&mut self, placement: &Placement) {
        self.shapes[placement.o_shape_idx].layout.iter().enumerate().for_each(|(y, row)| {
            row.iter().enumerate().for_each(|(x, v)| {
                if *v {
                    self.choices[placement.y + y][placement.x + x] = 0;
                    self.area[placement.y + y][placement.x + x] = true;
                }
            })
        });

        // Propagate the placements
        (0..WIDTH_LENGTH).for_each(|y| {
            (0..WIDTH_LENGTH).for_each(|x| {
                if self.shapes[placement.o_shape_idx].layout[y][x] {
                    self.propagate_placement(&(placement.y + y), &(placement.x + x));
                }
            })
        });
    }

    fn propagate_placement(&mut self, mark_y: &usize, mark_x: &usize) {
        let start_y = if *mark_y < WIDTH_LENGTH - 1 {
            0
        } else {
            mark_y + 1 - WIDTH_LENGTH
        };
        let start_x = if *mark_x < WIDTH_LENGTH - 1 {
            0
        } else {
            mark_x + 1 - WIDTH_LENGTH
        };

        (start_y..=*mark_y).for_each(|y| {
            (start_x..=*mark_x).for_each(|x| {
                let bits_2_clear = !self.all_shapes[mark_y - y][mark_x - x];
                self.choices[y][x] &= bits_2_clear;
            })
        });
    }

    fn place_present_orientation(&mut self, o_shape_idx: &usize, y: &usize, x: &usize) -> bool {
        let orientation_bit = 1u64 << o_shape_idx;
        if self.choices[*y][*x] & orientation_bit == 0 {
            return false;
        }

        let placement = Placement { o_shape_idx: *o_shape_idx, y: *y, x: *x };
        self.mark_placement(&placement);

        self.placements.push(placement);
        true
    }

    fn search_and_place_present_orientation(&mut self, o_shape_idx: &usize, start_y: &usize, start_x: &mut usize) -> bool {
        for y in *start_y..=(self.height() - WIDTH_LENGTH) {
            for x in *start_x..=(self.width() - WIDTH_LENGTH) {
                if self.place_present_orientation(o_shape_idx, &y, &x) {
                    return true;
                }
            }
            *start_x = 0;
        }
        false
    }

    fn search_and_place_present(&mut self, from_idx: usize, until_idx: usize, start_y: &usize, start_x: &usize) -> bool {
        let mut start_x = *start_x;
        let mut start_y = *start_y;
        for o_shape_idx in from_idx..until_idx {
            if self.search_and_place_present_orientation(&o_shape_idx, &start_y, &mut start_x) {
                return true;
            }
            start_x = 0;
            start_y = 0;
        }
        false
    }

    fn place_presents(&mut self) -> bool {
        let mut stack = self.ordering.iter().rev()
            .flat_map(|shape_idx| vec![*shape_idx; self.presents_to_place[*shape_idx]])
            .collect::<Vec<_>>();

        let mut start: Option<(usize, usize, usize)> = None;
        while !stack.is_empty() {
            let shape_idx = stack.pop().unwrap();
            let (from_idx, y, x) = start.unwrap_or((shape_idx * 8, 0, 0));
            start = None;
            let found = self.search_and_place_present(from_idx, (shape_idx + 1) * 8, &y, &x);
            if !found {
                stack.push(shape_idx);
            }
            if !found || self.check_placement() == Impossible {
                if self.placements.is_empty() {
                    return false;
                }
                let placement = self.remove_last_placement();
                stack.push(placement.o_shape_idx / 8);
                start = Some((placement.o_shape_idx, placement.y, placement.x + 1));
            }
        }
        true
    }

    fn check_placement(&self) -> Status {
        let mut pending = self.presents_to_place.clone();
        self.placements.iter().for_each(|placement| { pending[placement.o_shape_idx / 8] -= 1 });
        if pending.iter().all(|v| v == &0) {
            Done
        } else if (0..self.presents_to_place.len())
            .map(|idx| {
                let marker = 0b11111111 << (idx * 8);
                self.choices.iter().flatten()
                    .filter(|&&e| e & marker > 0)
                    .count()
            }).zip(pending)
            .all(|(possible, target)| possible >= target) {
            Possible
        } else {
            Impossible
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
enum Status {
    Done,
    Impossible,
    Possible,
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct Shape {
    layout: Vec<Vec<bool>>,
}

impl Display for Shape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let mut output = String::new();
        self.layout.iter().for_each(|row| {
            row.iter().for_each(|col| if *col { output.push('#'); } else { output.push('.'); });
            output.push('\n');
        });
        write!(f, "{}", output)
    }
}

impl Shape {
    fn parse(input: &str) -> Self {
        let mut lines = input.lines();
        lines.next().unwrap();
        let layout: Vec<Vec<bool>> = lines
            .map(|line| line.chars().map(|c| c == '#').collect()).collect();
        assert!(layout.iter().all(|l| l.len() == WIDTH_LENGTH));
        assert_eq!(layout.len(), WIDTH_LENGTH);
        Shape { layout }
    }

    fn rotate_left(&self) -> Self {
        let mut layout = self.layout.clone();
        for y in 0..layout.len() {
            for x in 0..layout[y].len() {
                layout[WIDTH_LENGTH - 1 - x][y] = self.layout[y][x];
            }
        }
        Shape { layout }
    }

    fn flip_vertical(&self) -> Self {
        let mut layout = self.layout.clone();
        for y in 0..layout.len() {
            for x in 0..layout[y].len() {
                layout[y][2 - x] = self.layout[y][x];
            }
        }
        Shape { layout }
    }
}

struct Region {
    width: usize,
    length: usize,
    presents_to_place: Vec<usize>,
}

impl Region {
    fn parse(input: &str) -> Self {
        let (dimensions, shapes) = input.split_once(": ").unwrap();
        let shapes = shapes.split(' ').map(|s| s.parse::<usize>().unwrap()).collect();
        let (width, length) = dimensions.split_once("x").unwrap();
        let width = width.parse::<usize>().unwrap();
        let length = length.parse::<usize>().unwrap();

        Region {
            width,
            length,
            presents_to_place: shapes,
        }
    }

    fn can_accommodate(&self) -> bool {
        self.presents_to_place.iter().sum::<usize>() * 7 <= (self.length * self.width)
    }
}

#[cfg(test)]
mod tests {
    use crate::day12::Status::{Done, Impossible, Possible};
    use super::{generator, part1, Attempt, Region, Shape, WIDTH_LENGTH};

    const INPUT: &str = "0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2";

    #[test]
    fn test_generator() {
        let s = generator(INPUT);
        assert_eq!(s.shapes.len(), 48);
        assert_eq!(s.regions.len(), 3);
        assert_eq!(s.all_shapes.len(), WIDTH_LENGTH);
        assert_eq!(s.all_shapes[0].len(), WIDTH_LENGTH);
        assert_eq!(s.all_shapes[0][0], 0b111111111111111111010010100110010110111111011011 & s.unique_choices);
        assert_eq!(s.all_shapes[2][2], 0b111111111111111100101101100110011111011010111101 & s.unique_choices);
    }

    #[test]
    fn test_shape() {
        let shape = Shape::parse("0:\n###\n##.\n##.");
        assert_eq!(shape.layout, vec![vec![true, true, true], vec![true, true, false], vec![true, true, false]]);
        assert_eq!(shape.rotate_left(), Shape::parse("0:\n#..\n###\n###"));

        assert_eq!(shape.flip_vertical(), Shape::parse("0:\n###\n.##\n.##"));
    }

    #[test]
    fn test_region() {
        let r = Region::parse("4x4: 0 0 0 0 2 0");
        assert!(r.can_accommodate());

        let r = Region::parse("4x3: 0 0 0 0 2 0");
        assert!(!r.can_accommodate());
    }

    #[test]
    fn test_attempt() {
        let s = generator(INPUT);
        let a = Attempt::new(&s.regions[0], &s);
        assert_eq!(a.choices.len(), s.regions[0].length);
        assert_eq!(a.choices[0].len(), s.regions[0].width);
        assert!(a.choices[s.regions[0].length - 2..].iter()
            .all(|row| row.iter().all(|v| v == &0)));
        assert!(a.choices.iter()
            .all(|row| row[s.regions[0].width - 2..].iter().all(|v| v == &0)));
        assert_eq!(a.check_placement(), Possible);
    }

    #[test]
    fn test_placement_removal() {
        let s = generator(INPUT);
        let r = Region::parse("4x4: 0 0 0 0 2 0");
        let mut a = Attempt::new(&r, &s);

        let expect_choices = a.choices.clone();
        assert!(a.place_present_orientation(&32, &0, &0));
        assert!(a.choices.iter().all(|row| row.iter().all(|v| v == &0)));
        assert_eq!(a.check_placement(), Impossible);
        a.remove_last_placement();
        assert!(a.area.iter().all(|row| row.iter().all(|v| !v)));
        assert_eq!(a.choices, expect_choices);
        assert_eq!(a.check_placement(), Possible);

        assert!(a.place_present_orientation(&32, &1, &1));
        assert!(!a.choices.iter().all(|row| row.iter().all(|v| v == &0)));
        assert_eq!(a.choices[0][0], (1 << 33) & s.unique_choices);
        assert_eq!(a.check_placement(), Possible);

        assert!(a.place_present_orientation(&33, &0, &0));
        assert!(a.choices.iter().all(|row| row.iter().all(|v| v == &0)));
        assert_eq!(a.check_placement(), Done);
        a.remove_last_placement();
        assert!(!a.choices.iter().all(|row| row.iter().all(|v| v == &0)));
        assert_eq!(a.choices[0][0], 1 << 33);
        assert_eq!(a.check_placement(), Possible);
        a.remove_last_placement();
        assert!(a.area.iter().all(|row| row.iter().all(|v| !v)));
        assert_eq!(a.choices, expect_choices);
        assert_eq!(a.check_placement(), Possible);
    }

    #[test]
    fn test_part_1() {
        let s = generator(INPUT);
        assert_eq!(part1(&s), 2);
    }
}