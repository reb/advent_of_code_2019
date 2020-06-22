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
///
///
/// --- Part Two ---
///
/// You quickly repair the oxygen system; oxygen gradually fills the area.
///
/// Oxygen starts in the location containing the repaired oxygen system. It takes one minute for
/// oxygen to spread to all open locations that are adjacent to a location that already contains
/// oxygen. Diagonal locations are not adjacent.
///
/// In the example above, suppose you've used the droid to explore the area fully and have the
/// following map (where locations that currently contain oxygen are marked O):
///
///  ##   
/// #..##
/// #.#..#
/// #.O.#
///  ###  
///
/// Initially, the only location which contains oxygen is the location of the repaired oxygen
/// system. However, after one minute, the oxygen spreads to all open (.) locations that are
/// adjacent to a location containing oxygen:
///
///  ##   
/// #..##
/// #.#..#
/// #OOO#
///  ###  
///
/// After a total of two minutes, the map looks like this:
///
///  ##   
/// #..##
/// #O#O.#
/// #OOO#
///  ###  
///
/// After a total of three minutes:
///
///  ##   
/// #O.##
/// #O#OO#
/// #OOO#
///  ###  
///
/// And finally, the whole region is full of oxygen after a total of four minutes:
///
///  ##   
/// #OO##
/// #O#OO#
/// #OOO#
///  ###  
///
/// So, in this example, all locations contain oxygen after 4 minutes.
///
/// Use the repair droid to get a complete map of the area. How many minutes will it take to fill
/// with oxygen?
use intcode;
use num::FromPrimitive;
use num_derive::{FromPrimitive, ToPrimitive};
use petgraph::algo::dijkstra;
use petgraph::graphmap::UnGraphMap;
use std::collections::HashMap;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

const INPUT: &str = include_str!("../input/day_15.txt");

pub fn run() {
    let program = intcode::load(INPUT);

    let map = create_map(program);
    display(&map);

    // converting to a graph for easier traversal
    let graph = convert_to_graph(&map);

    // create a map with distances to all points
    let oxygen_system_location = find_location(Section::OxygenSystem, &map);
    let distance_map = dijkstra(&graph, oxygen_system_location, None, |_| 1);

    // find the distance to the starting point
    let starting_location = find_location(Section::Start, &map);
    let distance_to_start = distance_map.get(&starting_location).unwrap();

    println!(
        "The distance from the starting point to the oxygen system is: {}",
        distance_to_start
    );

    // find the longest distance from the oxygen system
    let longest_distance =
        distance_map.iter().map(|(_, distance)| distance).max().unwrap();
    println!(
        "The amount of minutes it will take to fill everything with oxygen is: {}",
        longest_distance
    );
}

fn find_location(section: Section, map: &Map) -> Point {
    *map.iter()
        .find_map(
            |(location, map_section)| {
                if map_section == &section {
                    Some(location)
                } else {
                    None
                }
            },
        )
        .unwrap()
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
    let mut droids = Vec::new();
    let mut map = Map::new();

    let starting_droid = Droid::new(intcode::start(program));
    map.insert(starting_droid.position.clone(), Section::Start);
    droids.push(starting_droid);

    while droids.len() > 0 {
        droids = droids
            .iter()
            .flat_map(|droid| {
                // look in all direction
                Direction::iter()
                    .filter_map(|direction| {
                        let point = droid.point_in_direction(direction);
                        // check if the map already knows about the point in that direction
                        if !map.contains_key(&point) {
                            // let a clone of the droid go in that direction
                            let new_droid = droid.clone();
                            return Some(
                                new_droid.update_map(direction, &mut map),
                            );
                        }
                        None
                    })
                    .collect::<Vec<Droid<_>>>()
            })
            .collect();
    }
    map
}

fn convert_to_graph(map: &Map) -> Graph {
    let mut graph = Graph::new();

    for (point, section) in map.iter() {
        // discard walls
        if section == &Section::Wall {
            continue;
        }

        // add the node to the graph
        let node = graph.add_node(*point);

        // look in all directions to add edges
        for direction in Direction::iter() {
            let other_point = point_in_direction(point, direction);
            if graph.contains_node(other_point) {
                graph.add_edge(node, other_point, ());
            }
        }
    }
    graph
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
type Graph = UnGraphMap<Point, ()>;

#[derive(
    Debug, Copy, Clone, Eq, PartialEq, FromPrimitive, ToPrimitive, EnumIter,
)]
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

#[derive(Debug, Eq, PartialEq, Clone)]
struct Droid<S: intcode::Step> {
    position: Point,
    stepper: S,
}

impl<S: intcode::Step> Droid<S> {
    fn new(stepper: S) -> Droid<S> {
        let position = (0, 0);
        Droid { position, stepper }
    }

