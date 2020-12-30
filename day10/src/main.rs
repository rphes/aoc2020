use std::fs;

fn main() -> Result<(), std::num::ParseIntError> {
    let contents = fs::read_to_string("input.txt").expect("Could not read file.");
    let numbers = contents
        .lines()
        .map(|l| l.parse::<u32>())
        .collect::<Result<Vec<_>, _>>()?;

    println!("Part 1: {}.", part1(&numbers));
    println!("Part 2: {}.", part2(&numbers));
    Ok(())
}

fn part1(numbers: &Vec<u32>) -> u32 {
    let numbers = &mut numbers.clone()[..];
    numbers.sort_unstable();

    let (one, three) =
        numbers
            .windows(2)
            .fold((1, 1), |(one, three), window| match window[1] - window[0] {
                1 => (one + 1, three),
                3 => (one, three + 1),
                _ => (one, three),
            });

    one * three
}

fn part2(numbers: &Vec<u32>) -> u64 {
    let numbers = &mut numbers.clone();
    numbers.push(0);
    numbers.push(*numbers.iter().max().unwrap() + 3);
    numbers.sort_unstable();

    let mut acc = 1;
    let mut i = 0;
    let mut j;

    while i < numbers.len() {
        j = i;
        while j < numbers.len() {
            let n = (j - i) as u32;
            if numbers[j] - numbers[i] > n {
                if n > 2 && n <= 4 {
                    acc *= 2u64.pow(n - 2);
                } else if n > 4 {
                    acc *= 2u64.pow(n - 2) - ((n - 4) * (n - 4 + 1) / 2) as u64;
                }
                break;
            }
            j += 1;
        }

        i = j
    }

    acc
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];
        assert_eq!(part1(&input), 35);

        let input = vec![
            28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35,
            8, 17, 7, 9, 4, 2, 34, 10, 3,
        ];
        assert_eq!(part1(&input), 220);
    }

    #[test]
    fn test_part2() {
        let input = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];
        assert_eq!(part2(&input), 8);

        let input = vec![
            28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35,
            8, 17, 7, 9, 4, 2, 34, 10, 3,
        ];
        assert_eq!(part2(&input), 19208);
    }
}
