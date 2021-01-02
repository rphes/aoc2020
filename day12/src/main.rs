use std::fs;

mod part1 {
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum Direction {
        North = 0,
        East,
        South,
        West,
    }
    use Direction::*;
    impl Direction {
        fn from_i32(value: i32) -> Direction {
            match value {
                0 => North,
                1 => East,
                2 => South,
                3 => West,
                _ => panic!("Unknown value: {}", value),
            }
        }
    }

    #[derive(Debug, PartialEq)]
    pub struct Ship {
        pub x: i32,
        pub y: i32,
        pub d: Direction,
    }
    impl Ship {
        fn sail(&mut self, direction: Direction, value: i32) {
            match direction {
                North => self.y += value,
                East => self.x += value,
                South => self.y -= value,
                West => self.x -= value,
            };
        }
    }
    fn step(ship: &mut Ship, line: &str) {
        let (action, value) = line.split_at(1);
        let value = value
            .parse::<i32>()
            .expect(&format!("Could not parse line \"{}\"", line));

        match action.as_bytes()[0] {
            b'N' => ship.sail(North, value),
            b'E' => ship.sail(East, value),
            b'S' => ship.sail(South, value),
            b'W' => ship.sail(West, value),
            b'L' => ship.d = Direction::from_i32((ship.d as i32 + 4 - value / 90) % 4) as Direction,
            b'R' => ship.d = Direction::from_i32((ship.d as i32 + value / 90) % 4),
            b'F' => ship.sail(ship.d, value),
            _ => panic!("Invalid input on line \"{}\".", line),
        }
    }

    pub fn run(input: &str) -> Ship {
        let mut ship = Ship {
            x: 0,
            y: 0,
            d: East,
        };

        for line in input.lines() {
            step(&mut ship, line);
        }

        ship
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_part1() {
            let input = "F10\nN3\nF7\nR90\nF11";
            assert_eq!(
                run(input),
                Ship {
                    x: 17,
                    y: -8,
                    d: South
                }
            )
        }
    }
}

mod part2 {
    #[derive(Debug, PartialEq)]
    pub struct Point {
        pub x: i32,
        pub y: i32,
    }
    impl Point {
        fn rotate(&mut self, deg: i32) {
            let (sin, cos) = (deg as f32 / 180.0 * core::f32::consts::PI).sin_cos();
            let (sin, cos) = (sin as i32, cos as i32);

            let x = self.x * cos - self.y * sin;
            let y = self.x * sin + self.y * cos;
            self.x = x;
            self.y = y;
        }
    }

    #[derive(Debug, PartialEq)]
    pub struct Ship {
        pub pos: Point,
        pub waypoint: Point,
    }
    impl Ship {
        fn sail(&mut self, n: i32) {
            self.pos.x += self.waypoint.x * n;
            self.pos.y += self.waypoint.y * n;
        }
    }

    fn step(ship: &mut Ship, line: &str) {
        let (action, value) = line.split_at(1);
        let value = value
            .parse::<i32>()
            .expect(&format!("Could not parse line \"{}\"", line));

        match action.as_bytes()[0] {
            b'N' => ship.waypoint.y += value,
            b'E' => ship.waypoint.x += value,
            b'S' => ship.waypoint.y -= value,
            b'W' => ship.waypoint.x -= value,
            b'L' => ship.waypoint.rotate(value),
            b'R' => ship.waypoint.rotate(-value),
            b'F' => ship.sail(value),
            _ => panic!("Invalid input on line \"{}\".", line),
        }

        println!("{:?}", ship);
    }

    pub fn run(input: &str) -> Ship {
        let mut ship = Ship {
            pos: Point { x: 0, y: 0 },
            waypoint: Point { x: 10, y: 1 },
        };

        for line in input.lines() {
            step(&mut ship, line);
        }

        ship
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_part2() {
            let input = "F10\nN3\nF7\nR90\nF11";
            assert_eq!(
                run(input),
                Ship {
                    pos: Point { x: 214, y: -72 },
                    waypoint: Point { x: 4, y: -10 },
                }
            )
        }
    }
}

use part1::run as part1;
use part2::run as part2;
fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file.");

    let ship = part1(&input);
    println!("Part 1: {}.", ship.x.abs() + ship.y.abs());
    let ship = part2(&input);
    println!("Part 2: {}.", ship.pos.x.abs() + ship.pos.y.abs());
}
