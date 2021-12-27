use std::collections::HashMap;
use std::fmt;
use std::fs;
use std::iter::Iterator;
use std::time::Instant;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Could not read file.");
    let tiles = parse_input(&contents).expect("Could not parse tiles");
    let p1 = Instant::now();
    let neighbor_map = find_neighbors(&tiles);

    println!("Part 1: {} ({:?}).", part1(&neighbor_map), p1.elapsed());
    // let p2 = Instant::now();
    // println!(
    //     "Part 2: {} ({:?}).",
    //     part2(&neighbor_map, &tiles),
    //     p2.elapsed()
    // );
}

type Edges = [String; 4];
type EdgeMap = HashMap<usize, Edges>;
type TileMap = HashMap<usize, Vec<String>>;
const TILE_SIZE: usize = 10;

#[derive(PartialEq, Copy, Clone)]
struct Edge(u16);

impl fmt::Debug for Edge {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let text: String = (0..TILE_SIZE)
            .map(|i| if self.0 & (1 << i) != 0 { '#' } else { '.' })
            .collect();
        write!(f, "{}", text)
    }
}

impl Edge {
    pub fn reverse(&self) -> Self {
        Self(self.0.reverse_bits().rotate_left(TILE_SIZE as u32))
    }
}

struct Tile {
    id: usize,
    rows: Vec<Edge>,
}

impl Tile {
    pub fn left_edge(&self) -> Edge {
        return self.get_col(0);
    }

    pub fn right_edge(&self) -> Edge {
        return self.get_col(TILE_SIZE - 1);
    }

    pub fn top_edge(&self) -> Edge {
        self.rows[0]
    }

    pub fn bottom_edge(&self) -> Edge {
        self.rows[TILE_SIZE - 1]
    }

    pub fn edges(&self) -> [Edge; 4] {
        [
            self.top_edge(),
            self.right_edge(),
            self.bottom_edge(),
            self.left_edge(),
        ]
    }

    pub fn get_row(&self, row_idx: usize) -> Edge {
        self.rows[row_idx as usize]
    }

    pub fn get_col(&self, col_idx: usize) -> Edge {
        return Edge(self.rows.iter().enumerate().fold(0, |col, (row_idx, row)| {
            col | (row.0 & 1 << col_idx as u16) >> col_idx << row_idx
        }));
    }

    pub fn rotate(&self) -> Self {
        return Self {
            id: self.id,
            rows: (0..TILE_SIZE)
                .map(|col| self.get_col(col).reverse())
                .collect(),
        };
    }
}

impl fmt::Debug for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in &self.rows {
            write!(f, "{:?}\n", row);
        }
        Ok(())
    }
}

fn parse_input(contents: &str) -> Option<Vec<Tile>> {
    contents
        .split("\n\n")
        .map(parse_tile)
        .collect::<Option<Vec<_>>>()
}

fn parse_tile(string: &str) -> Option<Tile> {
    let (header, tile) = string.split_once('\n')?;
    let id: usize = header
        .strip_prefix("Tile ")?
        .strip_suffix(":")?
        .parse()
        .ok()?;

    let lines: Vec<&str> = tile.lines().collect();

    return Some(Tile {
        id,
        rows: lines
            .iter()
            .map(|line| {
                Edge(line.chars().enumerate().fold(
                    0,
                    |row, (i, c)| {
                        if c == '#' {
                            row | (1 << i)
                        } else {
                            row
                        }
                    },
                ))
            })
            .collect(),
    });
}

type Neighbors = HashMap<usize, Vec<usize>>;

fn find_neighbors(tiles: &[Tile]) -> Neighbors {
    let mut neighbor_map: Neighbors = tiles
        .iter()
        .map(|tile| (tile.id, Vec::with_capacity(4)))
        .collect();

    for tile in tiles {
        for tile2 in tiles {
            if tile.id == tile2.id {
                continue;
            }

            for edge in tile.edges().iter() {
                for edge2 in tile2.edges().iter() {
                    if edge == edge2 || *edge == edge2.reverse() {
                        neighbor_map.get_mut(&tile.id).unwrap().push(tile2.id)
                    }
                }
            }
        }
    }
    neighbor_map
}

