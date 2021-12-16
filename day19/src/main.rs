use std::collections::HashMap;
use std::fs;
use std::time::Instant;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Could not read file.");
    let (messages, mut map) = parse_input(&contents);

    let p1 = Instant::now();
    println!("Part 1: {} ({:?}).", part1(&messages, &map), p1.elapsed());
    let p2 = Instant::now();
    println!(
        "Part 2: {} ({:?}).",
        part2(&messages, &mut map),
        p2.elapsed()
    );
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Symbol {
    Terminal(char),
    Rule(usize),
}
type SubRule = Vec<Symbol>;
type Rule = Vec<SubRule>;
type Map = HashMap<usize, Rule>;

fn parse_input(string: &str) -> (Vec<Vec<char>>, Map) {
    let mut split = string.splitn(2, "\n\n");
    let rules = split.next().unwrap().lines().collect::<Vec<_>>();
    let messages: Vec<Vec<char>> = get_messages(split.next().unwrap());
    let map = build_map(&rules);

    return (messages, map);
}

fn parse_symbol(string: &str) -> Symbol {
    if let Ok(i) = string.parse::<usize>() {
        return Symbol::Rule(i);
    }
    if let Some(c) = string.trim_matches('"').chars().next() {
        if c.is_ascii_lowercase() {
            return Symbol::Terminal(c);
        }
    }
    panic!("Could not parse symbol");
}

fn parse_rule(line: &str) -> (usize, Rule) {
    let (index, rule) = line.split_once(": ").expect("Invalid rule.");
    let sub_rules = rule.split(" | ");

    (
        index.parse().expect("Could not parse rule index"),
        sub_rules
            .map(|s| s.split(' ').map(|x| parse_symbol(x)).collect::<Vec<_>>())
            .collect::<Vec<_>>(),
    )
}

fn build_map(rules: &[&str]) -> Map {
    rules.iter().map(|x| parse_rule(x)).collect()
}

fn get_messages(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

type Stack = Vec<(Symbol, Vec<Symbol>, usize)>;

fn push_stack(stack: &mut Stack, rules: &[Symbol], matched: usize) {
    let (head, tail) = rules.split_first().unwrap();
    stack.push((*head, tail.to_vec(), matched));
}

fn full_match(message: &[char], map: &Map) -> bool {
    let mut stack: Stack = map[&0]
        .iter()
        .map(|sub_rule| (sub_rule[0], sub_rule.get(1..).unwrap().to_vec(), 0usize))
        .collect();

    while let Some((symbol, rest, matched)) = stack.pop() {
        match symbol {
            Symbol::Terminal(c) => {
                if Some(&c) == message.get(matched) {
                    let new_matched = matched + 1;
                    if rest.is_empty() {
                        if new_matched == message.len() {
                            return true;
                        }
                        if new_matched > message.len() {
                            return false;
                        }
                    } else {
                        push_stack(&mut stack, &rest, new_matched)
                    }
                }
            }
            Symbol::Rule(i) => {
                let new_rule = map.get(&i).expect("Unknown rule");

                for sub_rule in new_rule {
                    let mut new_rest = sub_rule.clone();
                    new_rest.extend(rest.clone());
                    push_stack(&mut stack, &new_rest, matched);
                }
            }
        };
    }
    return false;
}

fn part1(messages: &Vec<Vec<char>>, map: &Map) -> usize {
    messages
        .iter()
        .filter(|m| {
            let does_match = full_match(m, map);
            does_match
        })
        .count()
}

fn part2(messages: &Vec<Vec<char>>, map: &mut Map) -> usize {
    map.extend([
        parse_rule("8: 42 | 42 8"),
        parse_rule("11: 42 31 | 42 11 31"),
    ]);
    messages
        .iter()
        .filter(|m| {
            let does_match = full_match(m, map);
            does_match
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_map() -> Map {
        use Symbol::*;
        [
            (0, vec![vec![Rule(1), Rule(2)]]),
            (1, vec![vec![Terminal('a')]]),
            (2, vec![vec![Rule(1), Rule(3)], vec![Rule(3), Rule(1)]]),
            (3, vec![vec![Terminal('b')]]),
        ]
        .iter()
        .cloned()
        .collect()
    }

    #[test]
    fn test_build_map() {
        let input = vec!["0: 1 2", "1: \"a\"", "2: 1 3 | 3 1", "3: \"b\""];
        assert_eq!(build_map(&input), get_map());
    }

    #[test]
    fn test_full_match() {
        let map = get_map();

        assert_eq!(
            full_match(&("aab".chars().collect::<Vec<_>>()), &map,),
            true
        );
        assert_eq!(
            full_match(&("aba".chars().collect::<Vec<_>>()), &map,),
            true
        );
        assert_eq!(
            full_match(&("abaa".chars().collect::<Vec<_>>()), &map,),
            false
        );
        assert_eq!(
            full_match(&("ab".chars().collect::<Vec<_>>()), &map,),
            false
        );
        assert_eq!(
            full_match(&("baa".chars().collect::<Vec<_>>()), &map,),
            false
        );
    }

    #[test]
    fn test_part1() {
        let input = "0: 4 1 5
1: 2 3 | 3 2
4: \"a\"
3: 4 5 | 5 4
2: 4 4 | 5 5
5: \"b\"

ababbb
bababa
abbbab
aaabbb
aaaabbb";
        let (messages, map) = parse_input(&input);
        println!("{:?}", map);

        assert_eq!(part1(&messages, &map), 2);
    }

    #[test]
    fn test_part2() {
        let input = "42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: \"a\"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: \"b\"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1

abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
bbabbbbaabaabba
babbbbaabbbbbabbbbbbaabaaabaaa
aaabbbbbbaaaabaababaabababbabaaabbababababaaa
bbbbbbbaaaabbbbaaabbabaaa
bbbababbbbaaaaaaaabbababaaababaabab
ababaaaaaabaaab
ababaaaaabbbaba
baabbaaaabbaaaababbaababb
abbbbabbbbaaaababbbbbbaaaababb
aaaaabbaabaaaaababaa
aaaabbaaaabbaaa
aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
babaaabbbaaabaababbaabababaaab
aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba";

        let (messages, mut map) = parse_input(&input);
        assert_eq!(part1(&messages, &map), 3);
        assert_eq!(part2(&messages, &mut map), 12);
    }
}
