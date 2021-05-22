use std::fs;
use std::time::Instant;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Could not read file.");

    let p1 = Instant::now();
    println!("Part 1: {} ({:?}).", part1(&contents), p1.elapsed());
    let p2 = Instant::now();
    println!("Part 2: {} ({:?}).", part2(&contents), p2.elapsed());
}

type GetPrecedence = fn(u8) -> Option<i8>;

fn op_precedence1(op: u8) -> Option<i8> {
    match op {
        b'*' | b'+' => Some(0),
        _ => None,
    }
}

fn op_precedence2(op: u8) -> Option<i8> {
    match op {
        b'*' => Some(0),
        b'+' => Some(1),
        _ => None,
    }
}

fn apply_op(lhs: u64, op: u8, rhs: u64) -> u64 {
    match op {
        b'*' => lhs * rhs,
        b'+' => lhs + rhs,
        _ => panic!("Invalid operator."),
    }
}

fn parse_primary(tokens: &[u8], index: &mut usize, get_precedence: GetPrecedence) -> u64 {
    match tokens.get(*index) {
        Some(b'(') => {
            *index += 1;
            let res = parse_expression(tokens, index, 0, get_precedence);
            if tokens.get(*index) != Some(&b')') {
                panic!("Expected ')'");
            }
            *index += 1;

            res
        }
        Some(&c) if c.is_ascii_digit() => {
            let res = (c as char).to_digit(10).unwrap() as u64;
            *index += 1;
            res
        }
        None => panic!("Unexpected EOL."),
        _ => panic!("Invalid token."),
    }
}

fn parse_expression<'a>(
    tokens: &[u8],
    index: &mut usize,
    min_precedence: i8,
    get_precedence: GetPrecedence,
) -> u64 {
    let mut lhs = parse_primary(tokens, index, get_precedence);

    while let Some(op) = tokens.get(*index) {
        if get_precedence(*op).map_or(true, |x| x < min_precedence) {
            break;
        }
        *index += 1;
        let rhs = parse_expression(tokens, index, min_precedence + 1, get_precedence);
        lhs = apply_op(lhs, *op, rhs);
    }

    lhs
}

fn do_homework(input: &str, get_precedence: GetPrecedence) -> u64 {
    input
        .lines()
        .map(|l| {
            parse_expression(
                &l.as_bytes()
                    .iter()
                    .copied()
                    .filter(|&x| x != b' ')
                    .collect::<Vec<_>>(),
                &mut 0,
                0,
                get_precedence,
            )
        })
        .sum()
}

fn part1(input: &str) -> u64 {
    do_homework(input, op_precedence1)
}

fn part2(input: &str) -> u64 {
    do_homework(input, op_precedence2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("1 + (2 * 3) + (4 * (5 + 6))"), 51);
        assert_eq!(part1("2 * 3 + (4 * 5)"), 26);
        assert_eq!(part1("5 + (8 * 3 + 9 + 3 * 4 * 3)"), 437);
        assert_eq!(part1("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"), 12240);
        assert_eq!(
            part1("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"),
            13632
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("1 + (2 * 3) + (4 * (5 + 6))"), 51);
        assert_eq!(part2("2 * 3 + (4 * 5)"), 46);
        assert_eq!(part2("5 + (8 * 3 + 9 + 3 * 4 * 3)"), 1445);
        assert_eq!(part2("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"), 669060);
        assert_eq!(
            part2("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"),
            23340
        );
    }
}
