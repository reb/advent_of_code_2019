/// --- Day 14: Space Stoichiometry ---
///
/// As you approach the rings of Saturn, your ship's low fuel indicator turns
/// on. There isn't any fuel here, but the rings have plenty of raw material.
/// Perhaps your ship's Inter-Stellar Refinery Union brand nanofactory can turn
/// these raw materials into fuel.
///
/// You ask the nanofactory to produce a list of the reactions it can perform
/// that are relevant to this process (your puzzle input). Every reaction turns
/// some quantities of specific input chemicals into some quantity of an output
/// chemical. Almost every chemical is produced by exactly one reaction; the
/// only exception, ORE, is the raw material input to the entire process and is
/// not produced by a reaction.
///
/// You just need to know how much ORE you'll need to collect before you can
/// produce one unit of FUEL.
///
/// Each reaction gives specific quantities for its inputs and output; reactions
/// cannot be partially run, so only whole integer multiples of these quantities
/// can be used. (It's okay to have leftover chemicals when you're done,
/// though.) For example, the reaction 1 A, 2 B, 3 C => 2 D means that exactly 2
/// units of chemical D can be produced by consuming exactly 1 A, 2 B and 3 C.
/// You can run the full reaction as many times as necessary; for example, you
/// could produce 10 D by consuming 5 A, 10 B, and 15 C.
///
/// Suppose your nanofactory produces the following list of reactions:
///
/// 10 ORE => 10 A
/// 1 ORE => 1 B
/// 7 A, 1 B => 1 C
/// 7 A, 1 C => 1 D
/// 7 A, 1 D => 1 E
/// 7 A, 1 E => 1 FUEL
///
/// The first two reactions use only ORE as inputs; they indicate that you can
/// produce as much of chemical A as you want (in increments of 10 units, each
/// 10 costing 10 ORE) and as much of chemical B as you want (each costing 1
/// ORE). To produce 1 FUEL, a total of 31 ORE is required: 1 ORE to produce 1
/// B, then 30 more ORE to produce the 7 + 7 + 7 + 7 = 28 A (with 2 extra A
/// wasted) required in the reactions to convert the B into C, C into D, D into
/// E, and finally E into FUEL. (30 A is produced because its reaction requires
/// that it is created in increments of 10.)
///
/// Or, suppose you have the following list of reactions:
///
/// 9 ORE => 2 A
/// 8 ORE => 3 B
/// 7 ORE => 5 C
/// 3 A, 4 B => 1 AB
/// 5 B, 7 C => 1 BC
/// 4 C, 1 A => 1 CA
/// 2 AB, 3 BC, 4 CA => 1 FUEL
///
/// The above list of reactions requires 165 ORE to produce 1 FUEL:
///
///     Consume 45 ORE to produce 10 A.
///     Consume 64 ORE to produce 24 B.
///     Consume 56 ORE to produce 40 C.
///     Consume 6 A, 8 B to produce 2 AB.
///     Consume 15 B, 21 C to produce 3 BC.
///     Consume 16 C, 4 A to produce 4 CA.
///     Consume 2 AB, 3 BC, 4 CA to produce 1 FUEL.
///
/// Here are some larger examples:
///
///     13312 ORE for 1 FUEL:
///
///     157 ORE => 5 NZVS
///     165 ORE => 6 DCFZ
///     44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
///     12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
///     179 ORE => 7 PSHF
///     177 ORE => 5 HKGWZ
///     7 DCFZ, 7 PSHF => 2 XJWVT
///     165 ORE => 2 GPVTF
///     3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT
///
///     180697 ORE for 1 FUEL:
///
///     2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
///     17 NVRVD, 3 JNWZP => 8 VPVL
///     53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL
///     22 VJHF, 37 MNCFX => 5 FWMGM
///     139 ORE => 4 NVRVD
///     144 ORE => 7 JNWZP
///     5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC
///     5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV
///     145 ORE => 6 MNCFX
///     1 NVRVD => 8 CXFTF
///     1 VJHF, 6 MNCFX => 4 RFSQX
///     176 ORE => 6 VJHF
///
///     2210736 ORE for 1 FUEL:
///
///     171 ORE => 8 CNZTR
///     7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
///     114 ORE => 4 BHXH
///     14 VRPVC => 6 BMBT
///     6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
///     6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
///     15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
///     13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
///     5 BMBT => 4 WPTQ
///     189 ORE => 9 KTJDG
///     1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
///     12 VRPVC, 27 CNZTR => 2 XDBXC
///     15 KTJDG, 12 BHXH => 5 XCVML
///     3 BHXH, 2 VRPVC => 7 MZWV
///     121 ORE => 7 VRPVC
///     7 XCVML => 6 RJRHP
///     5 BHXH, 4 VRPVC => 5 LTCX
///
/// Given the list of reactions in your puzzle input, what is the minimum amount
/// of ORE required to produce exactly 1 FUEL?
/// --- Part Two ---
///
/// After collecting ORE for a while, you check your cargo hold: 1 trillion
/// (1000000000000) units of ORE.
///
/// With that much ore, given the examples above:
///
///     The 13312 ORE-per-FUEL example could produce 82892753 FUEL.
///     The 180697 ORE-per-FUEL example could produce 5586022 FUEL.
///     The 2210736 ORE-per-FUEL example could produce 460664 FUEL.
///
/// Given 1 trillion ORE, what is the maximum amount of FUEL you can produce?
use itertools::Itertools;
use num::Integer;
use std::collections::HashMap;

