use std::collections::HashMap;
use std::fs;
use std::iter::Iterator;
use std::time::Instant;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Could not read file.");
    let (tiles, edges) = parse_input(&contents).expect("Could not parse tiles");
    let p1 = Instant::now();
    let neighbor_map = find_neighbors(&edges);

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

fn parse_input(contents: &str) -> Option<(TileMap, EdgeMap)> {
    let data = contents
        .split("\n\n")
        .map(parse_tile)
        .collect::<Option<Vec<_>>>()?;

    Some(
        data.into_iter()
            .map(|(id, data, edges)| ((id, data), (id, edges)))
            .unzip(),
    )
}

fn parse_tile(string: &str) -> Option<(usize, Vec<String>, Edges)> {
    let (header, tile) = string.split_once('\n')?;
    let id: usize = header
        .strip_prefix("Tile ")?
        .strip_suffix(":")?
        .parse()
        .ok()?;

    let lines: Vec<&str> = tile.lines().collect();
    let top = lines.get(0)?.to_string();
    let bottom = lines.last()?.chars().rev().collect::<String>();
    let left = lines
        .iter()
        .rev()
        .map(|x| x.chars().nth(0))
        .collect::<Option<_>>()?;
    let right = lines
        .iter()
        .map(|x| x.chars().last())
        .collect::<Option<_>>()?;

    return Some((
        id,
        lines.iter().map(|s| s.to_string()).collect(),
        [top, right, bottom, left],
    ));
}

type Neighbors = HashMap<usize, [Option<(usize, usize, bool)>; 4]>;

