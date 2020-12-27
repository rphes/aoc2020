use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("File not found.");

    let list: Option<Vec<_>> = contents
        .split('\n')
        .map(|s| s.parse::<u32>().ok())
        .collect();

    if let Some(list) = list {
        if let Some(product) = find_product2(&list[..], 2020) {
            println!("Result for part 1 is {}.", product)
        } else {
            println!("No result for part 1.")
        }

        if let Some(product) = find_product3(&list[..], 2020) {
            println!("Result for part 2 is {}.", product)
        } else {
            println!("No result for part 2.")
        }
    } else {
        println!("Could not parse input")
    }
}

fn find_product2(list: &[u32], sum: u32) -> Option<u32> {
    for a in list {
        for b in list {
            if a + b == sum {
                return Some(a * b);
            }
        }
    }

    None
}

fn find_product3(list: &[u32], sum: u32) -> Option<u32> {
    for a in list {
        for b in list {
            for c in list {
                if a + b + c == sum {
                    return Some(a * b * c);
                }
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_some() {
        let list = [1721, 979, 366, 299, 675, 1456];
        assert_eq!(find_product2(&list, 2020), Some(514579))
    }

    #[test]
    fn test_none() {
        let list = [1721, 979, 366, 500, 675, 1456];
        assert_eq!(find_product2(&list, 2020), None)
    }
}