fn part1(neighbor_map: &Neighbors) -> usize {
    neighbor_map
        .iter()
        .filter_map(|(k, v)| if v.len() == 2 { Some(*k) } else { None })
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "Tile 2311:
..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###

Tile 1951:
#.##...##.
#.####...#
.....#..##
#...######
.##.#....#
.###.#####
###.##.##.
.###....#.
..#.#..#.#
#...##.#..

Tile 1171:
####...##.
#..##.#..#
##.#..#.#.
.###.####.
..###.####
.##....##.
.#...####.
#.##.####.
####..#...
.....##...

Tile 1427:
###.##.#..
.#..#.##..
.#.##.#..#
#.#.#.##.#
....#...##
...##..##.
...#.#####
.#.####.#.
..#..###.#
..##.#..#.

Tile 1489:
##.#.#....
..##...#..
.##..##...
..#...#...
#####...#.
#..#.#.#.#
...#.#.#..
##.#...##.
..##.##.##
###.##.#..

Tile 2473:
#....####.
#..#.##...
#.##..#...
######.#.#
.#...#.#.#
.#########
.###.#..#.
########.#
##...##.#.
..###.#.#.

Tile 2971:
..#.#....#
#...###...
#.#.###...
##.##..#..
.#####..##
.#..####.#
#..#.#..#.
..####.###
..#.#.###.
...#.#.#.#

Tile 2729:
...#.#.#.#
####.#....
..#.#.....
....#..#.#
.##..##.#.
.#.####...
####.#.#..
##.####...
##..#.##..
#.##...##.

Tile 3079:
#.#.#####.
.#..######
..#.......
######....
####.#..#.
.#...#.##.
#.#####.##
..#.###...
..#.......
..#.###...";

    //     #[test]
    //     fn test_parse_input() {
    //         let edge_map = parse_input(TEST_INPUT).unwrap().1;
    //         assert_eq!(
    //             edge_map.get(&2311),
    //             Some(&[
    //                 "..##.#..#.".to_string(),
    //                 "...#.##..#".to_string(),
    //                 "###..###..".to_string(),
    //                 ".#..#####.".to_string()
    //             ])
    //         );
    //         assert_eq!(
    //             edge_map.get(&1951),
    //             Some(&[
    //                 "#.##...##.".to_string(),
    //                 ".#####..#.".to_string(),
    //                 "..#.##...#".to_string(),
    //                 "#..#..#.##".to_string()
    //             ])
    //         );
    //         assert_eq!(
    //             edge_map.get(&1171),
    //             Some(&[
    //                 "####...##.".to_string(),
    //                 ".#..#.....".to_string(),
    //                 "...##.....".to_string(),
    //                 ".##....###".to_string()
    //             ])
    //         );
    //         assert_eq!(
    //             edge_map.get(&1427),
    //             Some(&[
    //                 "###.##.#..".to_string(),
    //                 "..###.#.#.".to_string(),
    //                 ".#..#.##..".to_string(),
    //                 "......#..#".to_string()
    //             ])
    //         );
    //         assert_eq!(
    //             edge_map.get(&1489),
    //             Some(&[
    //                 "##.#.#....".to_string(),
    //                 ".....#..#.".to_string(),
    //                 "..#.##.###".to_string(),
    //                 "#.#.##...#".to_string()
    //             ])
    //         );
    //         assert_eq!(
    //             edge_map.get(&2473),
    //             Some(&[
    //                 "#....####.".to_string(),
    //                 "...###.#..".to_string(),
    //                 ".#.#.###..".to_string(),
    //                 ".##...####".to_string()
    //             ])
    //         );
    //         assert_eq!(
    //             edge_map.get(&2971),
    //             Some(&[
    //                 "..#.#....#".to_string(),
    //                 "#...##.#.#".to_string(),
    //                 "#.#.#.#...".to_string(),
    //                 "...#..###.".to_string()
    //             ])
    //         );
    //         assert_eq!(
    //             edge_map.get(&2729),
    //             Some(&[
    //                 "...#.#.#.#".to_string(),
    //                 "#..#......".to_string(),
    //                 ".##...##.#".to_string(),
    //                 "####....#.".to_string()
    //             ])
    //         );
    //         assert_eq!(
    //             edge_map.get(&3079),
    //             Some(&[
    //                 "#.#.#####.".to_string(),
    //                 ".#....#...".to_string(),
    //                 "...###.#..".to_string(),
    //                 "...#.##..#".to_string()
    //             ])
    //         );
    //     }

    //     #[test]
    //     fn test_find_neighbors() {
    //         let tiles = parse_input(TEST_INPUT).unwrap().1;
    //         let neighbor_map = find_neighbors(&tiles);

    //         assert_eq!(
    //             neighbor_map.get(&1951),
    //             Some(&[Some((2729, 2, false)), Some((2311, 3, false)), None, None])
    //         );
    //         assert_eq!(
    //             neighbor_map.get(&1427),
    //             Some(&[
    //                 Some((1489, 2, false)),
    //                 Some((2473, 2, false)),
    //                 Some((2311, 0, false)),
    //                 Some((2729, 1, false))
    //             ])
    //         );
    //     }

    #[test]
    fn test_part1() {
        let tiles = parse_input(TEST_INPUT).unwrap();
        let neighbor_map = find_neighbors(&tiles);
        assert_eq!(part1(&neighbor_map), 20899048083289);
    }
}
