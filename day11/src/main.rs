use std::fs;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Position {
    Empty,
    Occupied,
    Floor,
}

type Grid = Vec<Vec<Position>>;

fn main() -> Result<(), std::num::ParseIntError> {
    let contents = fs::read_to_string("input.txt").expect("Could not read file.");
    let mut grid = parse_grid(&contents);

    println!("Part 1: {}.", run(&mut grid));
    let mut grid = parse_grid(&contents);
    println!("Part 2: {}.", run2(&mut grid));
    Ok(())
}

fn parse_grid(input: &str) -> Grid {
    let mut iter = input.lines();
    let width = iter.next().unwrap_or("").len();
    let height = iter.count() + 1;
    let mut grid = Vec::with_capacity(height);

    for (y, line) in input.lines().enumerate() {
        grid.push(Vec::with_capacity(width));

        for (x, chr) in line.bytes().enumerate() {
            grid[y].push(match chr {
                b'L' => Position::Empty,
                b'.' => Position::Floor,
                _ => panic!("Invalid input '{}' on line {}.", chr, x),
            })
        }
    }

    grid
}

fn count_occupied(grid: &Grid) -> usize {
    grid.iter()
        .flat_map(|r| r.iter())
        .filter(|val| **val == Position::Occupied)
        .count()
}

fn get_neighbors(x: usize, y: usize, width: usize, height: usize) -> Vec<(usize, usize)> {
    let mut neighbors = Vec::new();

    for i in x.saturating_sub(1)..(x + 2).min(width) {
        for j in y.saturating_sub(1)..(y + 2).min(height) {
            if (i, j) != (x, y) {
                neighbors.push((i, j));
            }
        }
    }

    neighbors
}

fn update_grid(grid: &mut Grid) -> bool {
    let width = grid[0].len();
    let height = grid.len();
    let mut changed = false;
    let mut new_grid = grid.clone();

    for y in 0..height {
        for x in 0..width {
            let val = grid[y][x];
            let n = get_neighbors(x, y, width, height)
                .iter()
                .filter(|(x2, y2)| grid[*y2][*x2] == Position::Occupied)
                .count();

            if val == Position::Empty && n == 0 {
                new_grid[y][x] = Position::Occupied;
                changed = true;
            } else if val == Position::Occupied && n >= 4 {
                new_grid[y][x] = Position::Empty;
                changed = true;
            }
        }
    }

    std::mem::swap(grid, &mut new_grid);
    changed
}

fn run(grid: &mut Grid) -> usize {
    while update_grid(grid) {}
    count_occupied(grid)
}

fn update_grid2(grid: &mut Grid) -> bool {
    let width = grid[0].len() as i32;
    let height = grid.len() as i32;
    let directions = [
        (0, 1),
        (1, 1),
        (1, 0),
        (1, -1),
        (0, -1),
        (-1, -1),
        (-1, 0),
        (-1, 1),
    ];
    let mut changed = false;
    let mut new_grid = grid.clone();

    for y in 0..height {
        for x in 0..width {
            let mut occupied = 0;

            for (x_off, y_off) in &directions {
                let mut x_neig = x + x_off;
                let mut y_neig = y + y_off;
                let mut radius = 1;

                while x_neig >= 0 && x_neig < width && y_neig >= 0 && y_neig < height {
                    match grid[y_neig as usize][x_neig as usize] {
                        Position::Empty => {
                            break;
                        }
                        Position::Occupied => {
                            occupied += 1;
                            break;
                        }
                        _ => {
                            radius += 1;
                            x_neig = x + radius * x_off;
                            y_neig = y + radius * y_off;
                        }
                    }
                }
            }

            if grid[y as usize][x as usize] == Position::Empty && occupied == 0 {
                new_grid[y as usize][x as usize] = Position::Occupied;
                changed = true;
            } else if grid[y as usize][x as usize] == Position::Occupied && occupied >= 5 {
                new_grid[y as usize][x as usize] = Position::Empty;
                changed = true;
            }
        }
    }

    std::mem::swap(grid, &mut new_grid);
    changed
}

fn run2(grid: &mut Grid) -> usize {
    while update_grid2(grid) {}
    count_occupied(grid)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> &'static str {
        "\
L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL"
    }

    #[test]
    fn test_parse_grid() {
        let parsed = parse_grid(get_input());
        assert_eq!(parsed.len(), 10);
        assert_eq!(parsed[0].len(), 10);
        assert_eq!(
            parsed
                .iter()
                .flat_map(|r| r.iter())
                .filter(|&x| x == &Position::Empty)
                .count(),
            71
        );
    }

    #[test]
    fn test_run() {
        let mut grid = parse_grid(get_input());
        assert_eq!(run(&mut grid), 37);
    }

    #[test]
    fn test_run2() {
        let mut grid = parse_grid(get_input());
        assert_eq!(run2(&mut grid), 26);
    }
}
