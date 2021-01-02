use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Could not read file.");
    let lines: Vec<_> = contents.lines().collect();
    let ts = lines[0].parse::<u32>().expect("Could not parse timestamp.");
    let ids = lines[1];

    println!("Part 1: {}.", part1(ts, &ids));
    println!("Part 2: {}.", part2(&ids));
}

fn part1(ts: u32, ids: &str) -> u32 {
    let id = ids
        .split(',')
        .filter_map(|s| s.parse().ok())
        .min_by_key(|x| x - ts % x)
        .expect("No result found");

    id * (id - ts % id)
}

fn crt(pairs: &[(i64, i64)]) -> i64 {
    #[allow(non_snake_case)]
    let N: i64 = pairs.iter().map(|x| x.0).product();

    let res = pairs.iter().fold(0, |res, (n_i, a_i)| {
        let a = n_i;
        let b = N / n_i;
        let (_, _, s_i) = extended_euclid(*a, b);
        res + a_i * s_i * b
    });

    res.rem_euclid(N)
}

fn extended_euclid(x: i64, y: i64) -> (i64, i64, i64) {
    let (mut x, mut x0, mut x1, mut y, mut y0, mut y1) = (x, 1, 0, y, 0, 1);

    while y > 0 {
        let q = x / y;
        let y_old = y;
        y = x % y;
        x = y_old;
        let x0_old = x0;
        x0 = x1;
        x1 = x0_old - q * x1;
        let y0_old = y0;
        y0 = y1;
        y1 = y0_old - q * y1;
    }

    (x, x0, y0)
}

fn part2(ids: &str) -> i64 {
    let pairs: Vec<_> = ids
        .split(',')
        .enumerate()
        .filter_map(|(i, s)| s.parse().ok().map(|x| (x, x - i as i64)))
        .collect();

    crt(&pairs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(939, "7,13,x,x,59,x,31,19"), 295);
    }

    #[test]
    fn test_extended_euclid() {
        assert_eq!(extended_euclid(102, 38), (2, 3, -8));
    }

    #[test]
    fn test_crt() {
        assert_eq!(crt(&[(3, 0), (4, 3), (5, 4)]), 39);
        assert_eq!(
            crt(&[(7, 0), (13, 12), (59, 55), (31, 25), (19, 12)]),
            1068781
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("7,13,x,x,59,x,31,19"), 1068781);
    }
}
