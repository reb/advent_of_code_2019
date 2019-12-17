/// --- Day 6: Universal Orbit Map ---
///
/// You've landed at the Universal Orbit Map facility on Mercury. Because
/// navigation in space often involves transferring between orbits, the orbit
/// maps here are useful for finding efficient routes between, for example, you
/// and Santa. You download a map of the local orbits (your puzzle input).
///
/// Except for the universal Center of Mass (COM), every object in space is in
/// orbit around exactly one other object. An orbit looks roughly like this:
///
///                   \
///                    \
///                     |
///                     |
/// AAA--> o            o <--BBB
///                     |
///                     |
///                    /
///                   /
///
/// In this diagram, the object BBB is in orbit around AAA. The path that BBB
/// takes around AAA (drawn with lines) is only partly shown. In the map data,
/// this orbital relationship is written AAA)BBB, which means "BBB is in orbit
/// around AAA".
///
/// Before you use your map data to plot a course, you need to make sure it
/// wasn't corrupted during the download. To verify maps, the Universal Orbit
/// Map facility uses orbit count checksums - the total number of direct orbits
/// (like the one shown above) and indirect orbits.
///
/// Whenever A orbits B and B orbits C, then A indirectly orbits C. This chain
/// can be any number of objects long: if A orbits B, B orbits C, and C orbits
/// D, then A indirectly orbits D.
///
/// For example, suppose you have the following map:
///
/// COM)B
/// B)C
/// C)D
/// D)E
/// E)F
/// B)G
/// G)H
/// D)I
/// E)J
/// J)K
/// K)L
///
/// Visually, the above map of orbits looks like this:
///
///         G - H       J - K - L
///        /           /
/// COM - B - C - D - E - F
///                \
///                 I
///
/// In this visual representation, when two objects are connected by a line, the
/// one on the right directly orbits the one on the left.
///
/// Here, we can count the total number of orbits as follows:
///
///     D directly orbits C and indirectly orbits B and COM, a total of 3 orbits.
///     L directly orbits K and indirectly orbits J, E, D, C, B, and COM, a
///     total of 7 orbits.
///     COM orbits nothing.
///
/// The total number of direct and indirect orbits in this example is 42.
///
/// What is the total number of direct and indirect orbits in your map data?
/// --- Part Two ---
///
/// Now, you just need to figure out how many orbital transfers you (YOU) need
/// to take to get to Santa (SAN).
///
/// You start at the object YOU are orbiting; your destination is the object SAN
/// is orbiting. An orbital transfer lets you move from any object to an object
/// orbiting or orbited by that object.
///
/// For example, suppose you have the following map:
///
/// COM)B
/// B)C
/// C)D
/// D)E
/// E)F
/// B)G
/// G)H
/// D)I
/// E)J
/// J)K
/// K)L
/// K)YOU
/// I)SAN
///
/// Visually, the above map of orbits looks like this:
///
///                           YOU
///                          /
///         G - H       J - K - L
///        /           /
/// COM - B - C - D - E - F
///                \
///                 I - SAN
///
/// In this example, YOU are in orbit around K, and SAN is in orbit around I. To
/// move from K to I, a minimum of 4 orbital transfers are required:
///
///     K to J
///     J to E
///     E to D
///     D to I
///
/// Afterward, the map of orbits looks like this:
///
///         G - H       J - K - L
///        /           /
/// COM - B - C - D - E - F
///                \
///                 I - SAN
///                  \
///                   YOU
///
/// What is the minimum number of orbital transfers required to move from the
/// object YOU are orbiting to the object SAN is orbiting? (Between the objects
/// they are orbiting - not between YOU and SAN.)
use petgraph::algo::dijkstra;
use petgraph::graphmap::GraphMap;
use petgraph::visit::{Dfs, Reversed};
use petgraph::{Directed, EdgeType, Undirected};
type Graph<'a, Ty> = GraphMap<&'a str, (), Ty>;

const INPUT: &str = include_str!("../input/day_06.txt");

pub fn run() {
    let orbits = get_input::<Directed>();

    let total = total_orbits(&orbits);
    println!(
        "The total number of direct and indirect orbits is: {}",
        total
    );

    let undirected_orbits = get_input::<Undirected>();

    match transfers_needed(&undirected_orbits, "YOU", "SAN") {
        Some(number) => println!(
            "The orbital transfers required to get \"YOU\" to \"SAN\" is : {}",
            number
        ),
        None => println!("No connection found between \"YOU\" and \"SAN\""),
    };
}

fn get_input<'a, Ty: EdgeType>() -> Graph<'a, Ty> {
    let connecting_pairs: Vec<(&str, &str)> = INPUT
        .lines()
        .map(|line| line.trim().split(')').collect())
        .map(|elements: Vec<&str>| match &elements[..2] {
            &[from, to] => (from, to),
            _ => panic!("There were no 2 elements on a line in the input"),
        })
        .collect();
    Graph::<Ty>::from_edges(connecting_pairs)
}

fn total_orbits(graph: &Graph<Directed>) -> u32 {
    if let Some(root) = find_root(graph) {
        dijkstra(graph, root, None, |_| 1)
            .iter()
            .map(|(_, distance)| distance)
            .sum()
    } else {
        0
    }
}

fn find_root<'a>(graph: &Graph<'a, Directed>) -> Option<&'a str> {
    // grab any node
    let node = graph.nodes().next();
    if node.is_none() {
        return None;
    }

    // use it to initialize a depth-first search iterator
    let mut depth_first_search = Dfs::new(Reversed(graph), node.unwrap());

    // walk up the tree to find the root
    let mut root_node = None;
    while let Some(next_node) = depth_first_search.next(Reversed(graph)) {
        root_node = Some(next_node);
    }
    root_node
}

fn transfers_needed(
    graph: &Graph<Undirected>,
    from: &str,
    to: &str,
) -> Option<u32> {
    if let Some(distance) =
        dijkstra(graph, from, Some(to), |_| 1).iter().find_map(
            |(&node, &distance)| {
                if node == to {
                    Some(distance)
                } else {
                    None
                }
            },
        )
    {
        Some((distance - 2) as u32)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total_orbits() {
        let input = Graph::<Directed>::from_edges(&[
            ("A", "B"),
            ("B", "C"),
            ("C", "D"),
            ("D", "E"),
            ("E", "F"),
            ("B", "G"),
            ("G", "H"),
            ("D", "I"),
            ("E", "J"),
            ("J", "K"),
            ("K", "L"),
        ]);
        assert_eq!(total_orbits(&input), 42);
    }

    #[test]
    fn test_find_root() {
        let input = Graph::<Directed>::from_edges(&[
            ("COM", "B"),
            ("B", "C"),
            ("C", "D"),
        ]);
        assert_eq!(find_root(&input), Some("COM"));
    }

    #[test]
    fn test_transfers_needed() {
        let input = Graph::<Undirected>::from_edges(&[
            ("COM", "B"),
            ("B", "C"),
            ("C", "D"),
            ("D", "E"),
            ("E", "F"),
            ("B", "G"),
            ("G", "H"),
            ("D", "I"),
            ("E", "J"),
            ("J", "K"),
            ("K", "L"),
            ("K", "YOU"),
            ("I", "SAN"),
        ]);
        assert_eq!(transfers_needed(&input, "YOU", "SAN"), Some(4));
    }
}
