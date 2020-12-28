use std::collections::HashSet;
use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Could not read file.");
    let groups: Vec<&str> = contents.split("\n\n").collect();

    let sum = count_unique_answers(&groups);
    println!("Part 1: {}.", sum);
    let sum = count_common_answers(&groups);
    println!("Part 2: {}.", sum);
}

fn unique_answers(group: &str) -> HashSet<u8> {
    group
        .split_ascii_whitespace()
        .collect::<String>()
        .bytes()
        .collect::<HashSet<u8>>()
}

fn count_unique_answers(groups: &Vec<&str>) -> usize {
    groups
        .iter()
        .fold(0, |acc, val| acc + unique_answers(val).len())
}

fn count_common_answers(groups: &Vec<&str>) -> usize {
    groups.iter().fold(0, |acc, val| {
        let all = unique_answers(val);

        acc + val
            .lines()
            .fold(all, |set, ans| {
                set.intersection(&ans.bytes().collect::<HashSet<u8>>())
                    .cloned()
                    .collect::<HashSet<_>>()
            })
            .len()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_unique_answers() {
        let groups = Vec::from(["abc", "a\nb\nc", "ab\nac", "a\na\na\na", "b"]);

        assert_eq!(count_unique_answers(&groups), 11)
    }

    #[test]
    fn test_count_common_answers() {
        let groups = Vec::from(["abc", "a\nb\nc", "ab\nac", "a\na\na\na", "b"]);

        assert_eq!(count_common_answers(&groups), 6)
    }
}
