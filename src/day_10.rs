/// --- Day 10: Monitoring Station ---
///
/// You fly into the asteroid belt and reach the Ceres monitoring station. The
/// Elves here have an emergency: they're having trouble tracking all of the
/// asteroids and can't be sure they're safe.
///
/// The Elves would like to build a new monitoring station in a nearby area of
/// space; they hand you a map of all of the asteroids in that region (your
/// puzzle input).
///
/// The map indicates whether each position is empty (.) or contains an asteroid
/// (#). The asteroids are much smaller than they appear on the map, and every
/// asteroid is exactly in the center of its marked position. The asteroids can
/// be described with X,Y coordinates where X is the distance from the left edge
/// and Y is the distance from the top edge (so the top-left corner is 0,0 and
/// the position immediately to its right is 1,0).
///
/// Your job is to figure out which asteroid would be the best place to build a
/// new monitoring station. A monitoring station can detect any asteroid to
/// which it has direct line of sight - that is, there cannot be another
/// asteroid exactly between them. This line of sight can be at any angle, not
/// just lines aligned to the grid or diagonally. The best location is the
/// asteroid that can detect the largest number of other asteroids.
///
/// For example, consider the following map:
///
/// .#..#
/// .....
/// #####
/// ....#
/// ...##
///
/// The best location for a new monitoring station on this map is the
/// highlighted asteroid at 3,4 because it can detect 8 asteroids, more than any
/// other location. (The only asteroid it cannot detect is the one at 1,0; its
/// view of this asteroid is blocked by the asteroid at 2,2.) All other
/// asteroids are worse locations; they can detect 7 or fewer other asteroids.
/// Here is the number of other asteroids a monitoring station on each asteroid
/// could detect:
///
/// .7..7
/// .....
/// 67775
/// ....7
/// ...87
///
/// Here is an asteroid (#) and some examples of the ways its line of sight
/// might be blocked. If there were another asteroid at the location of a
/// capital letter, the locations marked with the corresponding lowercase letter
/// would be blocked and could not be detected:
///
/// #.........
/// ...A......
/// ...B..a...
/// .EDCG....a
/// ..F.c.b...
/// .....c....
/// ..efd.c.gb
/// .......c..
/// ....f...c.
/// ...e..d..c
///
/// Here are some larger examples:
///
///     Best is 5,8 with 33 other asteroids detected:
///
///     ......#.#.
///     #..#.#....
///     ..#######.
///     .#.#.###..
///     .#..#.....
///     ..#....#.#
///     #..#....#.
///     .##.#..###
///     ##...#..#.
///     .#....####
///
///     Best is 1,2 with 35 other asteroids detected:
///
///     #.#...#.#.
///     .###....#.
///     .#....#...
///     ##.#.#.#.#
///     ....#.#.#.
///     .##..###.#
///     ..#...##..
///     ..##....##
///     ......#...
///     .####.###.
///
///     Best is 6,3 with 41 other asteroids detected:
///
///     .#..#..###
///     ####.###.#
///     ....###.#.
///     ..###.##.#
///     ##.##.#.#.
///     ....###..#
///     ..#.#..#.#
///     #..#.#.###
///     .##...##.#
///     .....#.#..
///
///     Best is 11,13 with 210 other asteroids detected:
///
///     .#..##.###...#######
///     ##.############..##.
///     .#.######.########.#
///     .###.#######.####.#.
///     #####.##.#.##.###.##
///     ..#####..#.#########
///     ####################
///     #.####....###.#.#.##
///     ##.#################
///     #####.##.###..####..
///     ..######..##.#######
///     ####.##.####...##..#
///     .#####..#.######.###
///     ##...#.##########...
///     #.##########.#######
///     .####.#.###.###.#.##
///     ....##.##.###..#####
///     .#.#.###########.###
///     #.#.#.#####.####.###
///     ###.##.####.##.#..##
///
/// Find the best location for a new monitoring station. How many other
/// asteroids can be detected from that location?
use itertools::Itertools;
use num::integer::gcd;
use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("../input/day_10.txt");

type Point = (i32, i32);
type Fraction = (i32, i32);

pub fn run() {
    println!("Not implemented yet");
    unimplemented!();
}

fn count_visible(asteroids: &HashSet<Point>) -> HashMap<Point, u8> {
    HashMap::new()
}

fn find_fraction(station: &Point, asteroid: &Point) -> Fraction {
    let (station_x, station_y) = station;
    let (asteroid_x, asteroid_y) = asteroid;

    let difference_x = asteroid_x - station_x;
    let difference_y = asteroid_y - station_y;

    let gcd = gcd(difference_x, difference_y);

    (difference_x / gcd, difference_y / gcd)
}

fn load_asteroids(input: &str) -> HashSet<Point> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, c)| match c {
                '#' => Some((x as i32, y as i32)),
                _ => None,
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_asteroids_simple() {
        let input = ".#\n#.";
        let output = [(1, 0), (0, 1)].iter().cloned().collect();

        assert_eq!(load_asteroids(input), output);
    }

    #[test]
    fn test_load_asteroids_bigger() {
        let input = ".#..#\n.....\n#####\n....#\n...##";
        let output = [
            (1, 0),
            (4, 0),
            (0, 2),
            (1, 2),
            (2, 2),
            (3, 2),
            (4, 2),
            (4, 3),
            (3, 4),
            (4, 4),
        ]
        .iter()
        .cloned()
        .collect();

        assert_eq!(load_asteroids(input), output);
    }

    #[test]
    fn test_count_visible() {
        // .#..#
        // .....
        // #####
        // ....#
        // ...##
        let input = [
            (1, 0),
            (4, 0),
            (0, 2),
            (1, 2),
            (2, 2),
            (3, 2),
            (4, 2),
            (4, 3),
            (3, 4),
            (4, 4),
        ]
        .iter()
        .cloned()
        .collect();

        // .7..7
        // .....
        // 67775
        // ....7
        // ...87
        let output = [
            ((1, 0), 7),
            ((4, 0), 7),
            ((0, 2), 6),
            ((1, 2), 7),
            ((2, 2), 7),
            ((3, 2), 7),
            ((4, 2), 5),
            ((4, 3), 7),
            ((3, 4), 8),
            ((4, 4), 7),
        ]
        .iter()
        .cloned()
        .collect();

        assert_eq!(count_visible(&input), output);
    }

    #[test]
    fn test_find_fraction_1_1() {
        let station = (0, 0);
        let asteroid = (2, 2);

        let fraction = (1, 1);

        assert_eq!(find_fraction(&station, &asteroid), fraction);
    }

    #[test]
    fn test_find_fraction_negative_3_1() {
        let station = (12, 0);
        let asteroid = (0, 4);

        let fraction = (-3, 1);

        assert_eq!(find_fraction(&station, &asteroid), fraction);
    }
}
