use regex::Regex;
use std::collections::HashSet;
use std::fs;
use std::time::Instant;

#[derive(Debug, PartialEq)]
struct Rule {
    field: String,
    a: (u32, u32),
    b: (u32, u32),
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Could not read file.");
    let input: Vec<_> = contents.split("\n\n").collect();
    let rules = parse_rules(input[0]);
    let ticket = input[1];
    let nearby = input[2];

    let p1 = Instant::now();
    println!("Part 1: {} ({:?}).", part1(&rules, nearby), p1.elapsed());
    let p2 = Instant::now();
    println!(
        "Part 2: {} ({:?}).",
        part2(&rules, nearby, ticket),
        p2.elapsed()
    );
}

fn parse_rules(rules: &str) -> Vec<Rule> {
    let re = Regex::new(
        r"(?m)^(?P<field>[a-z ]+): (?P<min1>\d+)-(?P<max1>\d+) or (?P<min2>\d+)-(?P<max2>\d+)$",
    )
    .unwrap();

    re.captures_iter(rules)
        .map(|c| Rule {
            field: c["field"].to_owned(),
            a: (c["min1"].parse().unwrap(), c["max1"].parse().unwrap()),
            b: (c["min2"].parse().unwrap(), c["max2"].parse().unwrap()),
        })
        .collect()
}

fn part1(rules: &[Rule], nearby: &str) -> u32 {
    nearby
        .splitn(2, '\n')
        .nth(1)
        .unwrap()
        .split(|c| c == '\n' || c == ',')
        .filter_map(|s| {
            let x: u32 = s.parse().unwrap();
            if rules
                .iter()
                .any(|r| (x >= r.a.0 && x <= r.a.1) || (x >= r.b.0 && x <= r.b.1))
            {
                None
            } else {
                Some(x)
            }
        })
        .sum()
}

fn part2(rules: &[Rule], nearby: &str, ticket: &str) -> u64 {
    // valid maps a ticket value position to set of matching rules.
    let mut valid: Vec<HashSet<u32>> = vec![(0..rules.len() as u32).collect(); rules.len()];

    for l in nearby.lines().skip(1) {
        for (idx_value, x) in l.split(',').enumerate() {
            let x: u32 = x.parse().unwrap();
            let no_match: HashSet<_> = rules
                .iter()
                .enumerate()
                .filter_map(|(idx_rule, r)| {
                    if (x >= r.a.0 && x <= r.a.1) || (x >= r.b.0 && x <= r.b.1) {
                        None
                    } else {
                        Some(idx_rule as u32)
                    }
                })
                .collect();

            if no_match.len() != rules.len() {
                valid[idx_value] = valid[idx_value].difference(&no_match).cloned().collect();
            }
        }
    }

    let mut valid_rules = valid.iter().enumerate().collect::<Vec<_>>();
    valid_rules.sort_by_key(|(_, pos)| pos.len());
    let mut matched = HashSet::new();
    let mut prod = 1;
    let ticket: Vec<u32> = ticket
        .lines()
        .nth(1)
        .unwrap()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();

    for (position, matching_rules) in valid_rules {
        let rule = *matching_rules.difference(&matched).nth(0).unwrap();
        matched.insert(rule);

        if rules[rule as usize].field.starts_with("departure") {
            prod *= ticket[position as usize] as u64;
        }
    }

    prod
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parsed_rules() -> Vec<Rule> {
        vec![
            Rule {
                field: "class".to_owned(),
                a: (1, 3),
                b: (5, 7),
            },
            Rule {
                field: "row".to_owned(),
                a: (6, 11),
                b: (33, 44),
            },
            Rule {
                field: "seat".to_owned(),
                a: (13, 40),
                b: (45, 50),
            },
        ]
    }

    #[test]
    fn test_parse_rules() {
        let rules = "\
class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50";

        assert_eq!(parse_rules(rules), parsed_rules(),);
    }

    #[test]
    fn test_part1() {
        assert_eq!(
            part1(
                &parsed_rules(),
                "nearby tickets:\n7,3,47\n40,4,50\n55,2,20\n38,6,12"
            ),
            71
        );
    }

    #[test]
    fn test_part2() {
        let rules = parse_rules("class: 0-1 or 4-19\nrow: 0-5 or 8-19\nseat: 0-13 or 16-19");
        let ticket = "your ticket:\n11,12,13";
        let nearby = "nearby tickets:\n3,9,18\n15,1,5\n5,14,9";

        assert_eq!(part2(&rules, nearby, ticket), 1);
    }
}
