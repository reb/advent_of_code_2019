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
///
/// --- Part Two ---
///
/// Once you give them the coordinates, the Elves quickly deploy an Instant
/// Monitoring Station to the location and discover the worst: there are simply
/// too many asteroids.
///
/// The only solution is complete vaporization by giant laser.
///
/// Fortunately, in addition to an asteroid scanner, the new monitoring station
/// also comes equipped with a giant rotating laser perfect for vaporizing
/// asteroids. The laser starts by pointing up and always rotates clockwise,
/// vaporizing any asteroid it hits.
///
/// If multiple asteroids are exactly in line with the station, the laser only
/// has enough power to vaporize one of them before continuing its rotation. In
/// other words, the same asteroids that can be detected can be vaporized, but
/// if vaporizing one asteroid makes another one detectable, the newly-detected
/// asteroid won't be vaporized until the laser has returned to the same
/// position by rotating a full 360 degrees.
///
/// For example, consider the following map, where the asteroid with the new
/// monitoring station (and laser) is marked X:
///
/// .#....#####...#..
/// ##...##.#####..##
/// ##...#...#.#####.
/// ..#.....X...###..
/// ..#.#.....#....##
///
/// The first nine asteroids to get vaporized, in order, would be:
///
/// .#....###24...#..
/// ##...##.13#67..9#
/// ##...#...5.8####.
/// ..#.....X...###..
/// ..#.#.....#....##
///
/// Note that some asteroids (the ones behind the asteroids marked 1, 5, and 7)
/// won't have a chance to be vaporized until the next full rotation. The laser
/// continues rotating; the next nine to be vaporized are:
///
/// .#....###.....#..
/// ##...##...#.....#
/// ##...#......1234.
/// ..#.....X...5##..
/// ..#.9.....8....76
///
/// The next nine to be vaporized are then:
///
/// .8....###.....#..
/// 56...9#...#.....#
/// 34...7...........
/// ..2.....X....##..
/// ..1..............
///
/// Finally, the laser completes its first full rotation (1 through 3), a second
/// rotation (4 through 8), and vaporizes the last asteroid (9) partway through
/// its third rotation:
///
/// ......234.....6..
/// ......1...5.....7
/// .................
/// ........X....89..
/// .................
///
/// In the large example above (the one with the best monitoring station
/// location at 11,13):
///
///     The 1st asteroid to be vaporized is at 11,12.
///     The 2nd asteroid to be vaporized is at 12,1.
///     The 3rd asteroid to be vaporized is at 12,2.
///     The 10th asteroid to be vaporized is at 12,8.
///     The 20th asteroid to be vaporized is at 16,0.
///     The 50th asteroid to be vaporized is at 16,9.
///     The 100th asteroid to be vaporized is at 10,16.
///     The 199th asteroid to be vaporized is at 9,6.
///     The 200th asteroid to be vaporized is at 8,2.
///     The 201st asteroid to be vaporized is at 10,9.
///     The 299th and final asteroid to be vaporized is at 11,1.
///
/// The Elves are placing bets on which will be the 200th asteroid to be
/// vaporized. Win the bet by determining which asteroid that will be; what do
/// you get if you multiply its X coordinate by 100 and then add its Y
/// coordinate? (For example, 8,2 becomes 802.)
use itertools::Itertools;
use num::integer::gcd;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

const INPUT: &str = include_str!("../input/day_10.txt");

type Point = (i32, i32);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Quadrant {
    TopToRight,
    RightToBottom,
    BottomToLeft,
    LeftToTop,
    SamePoint,
}

#[derive(Debug, Clone)]
struct Fraction {
    top: i32,
    bottom: i32,
    quadrant: Quadrant,
    division: f32,
}

impl Fraction {
    fn new(top: i32, bottom: i32) -> Fraction {
        let quadrant = match (top.cmp(&0), bottom.cmp(&0)) {
            (Ordering::Equal, Ordering::Less) => Quadrant::TopToRight,
            (Ordering::Greater, Ordering::Less) => Quadrant::TopToRight,
            (Ordering::Greater, Ordering::Equal) => Quadrant::RightToBottom,
            (Ordering::Greater, Ordering::Greater) => Quadrant::RightToBottom,
            (Ordering::Equal, Ordering::Greater) => Quadrant::BottomToLeft,
            (Ordering::Less, Ordering::Greater) => Quadrant::BottomToLeft,
            (Ordering::Less, Ordering::Equal) => Quadrant::LeftToTop,
            (Ordering::Less, Ordering::Less) => Quadrant::LeftToTop,
            (Ordering::Equal, Ordering::Equal) => Quadrant::SamePoint,
        };

        let mut division = match quadrant {
            Quadrant::TopToRight => top as f32 / bottom as f32,
            Quadrant::RightToBottom => bottom as f32 / top as f32,
            Quadrant::BottomToLeft => top as f32 / bottom as f32,
            Quadrant::LeftToTop => bottom as f32 / top as f32,
            Quadrant::SamePoint => 0.0,
        };
        division = division.abs();
        if division.is_infinite() {
            division = 0.0
        }

        Fraction { top, bottom, quadrant, division }
    }

