use std::collections::HashMap;

fn main() -> Result<(), core::num::ParseIntError> {
    let input = "12,1,16,3,11,0";
    let numbers: Vec<u32> = input.split(',').map(str::parse).collect::<Result<_, _>>()?;

    println!("Part 1: {}.", part1(&numbers, 2020));
    println!("Part 2: {}.", part1(&numbers, 30000000));
    Ok(())
}

fn part1(numbers: &[u32], turns: u32) -> u32 {
    let mut spoken: HashMap<u32, Vec<u32>> = HashMap::new();

    for (turn, n) in numbers.iter().enumerate() {
        spoken.insert(*n, vec![turn as u32 + 1]);
    }

    let mut last = numbers[numbers.len() - 1];

    for turn in numbers.len() as u32 + 1..=turns {
        if let Some(spoken_turns) = spoken.get(&last) {
            let len = spoken_turns.len();

            if len == 1 {
                last = 0;
            } else {
                last = spoken_turns[len - 1] - spoken_turns[len - 2];
            }

            spoken.entry(last).or_insert(Vec::new()).push(turn);
        }
    }

    println!("{:?}", spoken);
    last
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&[0, 3, 6], 10), 0);
        assert_eq!(part1(&[1, 3, 2], 2020), 1);
        assert_eq!(part1(&[2, 1, 3], 2020), 10);
        assert_eq!(part1(&[1, 2, 3], 2020), 27);
        assert_eq!(part1(&[2, 3, 1], 2020), 78);
        assert_eq!(part1(&[3, 2, 1], 2020), 438);
        assert_eq!(part1(&[3, 1, 2], 2020), 1836);
    }
}
