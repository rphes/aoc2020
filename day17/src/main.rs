use std::collections::HashSet;
use std::fs;

type Coord = (i32, i32, i32, i32);

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Could not read file.");

    let init = parse(&contents);
    let count = run(init, 6, false);
    println!("Part 1: {}.", count);

    let init = parse(&contents);
    let count = run(init, 6, true);
    println!("Part 2: {}.", count);
}

fn parse(input: &str) -> HashSet<Coord> {
    let mut active = HashSet::new();

    for (y, line) in input.lines().enumerate() {
        for (x, chr) in line.bytes().enumerate() {
            if chr == '#' as u8 {
                active.insert((x as i32, y as i32, 0, 0));
            }
        }
    }

    active
}

fn neighbors(coord: &Coord, four_d: bool) -> Vec<Coord> {
    let mut neighbors = Vec::with_capacity(27);

    for x in coord.0 - 1..coord.0 + 2 {
        for y in coord.1 - 1..coord.1 + 2 {
            for z in coord.2 - 1..coord.2 + 2 {
                if four_d {
                    for w in coord.3 - 1..coord.3 + 2 {
                        neighbors.push((x, y, z, w));
                    }
                } else {
                    neighbors.push((x, y, z, 0));
                }
            }
        }
    }

    neighbors
}

fn step(active: &HashSet<Coord>, four_d: bool) -> HashSet<Coord> {
    let to_update: HashSet<Coord> = active.iter().flat_map(|c| neighbors(c, four_d)).collect();

    to_update
        .iter()
        .filter_map(|c| {
            let num_active_neighbors = active
                .intersection(&neighbors(c, four_d).iter().cloned().collect())
                .count();

            match (active.contains(c), num_active_neighbors) {
                (true, 4) => Some(*c),
                (_, 3) => Some(*c),
                _ => None,
            }
        })
        .collect()
}

fn run(init: HashSet<Coord>, steps: u32, four_d: bool) -> usize {
    let mut active = init;

    for _ in 0..steps {
        active = step(&active, four_d);
    }

    active.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn init() -> HashSet<Coord> {
        [
            (1, 0, 0, 0),
            (2, 1, 0, 0),
            (0, 2, 0, 0),
            (1, 2, 0, 0),
            (2, 2, 0, 0),
        ]
        .iter()
        .cloned()
        .collect::<HashSet<Coord>>()
    }

    #[test]
    fn test_parse() {
        let input = ".#.\n..#\n###";
        assert_eq!(parse(input), init());
    }

    #[test]
    fn test_step() {
        let t0 = init();
        let t1 = step(&t0, false);
        let t2 = step(&t1, false);
        let t3 = step(&t2, false);

        assert_eq!(t1.len(), 11);
        assert_eq!(t2.len(), 21);
        assert_eq!(t3.len(), 38);

        let t1 = step(&t0, true);
        let t2 = step(&t1, true);

        assert_eq!(t1.len(), 29);
        assert_eq!(t2.len(), 60);
    }

    #[test]
    fn test_run() {
        assert_eq!(run(init(), 6, false), 112);
        assert_eq!(run(init(), 6, true), 848);
    }
}
