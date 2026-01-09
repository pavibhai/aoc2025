use std::collections::HashMap;
use itertools::Itertools;

pub fn part1(reactor: &Reactor) -> u64 {
    reactor.find_paths_between_names("you", "out")
}

pub fn part2(reactor: &Reactor) -> u64 {
    let mut r = reactor.clone();
    let mut paths = r.find_paths_between_names("svr", "out");
    r.devices[r.device_codes["dac"]].clear();
    r.devices[r.device_codes["fft"]].clear();
    let paths_wo_both = r.find_paths_between_names("svr", "out");

    r.devices[r.device_codes["dac"]] = reactor.devices[r.device_codes["dac"]].clone();
    paths -= r.find_paths_between_names("svr", "out") - paths_wo_both;

    r.devices[r.device_codes["fft"]] = reactor.devices[r.device_codes["fft"]].clone();
    r.devices[r.device_codes["dac"]].clear();
    paths -= r.find_paths_between_names("svr", "out") - paths_wo_both;

    paths - paths_wo_both
}

pub fn generator(input: &str) -> Reactor {
    Reactor::new(input)
}

#[derive(Clone)]
pub struct Reactor {
    devices: Vec<Vec<usize>>,
    device_codes: HashMap<String, usize>,
}

impl Reactor {
    fn new(input: &str) -> Reactor {
        let mut device_codes = Vec::new();
        let mut devices_str = Vec::new();
        input.lines()
            .map(|line| {
                let (d, outs) = line.split_once(": ").unwrap();
                let outs = outs.split(' ').collect::<Vec<_>>();
                (d, outs)
            })
            .for_each(|(d, outs)| {
                device_codes.push(d);
                outs.iter().for_each(|o| device_codes.push(*o));
                devices_str.push((d, outs));
            });
        let device_codes = device_codes.iter()
            .unique()
            .enumerate()
            .map(|(idx, item)| (item.to_string(), idx))
            .collect::<HashMap<_, _>>();

        let mut devices = vec![Vec::new(); device_codes.len()];
        devices_str.iter().for_each(|(d, outs)| {
            let id = device_codes.get(*d).unwrap();
            outs.iter()
                .for_each(|o| {
                    devices[*id].push(device_codes[*o]);
                });
        });

        Reactor {
            devices,
            device_codes,
        }
    }

    fn find_paths_between_names(&self, start: &str, end: &str) -> u64 {
        let mut cache = vec![None; self.device_codes.len()];
        self.find_paths_between_codes(&self.device_codes[start], &self.device_codes[end], &mut cache)
    }

    fn find_paths_between_codes(&self, start: &usize, end: &usize, cache: &mut [Option<u64>]) -> u64 {
        //println!("Found start: {}", start);
        match start {
            v if cache[*v].is_some() => cache[*v].unwrap(),
            v => {
                cache[*v] = Some(self.devices[*v].iter()
                    .map(|next| {
                        if next == end {
                            1
                        } else {
                            self.find_paths_between_codes(next, end, cache)
                        }
                    }).sum());
                cache[*v].unwrap()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{generator, part1, part2};

    const INPUT: &str = "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out";

    const INPUT_2: &str = "svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out";

    #[test]
    fn test_generator() {
        let r = generator(INPUT);
        assert_eq!(r.devices.len(), 11);
        assert_eq!(r.devices[r.device_codes["you"]], vec![r.device_codes["bbb"], r.device_codes["ccc"]]);
        assert_eq!(r.devices.len(), r.device_codes.len());
    }

    #[test]
    fn test_part_1() {
        let r = generator(INPUT);
        assert_eq!(part1(&r), 5);
    }

    #[test]
    fn test_part_2() {
        let r = generator(INPUT_2);
        assert_eq!(part2(&r), 2);
    }
}
