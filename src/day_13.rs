/// --- Day 13: Care Package ---
///
/// As you ponder the solitude of space and the ever-increasing three-hour
/// roundtrip for messages between you and Earth, you notice that the Space Mail
/// Indicator Light is blinking. To help keep you sane, the Elves have sent you
/// a care package.
///
/// It's a new game for the ship's arcade cabinet! Unfortunately, the arcade is
/// all the way on the other end of the ship. Surely, it won't be hard to build
/// your own - the care package even comes with schematics.
///
/// The arcade cabinet runs Intcode software like the game the Elves sent (your
/// puzzle input). It has a primitive screen capable of drawing square tiles on
/// a grid. The software draws tiles to the screen with output instructions:
/// every three output instructions specify the x position (distance from the
/// left), y position (distance from the top), and tile id. The tile id is
/// interpreted as follows:
///
///     0 is an empty tile. No game object appears in this tile.
///     1 is a wall tile. Walls are indestructible barriers.
///     2 is a block tile. Blocks can be broken by the ball.
///     3 is a horizontal paddle tile. The paddle is indestructible.
///     4 is a ball tile. The ball moves diagonally and bounces off objects.
///
/// For example, a sequence of output values like 1,2,3,6,5,4 would draw a
/// horizontal paddle tile (1 tile from the left and 2 tiles from the top) and a
/// ball tile (6 tiles from the left and 5 tiles from the top).
///
/// Start the game. How many block tiles are on the screen when the game exits?
/// --- Part Two ---
///
/// The game didn't run because you didn't put in any quarters. Unfortunately,
/// you did not bring any quarters. Memory address 0 represents the number of
/// quarters that have been inserted; set it to 2 to play for free.
///
/// The arcade cabinet has a joystick that can move left and right. The software
/// reads the position of the joystick with input instructions:
///
///     If the joystick is in the neutral position, provide 0.
///     If the joystick is tilted to the left, provide -1.
///     If the joystick is tilted to the right, provide 1.
///
/// The arcade cabinet also has a segment display capable of showing a single
/// number that represents the player's current score. When three output
/// instructions specify X=-1, Y=0, the third output instruction is not a tile;
/// the value instead specifies the new score to show in the segment display.
/// For example, a sequence of output values like -1,0,12345 would show 12345 as
/// the player's current score.
///
/// Beat the game by breaking all the blocks. What is your score after the last
/// block is broken?
use intcode;
use itertools::Itertools;
use num::FromPrimitive;
use num_derive::{FromPrimitive, ToPrimitive};
use std::cmp::Ordering;
use std::collections::HashMap;

const INPUT: &str = include_str!("../input/day_13.txt");

pub fn run() {
    let mut game = intcode::load(INPUT);
    let (_, _, outputs) = intcode::start(game.clone(), Vec::new());
    let (screen, _) = render(outputs);

    let block_tiles =
        screen.values().filter(|tile| tile == &&Tile::Block).count();
    println!(
        "There have been {} block tiles rendered on the screen",
        block_tiles
    );
    display(&screen);

    // set the game to free play
    game.insert(0, 2);

    // play the game
}

type Point = (i64, i64);
type Screen = HashMap<Point, Tile>;

#[derive(Debug, PartialEq, FromPrimitive)]
enum Tile {
    Empty = 0,
    Wall = 1,
    Block = 2,
    HorizontalPaddle = 3,
    Ball = 4,
}

#[derive(Debug, PartialEq, ToPrimitive)]
enum Joystick {
    Neutral = 0,
    Left = -1,
    Right = 1,
}

fn determine_joystick(screen: &Screen) -> Joystick {
    let (ball_x, _) = find_tile(screen, Tile::Ball).unwrap();
    let (paddle_x, _) = find_tile(screen, Tile::HorizontalPaddle).unwrap();
    match ball_x.cmp(&paddle_x) {
        Ordering::Equal => Joystick::Neutral,
        Ordering::Greater => Joystick::Right,
        Ordering::Less => Joystick::Left,
    }
}

fn find_tile(screen: &Screen, to_find: Tile) -> Option<Point> {
    match screen.iter().find(|(_, tile)| tile == &&to_find) {
        Some((point, _)) => Some(*point),
        None => None,
    }
}

fn display(screen: &Screen) {
    let x_min = screen.keys().map(|&(x, _)| x).min().unwrap();
    let x_max = screen.keys().map(|&(x, _)| x).max().unwrap();
    let y_min = screen.keys().map(|&(_, y)| y).min().unwrap();
    let y_max = screen.keys().map(|&(_, y)| y).max().unwrap();

    for y in y_min..=y_max {
        for x in x_min..=x_max {
            match *screen.get(&(x, y)).unwrap_or(&Tile::Empty) {
                Tile::Empty => print!(" "),
                Tile::Wall => print!("\u{2588}"),
                Tile::Block => print!("\u{2592}"),
                Tile::HorizontalPaddle => print!("-"),
                Tile::Ball => print!("o"),
            };
        }
        print!("\n");
    }
}

fn render(outputs: intcode::Outputs) -> (Screen, i64) {
    let mut screen = HashMap::new();
    let mut score = 0;
    for (&x, &y, &value) in outputs.iter().tuples() {
        // extract the score
        if x == -1 && y == 0 {
            score = value;
        }
        let tile = FromPrimitive::from_i64(value).unwrap();
        screen.insert((x, y), tile);
    }

    (screen, score)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_1() {
        let outputs = vec![1, 2, 3, 6, 5, 4];

        let mut expected_screen = HashMap::new();
        expected_screen.insert((1, 2), Tile::HorizontalPaddle);
        expected_screen.insert((6, 5), Tile::Ball);

        let (actual_screen, _) = render(outputs);
        assert_eq!(actual_screen, expected_screen);
    }

    #[test]
    fn test_render_2() {
        let outputs = vec![-10, 5, 2, 400, -400, 1, 0, 0, 0];

        let mut expected_screen = HashMap::new();
        expected_screen.insert((-10, 5), Tile::Block);
        expected_screen.insert((400, -400), Tile::Wall);
        expected_screen.insert((0, 0), Tile::Empty);

        let (actual_screen, _) = render(outputs);
        assert_eq!(actual_screen, expected_screen);
    }

    #[test]
    fn determine_joystick_neutral() {
        let mut screen = HashMap::new();
        screen.insert((8, 8), Tile::HorizontalPaddle);
        screen.insert((8, 3), Tile::Ball);

        assert_eq!(determine_joystick(&screen), Joystick::Neutral);
    }

    #[test]
    fn determine_joystick_left() {
        let mut screen = HashMap::new();
        screen.insert((6, -10), Tile::HorizontalPaddle);
        screen.insert((2, -12), Tile::Ball);

        assert_eq!(determine_joystick(&screen), Joystick::Left);
    }

    #[test]
    fn determine_joystick_right() {
        let mut screen = HashMap::new();
        screen.insert((-10, 5), Tile::HorizontalPaddle);
        screen.insert((0, 5), Tile::Ball);

        assert_eq!(determine_joystick(&screen), Joystick::Right);
    }
}
