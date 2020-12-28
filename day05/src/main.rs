use std::fs;

#[derive(Debug, PartialEq)]
struct Seat {
    row: u8,
    col: u8,
}

fn main() -> Result<(), String> {
    let contents = fs::read_to_string("input.txt").expect("Could not read file.");
    let entries: Vec<&str> = contents.split("\n").collect();
    let mut seat_ids: Vec<u32> = entries
        .iter()
        .map(|e| decode(e).map(|s| seat_id(&s)))
        .collect::<Result<_, _>>()?;
    seat_ids.sort();

    let max_id = seat_ids.last();
    println!("Part 1: {}.", max_id.ok_or("No result found".to_string())?);
    let seat_id = seat_ids
        .windows(2)
        .find(|&slice| {
            if let &[a, b] = slice {
                a != b - 1
            } else {
                false
            }
        })
        .ok_or("No seat id found.")?[0]
        + 1;
    println!("Part 2: {}.", seat_id);
    Ok(())
}

fn decode(code: &str) -> Result<Seat, String> {
    if code.len() != 10 {
        return Err(format!("Invalid code '{}'.", code));
    }

    let (row_code, col_code) = code.split_at(7);

    let row =
        row_code
            .as_bytes()
            .iter()
            .rev()
            .enumerate()
            .try_fold(0u8, |acc, (idx, el)| match el {
                b'F' => Ok(acc),
                b'B' => Ok(acc | (1u8 << idx)),
                _ => Err(format!("Invalid symbol in row code: {}.", row_code)),
            })?;

    let col =
        col_code
            .as_bytes()
            .iter()
            .rev()
            .enumerate()
            .try_fold(0u8, |acc, (idx, el)| match el {
                b'L' => Ok(acc),
                b'R' => Ok(acc | (1u8 << idx)),
                _ => Err(format!("Invalid symbol in col code: {}.", col_code)),
            })?;

    Ok(Seat { row, col })
}

fn seat_id(seat: &Seat) -> u32 {
    seat.row as u32 * 8 + seat.col as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode() {
        assert_eq!(decode("BFFFBBFRRR"), Ok(Seat { row: 70, col: 7 }));
        assert_eq!(decode("FFFBBBFRRR"), Ok(Seat { row: 14, col: 7 }));
        assert_eq!(decode("BBFFBBFRLL"), Ok(Seat { row: 102, col: 4 }));
    }

    #[test]
    fn test_seat_id() {
        assert_eq!(seat_id(&Seat { row: 70, col: 7 }), 567);
        assert_eq!(seat_id(&Seat { row: 14, col: 7 }), 119);
        assert_eq!(seat_id(&Seat { row: 102, col: 4 }), 820);
    }
}