const INPUT: &str = include_str!("../input/day_14.txt");

pub fn run() {
    let formulas = load_formulas(INPUT);
    let mut storage = Storage::new();

    // force to produce 1 FUEL
    storage.insert("FUEL", -1);

    storage = produce_until_none_missing(storage, &formulas);

    let ore_per_fuel = -storage.get(&"ORE").expect("the ORE entry to be there");
    println!("The amount of ORE needed to produce 1 FUEL is: {}", ore_per_fuel);

    // set the amount of ore available to a trillion
    let ore_available = 1_000_000_000_000;
    storage = Storage::new();
    storage.insert("ORE", ore_available);

    let mut fuel_produced = 0;

    // execute fuel production in batches
    let mut next_fuel_batch;
    loop {
        // set a batch size that will at least be reached
        next_fuel_batch = storage.get("ORE").unwrap() / ore_per_fuel;
        // make sure the do at least 1
        if next_fuel_batch == 0 {
            next_fuel_batch = 1;
        }
        *storage.entry("FUEL").or_insert(0) -= next_fuel_batch;

        // produce the batch
        storage = produce_until_none_missing(storage, &formulas);
        if storage.get("ORE").expect("the ORE entry to be there") < &0 {
            // stop if it goes beyond the capacity of ORE
            break;
        }
        fuel_produced += next_fuel_batch;
    }
    println!(
        "The amount of FUEL that can be produced with a trillion ORE is: {}",
        fuel_produced
    );
}

type Storage<'a> = HashMap<&'a str, i64>;

#[derive(Debug, PartialEq)]
struct Component<'a> {
    quantity: i64,
    name: &'a str,
}

#[derive(Debug, PartialEq)]
struct Formula<'a> {
    requirements: Vec<Component<'a>>,
    result: Component<'a>,
}

impl<'a> Formula<'a> {
    fn from(requirements: &'a str, result: &'a str) -> Formula<'a> {
        Formula {
            result: Component::from(result),
            requirements: requirements
                .split(',')
                .map(|component| Component::from(component))
                .collect(),
        }
    }

    /// Produce at least the amount given
    fn produce(&self, amount: i64, storage: &mut Storage<'a>) {
        let times = match amount.div_rem(&self.result.quantity) {
            (quotient, 0) => quotient,
            (quotient, _) => quotient + 1,
        };
        *(storage.entry(&self.result.name).or_insert(0)) +=
            times * self.result.quantity;
        for component in self.requirements.iter() {
            *(storage.entry(&component.name).or_insert(0)) -=
                times * component.quantity;
        }
    }
}

