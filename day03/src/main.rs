use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Could not read file.");
    let lines = contents.split('\n').collect();

    println!("Part 1: {}.", count_trees(&lines, 3, 1));

    let prod = count_trees(&lines, 1, 1)
        * count_trees(&lines, 3, 1)
        * count_trees(&lines, 5, 1)
        * count_trees(&lines, 7, 1)
        * count_trees(&lines, 1, 2);

    println!("Part 2: {}.", prod);
}

fn count_trees(lines: &Vec<&str>, x_step: usize, y_step: usize) -> u32 {
    let mut x = 0;
    let mut trees = 0;
    let tree: u8 = '#' as u8;
    let mut stepped = lines.iter().step_by(y_step);
    let width: usize = stepped.next().unwrap_or(&"").len();

    for line in stepped {
        x += x_step;

        if line.as_bytes()[x % width] == tree {
            trees += 1;
        }
    }

    trees
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_trees() {
        let lines: Vec<&str> = "\
..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#"
            .split('\n')
            .collect();

        assert_eq!(count_trees(&lines, 1, 1), 2);
        assert_eq!(count_trees(&lines, 3, 1), 7);
        assert_eq!(count_trees(&lines, 5, 1), 3);
        assert_eq!(count_trees(&lines, 7, 1), 4);
        assert_eq!(count_trees(&lines, 1, 2), 2);
    }
}
