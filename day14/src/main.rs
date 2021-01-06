use std::collections::HashMap;
use std::fs;
use std::time::Instant;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Could not read file.");

    let p1 = Instant::now();
    println!("Part 1: {} ({:?}).", part1(&contents), p1.elapsed());
    let p2 = Instant::now();
    println!("Part 2: {} ({:?}).", part2(&contents), p2.elapsed());
}

fn part1(input: &str) -> u64 {
    let mut mem: HashMap<u64, u64> = HashMap::new();
    let mut ones = 0u64;
    let mut zeroes = 0u64;

    for line in input.lines() {
        match &line[..2] {
            "ma" => {
                let res = parse_mask(line);
                ones = res.0;
                zeroes = res.1;
            }
            "me" => {
                let res = parse_mem(line);
                mem.insert(res.0, (res.1 | ones) & !zeroes);
            }
            _ => panic!("Invalid input \"{}\".", line),
        }
    }

    mem.values().sum()
}

fn parse_mask(line: &str) -> (u64, u64) {
    let (_, mask) = line.split_at(7);
    let mut ones = 0u64;
    let mut zeroes = 0u64;
    mask.bytes()
        .rev()
        .enumerate()
        .for_each(|(idx, byte)| match byte {
            b'1' => ones |= 1 << idx,
            b'0' => zeroes |= 1 << idx,
            _ => {}
        });

    (ones, zeroes)
}

fn parse_mem(line: &str) -> (u64, u64) {
    let split: Vec<&str> = line.split(" = ").collect();
    let addr = split[0][4..split[0].len() - 1]
        .parse()
        .expect("Could not parse address.");
    let val = split[1].parse().expect("Could not parse value.");
    (addr, val)
}

fn part2(input: &str) -> u64 {
    let mut mem: HashMap<u64, u64> = HashMap::new();
    let mut mask: &str = "000000000000000000000000000000000000";
    let mut xs = Vec::new();

    for line in input.lines() {
        match &line.as_bytes()[1] {
            b'a' => {
                mask = &line[7..];
                xs = mask
                    .bytes()
                    .rev()
                    .enumerate()
                    .filter_map(|(idx, m)| if m == b'X' { Some(idx) } else { None })
                    .collect();
            }
            b'e' => {
                let res = parse_mem(line);
                mem.extend(apply_mask(res.0, mask, &xs, res.1));
            }
            _ => panic!("Invalid input \"{}\".", line),
        }
    }

    mem.values().sum()
}

fn apply_mask(addr: u64, mask: &str, xs: &[usize], value: u64) -> Vec<(u64, u64)> {
    let base_addr: u64 = mask
        .bytes()
        .rev()
        .enumerate()
        .fold(0u64, |acc, (idx, byte)| match (addr >> idx & 1, byte) {
            (1, b'0') | (_, b'1') => acc | 1 << idx,
            (_, b'X') => acc & !(1 << idx),
            _ => acc,
        });
    let num_mutations = 2u64.pow(xs.len() as u32);
    let mut pairs = Vec::with_capacity(num_mutations as usize);

    for perm in 0..num_mutations {
        let mut masked_addr = base_addr;

        for (idx_perm, idx_addr) in xs.iter().enumerate() {
            masked_addr |= ((perm >> idx_perm) & 1) << idx_addr
        }
        pairs.push((masked_addr, value))
    }

    pairs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_mask() {
        assert_eq!(
            parse_mask("mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X"),
            (64, 2)
        );
    }

    #[test]
    fn test_parse_mem() {
        assert_eq!(parse_mem("mem[8] = 11"), (8, 11));
    }

    #[test]
    fn test_part1() {
        let input = "\
mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0";

        assert_eq!(part1(input), 165);
    }

    #[test]
    fn test_part2() {
        let input = "\
mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1";

        assert_eq!(part2(input), 208);
    }
}
