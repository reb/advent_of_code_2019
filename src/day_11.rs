/// --- Day 11: Space Police ---
///
/// On the way to Jupiter, you're pulled over by the Space Police.
///
/// "Attention, unmarked spacecraft! You are in violation of Space Law! All
/// spacecraft must have a clearly visible registration identifier! You have 24
/// hours to comply or be sent to Space Jail!"
///
/// Not wanting to be sent to Space Jail, you radio back to the Elves on Earth
/// for help. Although it takes almost three hours for their reply signal to
/// reach you, they send instructions for how to power up the emergency hull
/// painting robot and even provide a small Intcode program (your puzzle input)
/// that will cause it to paint your ship appropriately.
///
/// There's just one problem: you don't have an emergency hull painting robot.
///
/// You'll need to build a new emergency hull painting robot. The robot needs to
/// be able to move around on the grid of square panels on the side of your
/// ship, detect the color of its current panel, and paint its current panel
/// black or white. (All of the panels are currently black.)
///
/// The Intcode program will serve as the brain of the robot. The program uses
/// input instructions to access the robot's camera: provide 0 if the robot is
/// over a black panel or 1 if the robot is over a white panel. Then, the
/// program will output two values:
///
///     First, it will output a value indicating the color to paint the panel
///     the robot is over: 0 means to paint the panel black, and 1 means to
///     paint the panel white.
///     Second, it will output a value indicating the direction the robot should
///     turn: 0 means it should turn left 90 degrees, and 1 means it should turn
///     right 90 degrees.
///
/// After the robot turns, it should always move forward exactly one panel. The
/// robot starts facing up.
///
/// The robot will continue running for a while like this and halt when it is
/// finished drawing. Do not restart the Intcode computer inside the robot
/// during this process.
///
/// For example, suppose the robot is about to start running. Drawing black
/// panels as ., white panels as #, and the robot pointing the direction it is
/// facing (< ^ > v), the initial state and region near the robot looks like
/// this:
///
/// .....
/// .....
/// ..^..
/// .....
/// .....
///
/// The panel under the robot (not visible here because a ^ is shown instead) is
/// also black, and so any input instructions at this point should be provided
/// 0. Suppose the robot eventually outputs 1 (paint white) and then 0 (turn
/// left). After taking these actions and moving forward one panel, the region
/// now looks like this:
///
/// .....
/// .....
/// .<#..
/// .....
/// .....
///
/// Input instructions should still be provided 0. Next, the robot might output
/// 0 (paint black) and then 0 (turn left):
///
/// .....
/// .....
/// ..#..
/// .v...
/// .....
///
/// After more outputs (1,0, 1,0):
///
/// .....
/// .....
/// ..^..
/// .##..
/// .....
///
/// The robot is now back where it started, but because it is now on a white
/// panel, input instructions should be provided 1. After several more outputs
/// (0,1, 1,0, 1,0), the area looks like this:
///
/// .....
/// ..<#.
/// ...#.
/// .##..
/// .....
///
/// Before you deploy the robot, you should probably have an estimate of the
/// area it will cover: specifically, you need to know the number of panels it
/// paints at least once, regardless of color. In the example above, the robot
/// painted 6 panels at least once. (It painted its starting panel twice, but
/// that panel is still only counted once; it also never painted the panel it
/// ended on.)
///
/// Build a new emergency hull painting robot and run the Intcode program on it.
/// How many panels does it paint at least once?
use intcode;
use intcode::ExitStatus::WaitingForInput;
use itertools::Itertools;
use num;
use num_derive::{FromPrimitive, ToPrimitive};
use std::collections::HashMap;
use std::mem::discriminant;

const INPUT: &str = include_str!("../input/day_11.txt");

type Point = (i32, i32);
type Hull = HashMap<Point, i64>;

#[derive(Debug, Copy, Clone, Eq, PartialEq, FromPrimitive, ToPrimitive)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug, Copy, Clone)]
struct Robot {
    position: Point,
    heading: Direction,
}

impl Robot {
    fn new() -> Robot {
        Robot { position: (0, 0), heading: Direction::Up }
    }

    /// Paint the hull at the current location with the given color
    fn paint(self, instruction: &i64, hull: &mut Hull) {
        hull.insert(self.position, *instruction);
    }