impl<'a> Component<'a> {
    fn from(string: &'a str) -> Component<'a> {
        let (quantity, name) = string
            .trim()
            .split(' ')
            .tuples()
            .map(|(quantity, name)| (quantity.parse().expect("an i64"), name))
            .next()
            .unwrap();

        Component { quantity, name }
    }
}

fn produce_until_none_missing<'a>(
    mut storage: Storage<'a>,
    formulas: &'a Vec<Formula<'a>>,
) -> Storage<'a> {
    loop {
        let new_storage = produce_missing(storage.clone(), &formulas);
        if new_storage == storage {
            return new_storage;
        }
        storage = new_storage;
    }
}

fn produce_missing<'a>(
    mut storage: Storage<'a>,
    formulas: &'a Vec<Formula<'a>>,
) -> Storage<'a> {
    // collect all components that are below 0
    let components_missing = storage
        .iter()
        .filter(|(_, amount)| amount < &&0)
        .map(|(name, amount)| (*name, *amount))
        .collect::<Vec<_>>();

    for (name, amount) in components_missing.iter() {
        match find_formula(name, formulas) {
            Some(formula) => formula.produce(-amount, &mut storage),
            None => {}
        }
    }

    storage
}

fn find_formula<'a>(
    name: &str,
    formulas: &'a Vec<Formula<'a>>,
) -> Option<&'a Formula<'a>> {
    formulas.iter().find(|formula| formula.result.name == name)
}

