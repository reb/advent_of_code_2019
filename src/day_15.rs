/// --- Day 15: Oxygen System ---
///
/// Out here in deep space, many things can go wrong. Fortunately, many of those things have
/// indicator lights. Unfortunately, one of those lights is lit: the oxygen system for part of the
/// ship has failed!
///
/// According to the readouts, the oxygen system must have failed days ago after a rupture in
/// oxygen tank two; that section of the ship was automatically sealed once oxygen levels went
/// dangerously low. A single remotely-operated repair droid is your only option for fixing the
/// oxygen system.
///
/// The Elves' care package included an Intcode program (your puzzle input) that you can use to
/// remotely control the repair droid. By running that program, you can direct the repair droid to
/// the oxygen system and fix the problem.
///
/// The remote control program executes the following steps in a loop forever:
///
///     Accept a movement command via an input instruction.
///     Send the movement command to the repair droid.
///     Wait for the repair droid to finish the movement operation.
///     Report on the status of the repair droid via an output instruction.
///
/// Only four movement commands are understood: north (1), south (2), west (3), and east (4). Any
/// other command is invalid. The movements differ in direction, but not in distance: in a long
/// enough east-west hallway, a series of commands like 4,4,4,4,3,3,3,3 would leave the repair
/// droid back where it started.
///
/// The repair droid can reply with any of the following status codes:
///
///     0: The repair droid hit a wall. Its position has not changed.
///     1: The repair droid has moved one step in the requested direction.
///     2: The repair droid has moved one step in the requested direction; its new position is the
///        location of the oxygen system.
///
/// You don't know anything about the area around the repair droid, but you can figure it out by
/// watching the status codes.
///
/// For example, we can draw the area using D for the droid, # for walls, . for locations the droid
/// can traverse, and empty space for unexplored locations. Then, the initial state looks like
/// this:
///
///       
///       
///    D  
///       
///       
///
/// To make the droid go north, send it 1. If it replies with 0, you know that location is a wall
/// and that the droid didn't move:
///
///       
///    #  
///    D  
///       
///       
///
/// To move east, send 4; a reply of 1 means the movement was successful:
///
///       
///    #  
///    .D
///       
///       
///
/// Then, perhaps attempts to move north (1), south (2), and east (4) are all met with replies of
/// 0:
///
///       
///    ##
///    .D#
///     #
///       
///
/// Now, you know the repair droid is in a dead end. Backtrack with 3 (which you already know will
/// get a reply of 1 because you already know that location is open):
///
///       
///    ##
///    D.#
///     #
///       
///
/// Then, perhaps west (3) gets a reply of 0, south (2) gets a reply of 1, south again (2) gets a
/// reply of 0, and then west (3) gets a reply of 2:
///
///       
///    ##
///   #..#
///   D.#
///    #  
///
/// Now, because of the reply of 2, you know you've found the oxygen system! In this example, it
/// was only 2 moves away from the repair droid's starting position.
///
/// What is the fewest number of movement commands required to move the repair droid from its
/// starting position to the location of the oxygen system?
use intcode;
use num;
use num_derive::{FromPrimitive, ToPrimitive};
use std::collections::HashMap;

const INPUT: &str = include_str!("../input/day_15.txt");

pub fn run() {
    let program = intcode::load(INPUT);

    let map = create_map(program);
    display(&map);
}

fn display(map: &Map) {
    let x_min = map.keys().map(|&(x, _)| x).min().unwrap();
    let x_max = map.keys().map(|&(x, _)| x).max().unwrap();
    let y_min = map.keys().map(|&(_, y)| y).min().unwrap();
    let y_max = map.keys().map(|&(_, y)| y).max().unwrap();

    for y in y_min..=y_max {
        for x in x_min..=x_max {
            match map.get(&(x, y)) {
                None => print!(" "),
                Some(Section::Path) => print!("\u{2591}"),
                Some(Section::Wall) => print!("\u{2588}"),
                Some(Section::OxygenSystem) => print!("O"),
                Some(Section::Start) => print!("X"),
            };
        }
        print!("\n");
    }
}

fn create_map(program: intcode::Program) -> Map {
    // create a droid to explore the map
    let mut droid = Droid::new();

    let mut runner = intcode::start(program);

    loop {
        let direction = droid.pledge_direction();
        runner = runner.step(direction as i64).unwrap();

        assert!(runner.outputs.len() == 1);

        let reply: Reply =
            num::FromPrimitive::from_i64(runner.outputs[0]).unwrap();
        droid.movement(direction, reply);
        if reply == Reply::OxygenSystem {
            break;
        }
    }

    droid.map
}

type Point = (i32, i32);

#[derive(Debug, PartialEq, Eq)]
enum Section {
    Start,
    Path,
    Wall,
    OxygenSystem,
}
type Map = HashMap<Point, Section>;

#[derive(Debug, Copy, Clone, Eq, PartialEq, FromPrimitive, ToPrimitive)]
enum Direction {
    North = 1,
    South,
    West,
    East,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, FromPrimitive, ToPrimitive)]
enum Reply {
    Wall,  // 0: The repair droid hit a wall. Its position has not changed.
    Moved, // 1: The repair droid has moved one step in the requested direction.
    OxygenSystem, // 2: The repair droid has moved one step in the requested direction; its new
                  // position is the location of the oxygen system.
}