    fn between(a: &Point, b: &Point) -> Fraction {
        let (a_x, a_y) = a;
        let (b_x, b_y) = b;

        let difference_x = b_x - a_x;
        let difference_y = b_y - a_y;

        let gcd = gcd(difference_x, difference_y);

        Fraction::new(difference_x / gcd, difference_y / gcd)
    }
}

impl Ord for Fraction {
    fn cmp(&self, other: &Self) -> Ordering {
        self.quadrant.cmp(&other.quadrant).then(
            self.division
                .partial_cmp(&other.division)
                .unwrap_or(Ordering::Equal),
        )
    }
}

impl PartialOrd for Fraction {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Fraction {}

impl PartialEq for Fraction {
    fn eq(&self, other: &Self) -> bool {
        self.top == other.top && self.bottom == other.bottom
    }
}

impl Hash for Fraction {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.top.hash(state);
        self.bottom.hash(state);
    }
}

pub fn run() {
    let asteroids = load_asteroids(INPUT);

    let fractions_map = calculate_fractions(&asteroids);
    let maximum_visible = fractions_map
        .values()
        .map(|fractions| fractions.into_iter().dedup().count())
        .max()
        .unwrap();
    println!(
        "The amount of asteroids to be detected from the best location is: {}",
        maximum_visible
    );
}

fn calculate_fractions(
    asteroids: &Vec<Point>,
) -> HashMap<Point, Vec<Fraction>> {
    let mut found_fractions = HashMap::new();
    for permutation in asteroids.iter().permutations(2) {
        match permutation[..] {
            [station, asteroid] => {
                let new_fraction = Fraction::between(station, asteroid);
                let existing_fractions =
                    found_fractions.entry(*station).or_insert(Vec::new());
                existing_fractions.push(new_fraction);
            }
            _ => panic!("Found an invalid permutation"),
        };
    }
    found_fractions.values_mut().for_each(|fractions| fractions.sort());
    found_fractions
}

