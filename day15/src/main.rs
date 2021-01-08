use std::time::Instant;

fn main() -> Result<(), core::num::ParseIntError> {
    let input = "12,1,16,3,11,0";
    let numbers: Vec<u32> = input.split(',').map(str::parse).collect::<Result<_, _>>()?;

    let p1 = Instant::now();
    println!("Part 1: {} ({:?}).", part1(&numbers, 2020), p1.elapsed());
    let p2 = Instant::now();
    println!(
        "Part 2: {} ({:?}).",
        part1(&numbers, 30_000_000),
        p2.elapsed()
    );
    Ok(())
}

fn part1(numbers: &[u32], turns: u32) -> u32 {
    let mut spoken = vec![None; turns as usize];

    for (turn, n) in numbers.iter().enumerate() {
        spoken[*n as usize] = Some(turn as u32 + 1);
    }

    let mut last = numbers[numbers.len() - 1];

    for turn in numbers.len() as u32..turns {
        let new_last = match spoken[last as usize] {
            None => 0,
            Some(prev_turn) => turn - prev_turn,
        };
        spoken[last as usize] = Some(turn);
        last = new_last
    }

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