fn find_neighbors(edge_map: &EdgeMap) -> Neighbors {
    let mut neighbor_map: Neighbors = edge_map.keys().map(|k| (*k, [None; 4])).collect();

    for (id, edges) in edge_map {
        if neighbor_map[id].iter().filter(|e| e.is_some()).count() == 4 {
            continue;
        }

        for (id2, edges2) in edge_map {
            if id != id2 {
                for (idx, edge) in edges.iter().enumerate() {
                    if neighbor_map[id][idx].is_some() {
                        continue;
                    }

                    for (idx2, edge2) in edges2.iter().enumerate() {
                        let reverse = edge == edge2;
                        if edge == &edge2.chars().rev().collect::<String>() || reverse {
                            neighbor_map.get_mut(id2).unwrap()[idx2] = Some((*id, idx, reverse));
                            neighbor_map.get_mut(id).unwrap()[idx] = Some((*id2, idx2, reverse));
                        }
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
        .filter_map(|(k, v)| {
            if v.iter().filter(|e| e.is_some()).count() == 2 {
                Some(*k)
            } else {
                None
            }
        })
        .product()
}

// fn transpose(tile: Vec<&str>) -> Vec<&str> {}

// fn rotate(tile: Vec<&str>) -> Vec<&str> {
//     vec![""]
// }

fn part2(neighbor_map: &Neighbors, tile_map: &TileMap) -> usize {
    let mut edge = 2i8;
    let mut mirror_x = false;
    let mut mirror_y = false;
    let mut neighbors = neighbor_map
        .values()
        .find(|v| v[0].is_none() && v[1].is_some() && v[2].is_some() && v[3].is_none())
        .unwrap();

    loop {
        let mut neighbors2 = neighbors;
        let mut edge2 = (edge - 1).rem_euclid(4);

        while let Some(right_neighbor) = neighbors2[edge2 as usize] {
            let (id, local_edge_num, local_reverse) = right_neighbor;

            if local_reverse {
                if local_edge_num % 2 == 0 {
                    mirror_x = mirror_x ^ local_reverse;
                } else {
                    mirror_y = mirror_y ^ local_reverse;
                }
            }

            edge2 = (edge2 + (3 - local_edge_num as i8)).rem_euclid(4);
            neighbors2 = neighbor_map.get(&id).unwrap();
            println!("{:?} {:?} {:?}", id, local_edge_num, edge2);
        }
        println!("");

        if let Some(bottom_neighbor) = neighbors[edge as usize] {
            let (id, local_edge_num, local_reverse) = bottom_neighbor;
            edge = (edge + (0 - local_edge_num as i8)).rem_euclid(4);
            neighbors = neighbor_map.get(&id).unwrap();
            println!("{:?} {:?} {:?}", id, local_edge_num, edge);
        } else {
            break;
        }
    }

    println!("{:?}", neighbors);

    0
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

    #[test]
    fn test_parse_input() {
        let edge_map = parse_input(TEST_INPUT).unwrap().1;
        assert_eq!(
            edge_map.get(&2311),
            Some(&[
                "..##.#..#.".to_string(),
                "...#.##..#".to_string(),
                "###..###..".to_string(),
                ".#..#####.".to_string()
            ])
        );
        assert_eq!(
            edge_map.get(&1951),
            Some(&[
                "#.##...##.".to_string(),
                ".#####..#.".to_string(),
                "..#.##...#".to_string(),
                "#..#..#.##".to_string()
            ])
        );
        assert_eq!(
            edge_map.get(&1171),
            Some(&[
                "####...##.".to_string(),
                ".#..#.....".to_string(),
                "...##.....".to_string(),
                ".##....###".to_string()
            ])
        );
        assert_eq!(
            edge_map.get(&1427),
            Some(&[
                "###.##.#..".to_string(),
                "..###.#.#.".to_string(),
                ".#..#.##..".to_string(),
                "......#..#".to_string()
            ])
        );
        assert_eq!(
            edge_map.get(&1489),
            Some(&[
                "##.#.#....".to_string(),
                ".....#..#.".to_string(),
                "..#.##.###".to_string(),
                "#.#.##...#".to_string()
            ])
        );
        assert_eq!(
            edge_map.get(&2473),
            Some(&[
                "#....####.".to_string(),
                "...###.#..".to_string(),
                ".#.#.###..".to_string(),
                ".##...####".to_string()
            ])
        );
        assert_eq!(
            edge_map.get(&2971),
            Some(&[
                "..#.#....#".to_string(),
                "#...##.#.#".to_string(),
                "#.#.#.#...".to_string(),
                "...#..###.".to_string()
            ])
        );
        assert_eq!(
            edge_map.get(&2729),
            Some(&[
                "...#.#.#.#".to_string(),
                "#..#......".to_string(),
                ".##...##.#".to_string(),
                "####....#.".to_string()
            ])
        );
        assert_eq!(
            edge_map.get(&3079),
            Some(&[
                "#.#.#####.".to_string(),
                ".#....#...".to_string(),
                "...###.#..".to_string(),
                "...#.##..#".to_string()
            ])
        );
    }

    #[test]
    fn test_find_neighbors() {
        let tiles = parse_input(TEST_INPUT).unwrap().1;
        let neighbor_map = find_neighbors(&tiles);

        assert_eq!(
            neighbor_map.get(&1951),
            Some(&[Some((2729, 2, false)), Some((2311, 3, false)), None, None])
        );
        assert_eq!(
            neighbor_map.get(&1427),
            Some(&[
                Some((1489, 2, false)),
                Some((2473, 2, false)),
                Some((2311, 0, false)),
                Some((2729, 1, false))
            ])
        );
    }

    #[test]
    fn test_part1() {
        let edges = parse_input(TEST_INPUT).unwrap().1;
        let neighbor_map = find_neighbors(&edges);
        assert_eq!(part1(&neighbor_map), 20899048083289);
    }

    #[test]
    fn test_part2() {
        let (tile_map, edge_map) = parse_input(TEST_INPUT).unwrap();
        let neighbor_map = find_neighbors(&edge_map);
        assert_eq!(part2(&neighbor_map, &tile_map), 273);
    }
}