fn load_asteroids(input: &str) -> Vec<Point> {
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
        let output = vec![(1, 0), (0, 1)];

        assert_eq!(load_asteroids(input), output);
    }

    #[test]
    fn test_load_asteroids_bigger() {
        let input = ".#..#\n.....\n#####\n....#\n...##";
        let output = vec![
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
        ];

        assert_eq!(load_asteroids(input), output);
    }

    #[test]
    fn test_calculate_fractions() {
        // .X..#
        // .....
        // #####
        // ....#
        // ...##
        let input = vec![
            (1, 0), // X
            (4, 0),
            (0, 2),
            (1, 2),
            (2, 2),
            (3, 2),
            (4, 2),
            (4, 3),
            (3, 4),
            (4, 4),
        ];

        let mut expected = vec![
            Fraction::new(1, 0),  // 4, 0
            Fraction::new(-1, 2), // 0, 2
            Fraction::new(0, 1),  // 1, 2
            Fraction::new(1, 2),  // 2, 2
            Fraction::new(1, 1),  // 3, 2
            Fraction::new(3, 2),  // 4, 2
            Fraction::new(1, 1),  // 4, 3
            Fraction::new(1, 2),  // 3, 4
            Fraction::new(3, 4),  // 4, 4
        ];

        let mut actual_output = calculate_fractions(&input);
        let first_point = actual_output.get_mut(&(1, 0)).unwrap();

        // see if the fraction for (1, 0) are all contained in the heap
        while let Some(fraction) = first_point.pop() {
            assert_ne!(
                expected.remove_item(&fraction),
                None,
                "Fraction {:?} was not expected in the heap",
                fraction
            );
        }
        assert!(
            expected.is_empty(),
            "Fractions {:?} were not in the heap",
            expected
        );
    }

    #[test]
    fn test_fraction_between_1_1() {
        let station = (0, 0);
        let asteroid = (2, 2);

        let fraction = Fraction::new(1, 1);

        assert_eq!(Fraction::between(&station, &asteroid), fraction);
    }

    #[test]
    fn test_fraction_between_negative_3_1() {
        let station = (12, 0);
        let asteroid = (0, 4);

        let fraction = Fraction::new(-3, 1);

        assert_eq!(Fraction::between(&station, &asteroid), fraction);
    }

    #[test]
    fn test_fraction_quadrant_top_right() {
        assert_eq!(
            Fraction::between(&(1, 1), &(1, 0)).quadrant, // 0, -1
            Quadrant::TopToRight
        );
        assert_eq!(
            Fraction::between(&(1, 1), &(2, 0)).quadrant, // 1, -1
            Quadrant::TopToRight
        );
    }

    #[test]
    fn test_fraction_quadrant_right_bottom() {
        assert_eq!(
            Fraction::between(&(1, 1), &(2, 1)).quadrant, // 1, 0
            Quadrant::RightToBottom
        );
        assert_eq!(
            Fraction::between(&(1, 1), &(2, 2)).quadrant, // 1, 1
            Quadrant::RightToBottom
        );
    }

    #[test]
    fn test_fraction_quadrant_bottom_left() {
        assert_eq!(
            Fraction::between(&(1, 1), &(1, 2)).quadrant, // 0, 1
            Quadrant::BottomToLeft
        );
        assert_eq!(
            Fraction::between(&(1, 1), &(0, 2)).quadrant, // -1, 1
            Quadrant::BottomToLeft
        );
    }

    #[test]
    fn test_fraction_quadrant_left_top() {
        assert_eq!(
            Fraction::between(&(1, 1), &(0, 1)).quadrant, // -1, 0
            Quadrant::LeftToTop
        );
        assert_eq!(
            Fraction::between(&(1, 1), &(0, 0)).quadrant, // -1, -1
            Quadrant::LeftToTop
        );
    }

    #[test]
    fn test_fraction_ordering_full_circle() {
        // ##123
        // #...4
        // #.X.5
        // #...6
        // ##987
        //
        // 67...
        // 5....
        // 4.X..
        // 3....
        // 21...
        let mut sorted = vec![
            Fraction::between(&(2, 2), &(2, 0)), // 0, -1
            Fraction::between(&(2, 2), &(0, 4)), // -1, 1
            Fraction::between(&(2, 2), &(4, 3)), // 2, 1
            Fraction::between(&(2, 2), &(4, 2)), // 1, 0
            Fraction::between(&(2, 2), &(3, 0)), // 1, -2
            Fraction::between(&(2, 2), &(2, 4)), // 0, 1
            Fraction::between(&(2, 2), &(1, 4)), // -1, 2
            Fraction::between(&(2, 2), &(3, 4)), // 1, 2
            Fraction::between(&(2, 2), &(0, 3)), // -2, 1
            Fraction::between(&(2, 2), &(0, 2)), // -1, 0
            Fraction::between(&(2, 2), &(0, 1)), // -2, -1
            Fraction::between(&(2, 2), &(4, 4)), // 1, 1
            Fraction::between(&(2, 2), &(4, 0)), // 1, -1
            Fraction::between(&(2, 2), &(4, 1)), // 2, -1
            Fraction::between(&(2, 2), &(0, 0)), // -1, -1
            Fraction::between(&(2, 2), &(1, 0)), // -1, -2
        ];
        sorted.sort();

        let expected = vec![
            Fraction::new(0, -1),
            Fraction::new(1, -2),
            Fraction::new(1, -1),
            Fraction::new(2, -1),
            Fraction::new(1, 0),
            Fraction::new(2, 1),
            Fraction::new(1, 1),
            Fraction::new(1, 2),
            Fraction::new(0, 1),
            Fraction::new(-1, 2),
            Fraction::new(-1, 1),
            Fraction::new(-2, 1),
            Fraction::new(-1, 0),
            Fraction::new(-2, -1),
            Fraction::new(-1, -1),
            Fraction::new(-1, -2),
        ];

        assert_eq!(sorted, expected);
    }

    #[test]
    fn test_fraction_equality() {
        assert_eq!(Fraction::between(&(0, 0), &(1, 1)), Fraction::new(1, 1));
    }

    #[test]
    fn test_fraction_ordering_inside_quadrant() {
        // .24.....
        // 13.67..9
        // .5.8....
        // X.......
        let mut sorted = vec![
            Fraction::between(&(0, 3), &(0, 1)),
            Fraction::between(&(0, 3), &(2, 0)),
            Fraction::between(&(0, 3), &(1, 1)),
            Fraction::between(&(0, 3), &(4, 1)),
            Fraction::between(&(0, 3), &(3, 1)),
            Fraction::between(&(0, 3), &(1, 0)),
            Fraction::between(&(0, 3), &(7, 1)),
            Fraction::between(&(0, 3), &(3, 2)),
            Fraction::between(&(0, 3), &(1, 2)),
        ];
        sorted.sort();

        let expected = vec![
            Fraction::between(&(0, 3), &(0, 1)),
            Fraction::between(&(0, 3), &(1, 0)),
            Fraction::between(&(0, 3), &(1, 1)),
            Fraction::between(&(0, 3), &(2, 0)),
            Fraction::between(&(0, 3), &(1, 2)),
            Fraction::between(&(0, 3), &(3, 1)),
            Fraction::between(&(0, 3), &(4, 1)),
            Fraction::between(&(0, 3), &(3, 2)),
            Fraction::between(&(0, 3), &(7, 1)),
        ];

        assert_eq!(sorted, expected);
    }
}
