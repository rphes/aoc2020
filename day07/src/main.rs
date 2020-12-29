use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

type Triplet = (String, u32, String);

fn main() -> Result<(), String> {
    let contents = fs::read_to_string("input.txt").expect("Could not read file.");
    let lines: Vec<&str> = contents.lines().collect();

    let triplets: Vec<Triplet> = lines
        .iter()
        .map(parse_line)
        .collect::<Result<Vec<Vec<Triplet>>, _>>()?
        .into_iter()
        .flatten()
        .collect();

    println!("Part 1: {}.", count1(&triplets));
    println!("Part 2: {}.", count2(&triplets));

    Ok(())
}

fn parse_line(line: &&str) -> Result<Vec<Triplet>, String> {
    match line.splitn(2, " bags contain ").collect::<Vec<&str>>()[..] {
        [_, "no other bags."] => Ok(Vec::new()),
        [container, rest] => Ok(rest
            .split(", ")
            .map(|x| {
                if let [n, a, b, _] = x.split(' ').collect::<Vec<_>>()[..] {
                    if let Ok(amount) = n.parse::<u32>() {
                        Ok((container.to_string(), amount, format!("{} {}", a, b)))
                    } else {
                        Err(format!(
                            "Could not parse number of bags in line \"{}\".",
                            line
                        ))
                    }
                } else {
                    Err(format!("Could not parse line \"{}\"", line))
                }
            })
            .collect::<Result<Vec<_>, String>>()?),
        _ => Err(format!("Could not parse line \"{}\"", line)),
    }
}

fn count1(triplets: &Vec<Triplet>) -> usize {
    let mut map = HashMap::new();

    for (value, _, key) in triplets {
        let items = map.entry(key).or_insert(Vec::new());
        items.push(value);
    }

    let mut visited = HashSet::new();
    let start = "shiny gold".to_string();
    let mut to_visit = Vec::from([&start]);

    loop {
        to_visit = to_visit
            .iter()
            .flat_map(|k| map.remove(k).unwrap_or(Vec::new()))
            .collect();

        if to_visit.is_empty() {
            break;
        }

        visited.extend(to_visit.iter().cloned());
    }

    visited.len()
}

fn count2(triplets: &Vec<Triplet>) -> u32 {
    let mut map = HashMap::new();

    for (container, amount, contained) in triplets {
        let items = map.entry(container).or_insert(Vec::new());
        items.push((*amount, contained));
    }

    let start = "shiny gold".to_string();
    let default = Vec::new();
    let mut count = 0;
    let mut to_visit = Vec::from([(1, &start)]);

    loop {
        to_visit = to_visit
            .iter()
            .flat_map(|(amt, container)| {
                map.get(container)
                    .unwrap_or(&default)
                    .iter()
                    .map(|(amt2, contained)| {
                        let add = amt * amt2;
                        count += add;

                        (add, *contained)
                    })
                    .collect::<Vec<(u32, &String)>>()
            })
            .collect();

        if to_visit.is_empty() {
            return count;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn triplets() -> Vec<Triplet> {
        [
            ("light red", 1, "bright white"),
            ("light red", 2, "muted yellow"),
            ("dark orange", 3, "bright white"),
            ("dark orange", 4, "muted yellow"),
            ("bright white", 1, "shiny gold"),
            ("muted yellow", 2, "shiny gold"),
            ("muted yellow", 9, "faded plum"),
            ("shiny gold", 1, "dark olive"),
            ("shiny gold", 2, "vibrant plum"),
            ("dark olive", 3, "faded blue"),
            ("dark olive", 4, "dotted black"),
            ("vibrant plum", 5, "faded blue"),
            ("vibrant plum", 6, "dotted black"),
        ]
        .iter()
        .map(|&(a, b, c)| (a.to_string(), b, c.to_string()))
        .collect()
    }

    #[test]
    fn test_parse_line() -> Result<(), String> {
        assert_eq!(
            parse_line(&"light red bags contain 1 bright white bag, 2 muted yellow bags.")?,
            Vec::from([
                ("light red".to_string(), 1, "bright white".to_string()),
                ("light red".to_string(), 2, "muted yellow".to_string())
            ])
        );

        assert_eq!(
            parse_line(&"bright white bags contain 1 shiny gold bag.")?,
            Vec::from([("bright white".to_string(), 1, "shiny gold".to_string())])
        );

        assert_eq!(
            parse_line(&"faded blue bags contain no other bags.")?,
            Vec::new()
        );

        Ok(())
    }

    #[test]
    fn test_count1() {
        assert_eq!(count1(&triplets()), 4);
    }

    #[test]
    fn test_count2() {
        assert_eq!(count2(&triplets()), 32);
    }
}