    /// Execute a turn instruction (turn & move forward)
    fn turn(&mut self, instruction: &i64) {
        // turn depending on the instruction
        let heading = self.heading as i8;
        self.heading = match instruction {
            0 => num::FromPrimitive::from_i8((heading - 1).rem_euclid(4)),
            1 => num::FromPrimitive::from_i8((heading + 1).rem_euclid(4)),
            _ => panic!("Got an unknown turning instruction!"),
        }
        .unwrap();

        // move one step forward
        self.forward();
    }

    fn forward(&mut self) {
        let (x, y) = self.position;
        self.position = match self.heading {
            Direction::Up => (x, y - 1),
            Direction::Right => (x + 1, y),
            Direction::Down => (x, y + 1),
            Direction::Left => (x - 1, y),
        }
    }

    /// Give a readout of the color of the hull at the currenct position
    fn read_camera(self, hull: &Hull) -> i64 {
        *hull.get(&self.position).unwrap_or(&0)
    }
}

pub fn run() {
    let brain = intcode::load(INPUT);

    let painted_hull = paint_hull(brain.clone(), HashMap::new());

    println!(
        "The amount of panels painted at least once is: {}",
        painted_hull.len()
    );
    display(&painted_hull);

    // start on a white square
    let mut white_starting_hull = HashMap::new();
    white_starting_hull.insert((0, 0), 1);
    let proper_painted_hull = paint_hull(brain.clone(), white_starting_hull);

    println!("With starting on a white square the robot paints:");
    display(&proper_painted_hull);
}

fn paint_hull(brain: intcode::Program, mut hull: Hull) -> Hull {
    let mut robot = Robot::new();

    let (mut program, mut status, mut outputs) =
        intcode::start(brain, Vec::new());
    while discriminant(&status) == discriminant(&WaitingForInput(0, 0)) {
        for (paint_instruction, turn_instruction) in outputs.iter().tuples() {
            robot.paint(paint_instruction, &mut hull);
            robot.turn(turn_instruction);
        }
        let inputs = vec![robot.read_camera(&hull)];
        let (new_program, new_status, new_outputs) =
            intcode::resume(program, status, inputs);
        program = new_program;
        status = new_status;
        outputs = new_outputs;
    }
    hull
}

fn display(hull: &Hull) {
    let x_min = hull.keys().map(|&(x, _)| x).min().unwrap();
    let x_max = hull.keys().map(|&(x, _)| x).max().unwrap();
    let y_min = hull.keys().map(|&(_, y)| y).min().unwrap();
    let y_max = hull.keys().map(|&(_, y)| y).max().unwrap();

    for y in y_min..=y_max {
        for x in x_min..=x_max {
            match *hull.get(&(x, y)).unwrap_or(&0) {
                0 => print!(" "),
                1 => print!("\u{2588}"),
                _ => print!("?"),
            };
        }
        print!("\n");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_robot_read_camera_default_color() {
        let hull = HashMap::new();
        let robot = Robot::new();

        assert_eq!(robot.read_camera(&hull), 0);
    }

    #[test]
    fn test_robot_read_camera_white() {
        let mut hull = HashMap::new();
        hull.insert((1, 1), 1);
        let mut robot = Robot::new();
        robot.position = (1, 1);

        assert_eq!(robot.read_camera(&hull), 1);
    }

    #[test]
    fn test_robot_paint_white() {
        let mut hull = HashMap::new();
        let mut robot = Robot::new();
        robot.position = (2, 0);

        let mut expected_hull = HashMap::new();
        expected_hull.insert((2, 0), 1);

        robot.paint(&1, &mut hull);
        assert_eq!(hull, expected_hull);
    }

    #[test]
    fn test_robot_paint_black() {
        let mut hull = HashMap::new();
        let mut robot = Robot::new();
        robot.position = (-1, -2);

        let mut expected_hull = HashMap::new();
        expected_hull.insert((-1, -2), 0);

        robot.paint(&0, &mut hull);
        assert_eq!(hull, expected_hull);
    }

    #[test]
    fn test_robot_turn_left() {
        let mut robot = Robot::new();
        robot.position = (1, 1);
        robot.heading = Direction::Up;

        robot.turn(&0);

        assert_eq!(robot.heading, Direction::Left);
        assert_eq!(robot.position, (0, 1));
    }

    #[test]
    fn test_robot_turn_right() {
        let mut robot = Robot::new();
        robot.position = (0, -2);
        robot.heading = Direction::Right;

        robot.turn(&1);

        assert_eq!(robot.heading, Direction::Down);
        assert_eq!(robot.position, (0, -1));
    }
}