fn load_formulas(input: &str) -> Vec<Formula> {
    input
        .lines()
        .map(|line| {
            let (requirements, result) =
                line.split("=>").tuples().next().unwrap();
            Formula::from(requirements, result)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_formulas_1() {
        let input = "10 ORE => 10 A\n\
                     1 ORE => 1 B\n\
                     7 A, 1 B => 1 C\n\
                     7 A, 1 C => 1 D\n\
                     7 A, 1 D => 1 E\n\
                     7 A, 1 E => 1 FUEL\n";

        let expected_formulas = vec![
            Formula {
                requirements: vec![Component { quantity: 10, name: "ORE" }],
                result: Component { quantity: 10, name: "A" },
            },
            Formula {
                requirements: vec![Component { quantity: 1, name: "ORE" }],
                result: Component { quantity: 1, name: "B" },
            },
            Formula {
                requirements: vec![
                    Component { quantity: 7, name: "A" },
                    Component { quantity: 1, name: "B" },
                ],
                result: Component { quantity: 1, name: "C" },
            },
            Formula {
                requirements: vec![
                    Component { quantity: 7, name: "A" },
                    Component { quantity: 1, name: "C" },
                ],
                result: Component { quantity: 1, name: "D" },
            },
            Formula {
                requirements: vec![
                    Component { quantity: 7, name: "A" },
                    Component { quantity: 1, name: "D" },
                ],
                result: Component { quantity: 1, name: "E" },
            },
            Formula {
                requirements: vec![
                    Component { quantity: 7, name: "A" },
                    Component { quantity: 1, name: "E" },
                ],
                result: Component { quantity: 1, name: "FUEL" },
            },
        ];

        assert_eq!(load_formulas(input), expected_formulas);
    }

    #[test]
    fn test_load_formulas_2() {
        let input =
            "157 ORE => 5 NZVS\n\
             165 ORE => 6 DCFZ\n\
             44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL\n\
             12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ\n\
             179 ORE => 7 PSHF\n\
             177 ORE => 5 HKGWZ\n\
             7 DCFZ, 7 PSHF => 2 XJWVT\n\
             165 ORE => 2 GPVTF\n\
             3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT";

        let expected_formulas = vec![
            Formula {
                requirements: vec![Component { quantity: 157, name: "ORE" }],
                result: Component { quantity: 5, name: "NZVS" },
            },
            Formula {
                requirements: vec![Component { quantity: 165, name: "ORE" }],
                result: Component { quantity: 6, name: "DCFZ" },
            },
            Formula {
                requirements: vec![
                    Component { quantity: 44, name: "XJWVT" },
                    Component { quantity: 5, name: "KHKGT" },
                    Component { quantity: 1, name: "QDVJ" },
                    Component { quantity: 29, name: "NZVS" },
                    Component { quantity: 9, name: "GPVTF" },
                    Component { quantity: 48, name: "HKGWZ" },
                ],
                result: Component { quantity: 1, name: "FUEL" },
            },
            Formula {
                requirements: vec![
                    Component { quantity: 12, name: "HKGWZ" },
                    Component { quantity: 1, name: "GPVTF" },
                    Component { quantity: 8, name: "PSHF" },
                ],
                result: Component { quantity: 9, name: "QDVJ" },
            },
            Formula {
                requirements: vec![Component { quantity: 179, name: "ORE" }],
                result: Component { quantity: 7, name: "PSHF" },
            },
            Formula {
                requirements: vec![Component { quantity: 177, name: "ORE" }],
                result: Component { quantity: 5, name: "HKGWZ" },
            },
            Formula {
                requirements: vec![
                    Component { quantity: 7, name: "DCFZ" },
                    Component { quantity: 7, name: "PSHF" },
                ],
                result: Component { quantity: 2, name: "XJWVT" },
            },
            Formula {
                requirements: vec![Component { quantity: 165, name: "ORE" }],
                result: Component { quantity: 2, name: "GPVTF" },
            },
            Formula {
                requirements: vec![
                    Component { quantity: 3, name: "DCFZ" },
                    Component { quantity: 7, name: "NZVS" },
                    Component { quantity: 5, name: "HKGWZ" },
                    Component { quantity: 10, name: "PSHF" },
                ],
                result: Component { quantity: 8, name: "KHKGT" },
            },
        ];

        assert_eq!(load_formulas(input), expected_formulas);
    }

    #[test]
    fn test_formula_from() {
        let requirements =
            "53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV ";
        let result = " 1 FUEL";

        let expected_formula = Formula {
            requirements: vec![
                Component { quantity: 53, name: "STKFG" },
                Component { quantity: 6, name: "MNCFX" },
                Component { quantity: 46, name: "VJHF" },
                Component { quantity: 81, name: "HVMC" },
                Component { quantity: 68, name: "CXFTF" },
                Component { quantity: 25, name: "GNMV" },
            ],
            result: Component { quantity: 1, name: "FUEL" },
        };

        assert_eq!(Formula::from(requirements, result), expected_formula);
    }

    #[test]
    fn test_produce_missing() {
        let mut storage = Storage::new();
        // require 4 of C
        storage.insert("C", -4);
        let formulas = vec![
            Formula {
                requirements: vec![Component { quantity: 1, name: "0" }],
                result: Component { quantity: 1, name: "A" },
            },
            Formula {
                requirements: vec![Component { quantity: 1, name: "0" }],
                result: Component { quantity: 1, name: "B" },
            },
            Formula {
                requirements: vec![
                    Component { quantity: 25, name: "A" },
                    Component { quantity: 3, name: "B" },
                ],
                result: Component { quantity: 3, name: "C" },
            },
        ];

        // first round should only produce for the missing "C"
        let mut expected_storage = Storage::new();
        expected_storage.insert("A", -50);
        expected_storage.insert("B", -6);
        expected_storage.insert("C", 2);

        storage = produce_missing(storage, &formulas);
        assert_eq!(storage, expected_storage);

        // second round should produce the missing "A" and "B"
        expected_storage.insert("A", 0);
        expected_storage.insert("B", 0);
        expected_storage.insert("0", -56);

        storage = produce_missing(storage, &formulas);
        assert_eq!(storage, expected_storage);
    }
}
