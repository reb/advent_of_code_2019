/// --- Day 3: Crossed Wires ---
///
/// The gravity assist was successful, and you're well on your way to the Venus
/// refuelling station. During the rush back on Earth, the fuel management
/// system wasn't completely installed, so that's next on the priority list.
///
/// Opening the front panel reveals a jumble of wires. Specifically, two wires
/// are connected to a central port and extend outward on a grid. You trace the
/// path each wire takes as it leaves the central port, one wire per line of
/// text (your puzzle input).
///
/// The wires twist and turn, but the two wires occasionally cross paths. To fix
/// the circuit, you need to find the intersection point closest to the central
/// port. Because the wires are on a grid, use the Manhattan distance for this
/// measurement. While the wires do technically cross right at the central port
/// where they both start, this point does not count, nor does a wire count as
/// crossing with itself.
///
/// For example, if the first wire's path is R8,U5,L5,D3, then starting from the
/// central port (o), it goes right 8, up 5, left 5, and finally down 3:
///
/// ...........
/// ...........
/// ...........
/// ....+----+.
/// ....|....|.
/// ....|....|.
/// ....|....|.
/// .........|.
/// .o-------+.
/// ...........
///
/// Then, if the second wire's path is U7,R6,D4,L4, it goes up 7, right 6, down
/// 4, and left 4:
///
/// ...........
/// .+-----+...
/// .|.....|...
/// .|..+--X-+.
/// .|..|..|.|.
/// .|.-X--+.|.
/// .|..|....|.
/// .|.......|.
/// .o-------+.
/// ...........
///
/// These wires cross at two locations (marked X), but the lower-left one is
/// closer to the central port: its distance is 3 + 3 = 6.
///
/// Here are a few more examples:
///
///     R75,D30,R83,U83,L12,D49,R71,U7,L72
///     U62,R66,U55,R34,D71,R55,D58,R83 = distance 159
///     R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
///     U98,R91,D20,R16,D67,R40,U7,R15,U6,R7 = distance 135
///
/// What is the Manhattan distance from the central port to the closest
/// intersection?

use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;

const INPUT: &str = include_str!("../input/day_03.txt");

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Move {
    direction: Direction,
    steps: u32,
}

pub fn run() {
    let wires = get_input();

    println!("{:?}", wires);
}

fn get_input() -> Vec<Vec<Move>> {
    INPUT.lines()
        .map(|line| {
            line.split(',')
                .map(|item| convert_to_move(item))
                .filter_map(|m| m)
                .collect()
        })
        .collect()
}

fn convert_to_move(item: &str) -> Option<Move> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"([URDL])([0-9]+)").unwrap();
    }
    let captures = RE.captures(item);
    match captures {
        Some(groups) => {
            match (groups.get(1), groups.get(2)) {
                (Some(direction), Some(steps)) => {
                    return Some(Move {
                        direction: match direction.as_str() {
                            "U" => Direction::Up,
                            "R" => Direction::Right,
                            "D" => Direction::Down,
                            "L" => Direction::Left,
                            _ => return None,
                        },
                        steps: match steps.as_str().parse() {
                            Ok(number) => number,
                            Err(_) => return None,
                        }
                    });
                },
                _ => None,
            }
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_to_move_up() {
        let input = "U7";
        let output = Some(Move { direction: Direction::Up, steps: 7 });

        assert_eq!(convert_to_move(input), output);
    }

    #[test]
    fn test_convert_to_move_right() {
        let input = "R6";
        let output = Some(Move { direction: Direction::Right, steps: 6 });

        assert_eq!(convert_to_move(input), output);
    }

    #[test]
    fn test_convert_to_move_down() {
        let input = "D4";
        let output = Some(Move { direction: Direction::Down, steps: 4 });

        assert_eq!(convert_to_move(input), output);
    }

    #[test]
    fn test_convert_to_move_left() {
        let input = "L14";
        let output = Some(Move { direction: Direction::Left, steps: 14 });

        assert_eq!(convert_to_move(input), output);
    }

    #[test]
    fn test_convert_to_move_invalid() {
        let input = "34";
        assert_eq!(convert_to_move(input), None);
    }
}
