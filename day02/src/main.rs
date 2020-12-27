use regex::Regex;
use std::fs;

#[derive(PartialEq, Debug)]
struct Rule {
    character: char,
    a: usize,
    b: usize,
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("File not found.");
    let pairs = parse(&contents);
    let part1 = pairs.iter().filter(|(r, p)| validate(r, p)).count();
    let part2 = pairs.iter().filter(|(r, p)| validate2(r, p)).count();

    println!("Part1: {}", part1);
    println!("Part2: {}", part2);
}

fn parse(input: &str) -> Vec<(Rule, String)> {
    let re =
        Regex::new(r"(?m)^(?P<a>\d+)-(?P<b>\d+) (?P<char>[a-z]): (?P<password>[a-z]+)$").unwrap();

    re.captures_iter(input)
        .map(|c| {
            (
                Rule {
                    character: c["char"].parse().unwrap(),
                    a: c["a"].parse().unwrap(),
                    b: c["b"].parse().unwrap(),
                },
                String::from(&c["password"]),
            )
        })
        .collect()
}

fn validate(rule: &Rule, password: &str) -> bool {
    let count = password.matches(rule.character).count();

    count >= rule.a && count <= rule.b
}

fn validate2(rule: &Rule, password: &str) -> bool {
    let bytes = password.as_bytes();
    (bytes[rule.a - 1] as char == rule.character) ^ (bytes[rule.b - 1] as char == rule.character)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_rules() -> [(Rule, String); 3] {
        [
            (
                Rule {
                    character: 'a',
                    a: 1,
                    b: 3,
                },
                String::from("abcde"),
            ),
            (
                Rule {
                    character: 'b',
                    a: 1,
                    b: 3,
                },
                String::from("cdefg"),
            ),
            (
                Rule {
                    character: 'c',
                    a: 2,
                    b: 9,
                },
                String::from("ccccccccc"),
            ),
        ]
    }

    #[test]
    fn test_parse() {
        let input = "1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc";

        assert_eq!(parse(input), get_rules());
    }

    #[test]
    fn test_validate() {
        let rules = get_rules();

        assert_eq!(validate(&rules[0].0, &rules[0].1), true);
        assert_eq!(validate(&rules[1].0, &rules[1].1), false);
        assert_eq!(validate(&rules[2].0, &rules[2].1), true);
    }

    #[test]
    fn test_validate2() {
        let rules = get_rules();

        assert_eq!(true ^ false, true);
        assert_eq!(validate2(&rules[0].0, &rules[0].1), true);
        assert_eq!(validate2(&rules[1].0, &rules[1].1), false);
        assert_eq!(validate2(&rules[2].0, &rules[2].1), false);
    }
}