    fn update_map(mut self, command: Direction, map: &mut Map) -> Self {
        self.stepper = self.stepper.step(command as i64);
        let next_position = self.point_in_direction(command);

        let reply = FromPrimitive::from_i64(self.stepper.output());
        match reply {
            Some(Reply::Wall) => {
                map.insert(next_position, Section::Wall);
            }
            Some(Reply::Moved) => {
                self.position = next_position;
                map.insert(self.position, Section::Path);
            }
            Some(Reply::OxygenSystem) => {
                self.position = next_position;
                map.insert(self.position, Section::OxygenSystem);
            }
            None => panic!("Unexpected reply"),
        }
        self
    }

    fn point_in_direction(&self, direction: Direction) -> Point {
        point_in_direction(&self.position, direction)
    }
}

fn point_in_direction(start: &Point, direction: Direction) -> Point {
    let (x, y) = *start;
    match direction {
        Direction::North => (x, y - 1),
        Direction::South => (x, y + 1),
        Direction::West => (x - 1, y),
        Direction::East => (x + 1, y),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use intcode::MockStep;

    #[test]
    fn test_droid_movement_wall() {
        let mut step_0 = MockStep::new();
        let mut step_1 = MockStep::new();

        step_1.expect_output().return_const(Reply::Wall as i64);
        step_0.expect_step().return_once(move |_| step_1);

        let mut map = Map::new();
        let mut droid = Droid::new(step_0);
        droid = droid.update_map(Direction::North, &mut map);

        let mut expected_map = Map::new();
        expected_map.insert((0, -1), Section::Wall);

        assert_eq!(&droid.position, &(0, 0));
        assert_eq!(map, expected_map);
    }

    #[test]
    fn test_droid_movement_moved() {
        let mut step_0 = MockStep::new();
        let mut step_1 = MockStep::new();

        step_1.expect_output().return_const(Reply::Moved as i64);
        step_0.expect_step().return_once(move |_| step_1);

        let mut map = Map::new();
        let mut droid = Droid::new(step_0);
        droid = droid.update_map(Direction::South, &mut map);

        let mut expected_map = Map::new();
        expected_map.insert((0, 1), Section::Path);

        assert_eq!(&droid.position, &(0, 1));
        assert_eq!(map, expected_map);
    }

    #[test]
    fn test_droid_movement_oxygen_system() {
        let mut step_0 = MockStep::new();
        let mut step_1 = MockStep::new();
        let mut step_2 = MockStep::new();
        let mut step_3 = MockStep::new();

        step_3.expect_output().return_const(Reply::OxygenSystem as i64);
        step_2.expect_output().return_const(Reply::Moved as i64);
        step_1.expect_output().return_const(Reply::Moved as i64);

        step_2.expect_step().return_once(move |_| step_3);
        step_1.expect_step().return_once(move |_| step_2);
        step_0.expect_step().return_once(move |_| step_1);

        let mut droid = Droid::new(step_0);
        let mut map = Map::new();
        droid = droid.update_map(Direction::West, &mut map);
        droid = droid.update_map(Direction::West, &mut map);
        droid = droid.update_map(Direction::West, &mut map);

        let mut expected_map = Map::new();
        expected_map.insert((-1, 0), Section::Path);
        expected_map.insert((-2, 0), Section::Path);
        expected_map.insert((-3, 0), Section::OxygenSystem);

        assert_eq!(&droid.position, &(-3, 0));
        assert_eq!(map, expected_map);
    }

    #[test]
    fn test_convert_to_graph() {
        //   01234
        // 0  XXX
        // 1 XS..X
        // 2 X.X.X
        // 3 XO..X
        // 4  XXX
        let mut map = Map::new();
        map.insert((0, 1), Section::Wall);
        map.insert((0, 2), Section::Wall);
        map.insert((0, 3), Section::Wall);

        map.insert((1, 0), Section::Wall);
        map.insert((1, 1), Section::Start);
        map.insert((1, 2), Section::Path);
        map.insert((1, 3), Section::Path);
        map.insert((1, 4), Section::Wall);

        map.insert((2, 0), Section::Wall);
        map.insert((2, 1), Section::Path);
        map.insert((2, 2), Section::Wall);
        map.insert((2, 3), Section::Path);
        map.insert((2, 4), Section::Wall);

        map.insert((3, 0), Section::Wall);
        map.insert((3, 1), Section::OxygenSystem);
        map.insert((3, 2), Section::Path);
        map.insert((3, 3), Section::Path);
        map.insert((3, 4), Section::Wall);

        map.insert((4, 1), Section::Wall);
        map.insert((4, 2), Section::Wall);
        map.insert((4, 3), Section::Wall);

        let graph = convert_to_graph(&map);

        assert_eq!(graph.node_count(), 8);
        assert_eq!(graph.edge_count(), 8);
    }
}
