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
use num_derive::FromPrimitive;
use std::collections::HashMap;

const INPUT: &str = include_str!("../input/day_13.txt");

pub fn run() {
    let game = intcode::load(INPUT);
    let (_, _, outputs) = intcode::start(game, Vec::new());
    let screen = render(outputs);

    let block_tiles =
        screen.values().filter(|tile| tile == &&Tile::Block).count();
    println!(
        "There have been {} block tiles rendered on the screen",
        block_tiles
    );
    display(&screen);
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

fn render(outputs: intcode::Outputs) -> Screen {
    let mut screen = HashMap::new();
    for (&x, &y, &tile_code) in outputs.iter().tuples() {
        let tile = FromPrimitive::from_i64(tile_code).unwrap();
        screen.insert((x, y), tile);
    }

    screen
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

        assert_eq!(render(outputs), expected_screen);
    }

    #[test]
    fn test_render_2() {
        let outputs = vec![-10, 5, 2, 400, -400, 1, 0, 0, 0];

        let mut expected_screen = HashMap::new();
        expected_screen.insert((-10, 5), Tile::Block);
        expected_screen.insert((400, -400), Tile::Wall);
        expected_screen.insert((0, 0), Tile::Empty);

        assert_eq!(render(outputs), expected_screen);
    }
}
