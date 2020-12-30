use std::fs;

fn main() -> Result<(), std::num::ParseIntError> {
    let contents = fs::read_to_string("input.txt").expect("Could not read file.");
    let numbers = contents
        .lines()
        .map(|l| l.parse::<u64>())
        .collect::<Result<Vec<_>, _>>()?;

    let invalid = scan(&numbers, 25).expect("No result found for part 1.");
    println!("Part 1: {}.", invalid);
    println!(
        "Part 2: {}.",
        scan2(&numbers, invalid).expect("No result found for part 2.")
    );
    Ok(())
}

fn scan(numbers: &Vec<u64>, preamble_len: usize) -> Option<u64> {
    for window in numbers[..].windows(preamble_len + 1) {
        let (last, rest) = window.split_last().unwrap();

        if !compare(&rest, *last) {
            return Some(*last);
        }
    }

    None
}

fn compare(xs: &[u64], n: u64) -> bool {
    for x in xs {
        for y in xs {
            if x + y == n {
                return true;
            }
        }
    }

    false
}

fn scan2(numbers: &Vec<u64>, invalid: u64) -> Option<u64> {
    for i in 0..numbers.len() {
        let mut sum = 0;

        for (j, n) in numbers[i..].iter().enumerate() {
            let j = i + j;
            sum += n;

            if sum > invalid {
                break;
            }

            if sum == invalid {
                if let Some(max) = numbers[i..j + 1].iter().max() {
                    if let Some(min) = numbers[i..j + 1].iter().min() {
                        return Some(max + min);
                    }
                }

                return None;
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scan() {
        let input = vec![
            35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309,
            576,
        ];

        assert_eq!(scan(&input, 5), Some(127));
    }

    #[test]
    fn test_scan2() {
        let input = vec![
            35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309,
            576,
        ];

        assert_eq!(scan2(&input, 127), Some(62));
    }
}