#[derive(Debug, Eq, PartialEq)]
struct Droid {
    position: Point,
    map: Map,
    hand: Option<Direction>,
    turns: i32,
}

impl Droid {
    fn new() -> Droid {
        let position = (0, 0);
        let mut map = Map::new();
        map.insert(position, Section::Start);
        Droid { position, map, hand: None, turns: 0 }
    }

    fn movement(&mut self, command: Direction, reply: Reply) {
        match reply {
            Reply::Wall => {
                self.map
                    .insert(self.point_in_direction(command), Section::Wall);
            }
            Reply::Moved => {
                self.position = self.point_in_direction(command);
                self.map.insert(self.position, Section::Path);
            }
            Reply::OxygenSystem => {
                self.position = self.point_in_direction(command);
                self.map.insert(self.position, Section::OxygenSystem);
            }
        }
    }

    fn point_in_direction(&self, direction: Direction) -> Point {
        let (x, y) = self.position;
        match direction {
            Direction::North => (x, y - 1),
            Direction::South => (x, y + 1),
            Direction::West => (x - 1, y),
            Direction::East => (x + 1, y),
        }
    }

    fn pledge_direction(&mut self) -> Direction {
        // Pledge algorithm
        let direction = match self.hand {
            None => {
                // if there is no hand on the wall prefer North
                let wall_in_preferred_direction = self
                    .map
                    .get(&self.point_in_direction(Direction::North))
                    .contains(&&Section::Wall);
                match wall_in_preferred_direction {
                    true => {
                        // if there is a wall put a hand in the North direction
                        self.hand = Some(Direction::North);
                        self.turns -= 1;
                        // and take a left turn to the West
                        Direction::West
                    }
                    false => Direction::North,
                }
            }
            Some(hand_direction) => {
                let wall_in_hand_direction = self
                    .map
                    .get(&self.point_in_direction(hand_direction))
                    .contains(&&Section::Wall);
                match wall_in_hand_direction {
                    true => {
                        // with a hand on the wall turn counter-clockwise
                        let intended_direction = match hand_direction {
                            Direction::North => Direction::West,
                            Direction::West => Direction::South,
                            Direction::South => Direction::East,
                            Direction::East => Direction::North,
                        };
                        let wall_in_intended_direction = self
                            .map
                            .get(&self.point_in_direction(intended_direction))
                            .contains(&&Section::Wall);
                        match wall_in_intended_direction {
                            true => {
                                // there is also a wall to the left, change hand to the left
                                self.turns -= 1;
                                self.hand = Some(intended_direction);
                                // and try again
                                self.pledge_direction()
                            }
                            false => intended_direction,
                        }
                    }
                    false => {
                        // its possible to go in the hand direction, turn hand clockwise
                        self.turns += 1;
                        self.hand = Some(match hand_direction {
                            Direction::North => Direction::East,
                            Direction::East => Direction::South,
                            Direction::South => Direction::West,
                            Direction::West => Direction::North,
                        });
                        hand_direction
                    }
                }
            }
        };
        if self.turns == 0 {
            self.hand = None
        }
        // println!(
        //     "Going {:?}, at {:?}, with hand: {:?} and turns: {}",
        //     direction, self.position, self.hand, self.turns
        // );
        direction
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_droid_movement_wall() {
        let mut droid = Droid::new();
        droid.movement(Direction::North, Reply::Wall);

        let mut expected_map = Map::new();
        expected_map.insert((0, 0), Section::Start);
        expected_map.insert((0, -1), Section::Wall);
        let expected_droid =
            Droid { position: (0, 0), map: expected_map, hand: None, turns: 0 };

        assert_eq!(droid, expected_droid);
    }

    #[test]
    fn test_droid_movement_moved() {
        let mut droid = Droid::new();
        droid.movement(Direction::South, Reply::Moved);

        let mut expected_map = Map::new();
        expected_map.insert((0, 0), Section::Start);
        expected_map.insert((0, 1), Section::Path);
        let expected_droid =
            Droid { position: (0, 1), map: expected_map, hand: None, turns: 0 };

        assert_eq!(droid, expected_droid);
    }

    #[test]
    fn test_droid_movement_oxygen_system() {
        let mut droid = Droid::new();
        droid.movement(Direction::West, Reply::Moved);
        droid.movement(Direction::West, Reply::Moved);
        droid.movement(Direction::West, Reply::OxygenSystem);

        let mut expected_map = Map::new();
        expected_map.insert((0, 0), Section::Start);
        expected_map.insert((-1, 0), Section::Path);
        expected_map.insert((-2, 0), Section::Path);
        expected_map.insert((-3, 0), Section::OxygenSystem);
        let expected_droid = Droid {
            position: (-3, 0),
            map: expected_map,
            hand: None,
            turns: 0,
        };

        assert_eq!(droid, expected_droid);
    }

    #[test]
    fn test_droid_pledge_direction_starts_north() {
        let mut droid = Droid::new();

        assert_eq!(droid.pledge_direction(), Direction::North);
    }

    #[test]
    fn test_droid_pledge_direction_places_hand_when_blocked() {
        let mut droid = Droid::new();
        let mut map = Map::new();
        map.insert((0, -1), Section::Wall);
        droid.map = map;

        assert_eq!(droid.pledge_direction(), Direction::West);
        assert_eq!(droid.hand, Some(Direction::North));
        assert_eq!(droid.turns, -1);
    }
}
