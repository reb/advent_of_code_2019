/// --- Day 4: Secure Container ---
///
/// You arrive at the Venus fuel depot only to discover it's protected by a
/// password. The Elves had written the password on a sticky note, but someone
/// threw it out.
///
/// However, they do remember a few key facts about the password:
///
///     It is a six-digit number.
///     The value is within the range given in your puzzle input.
///     Two adjacent digits are the same (like 22 in 122345).
///     Going from left to right, the digits never decrease; they only ever
///     increase or stay the same (like 111123 or 135679).
///
/// Other than the range rule, the following are true:
///
///     111111 meets these criteria (double 11, never decreases).
///     223450 does not meet these criteria (decreasing pair of digits 50).
///     123789 does not meet these criteria (no double).
///
/// How many different passwords within the range given in your puzzle input
/// meet these criteria?
///
/// Your puzzle input is 264360-746325.

struct Digits {
    number: u32,
    divisor: u32,
}

impl Digits {
    fn new(number: u32) -> Self {
        let mut divisor = 1;
        while number / divisor >= 10 {
            divisor *= 10;
        }

        Digits {
            number: number,
            divisor: divisor,
        }
    }
}

impl Iterator for Digits {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.divisor == 0 {
            None
        } else {
            let next_digit = Some(self.number / self.divisor);
            self.number %= self.divisor;
            self.divisor /= 10;
            next_digit
        }
    }
}

pub fn run() {
    let start = 264360;
    let end = 746325;

    let mut valid_passwords = 0;
    let mut valid_stricter_passwords = 0;

    for number in start..=end {
        if is_valid(number) {
            valid_passwords += 1;
        }
        if is_valid_with_stricter_criteria(number) {
            valid_stricter_passwords += 1;
        }
    }

    println!(
        "Amount of passwords meeting the criteria in the given range: {}",
        valid_passwords);
    println!(
        "Amount of passwords meeting the stricter criteria in the given range: {}",
        valid_stricter_passwords);
}

fn is_valid(number: u32) -> bool {
    let mut iter = Digits::new(number).peekable();
    let mut two_same_adjacent = false;
    while let Some(digit) = iter.next() {
        if let Some(next_digit) = iter.peek() {
            if next_digit < &digit {
                return false;
            }

            if next_digit == &digit {
                two_same_adjacent = true;
            }
        }
    }

    // all digits increase in value, validity depends on two same adjacent digits
    two_same_adjacent
}

fn is_valid_with_stricter_criteria(number: u32) -> bool {
    let mut iter = Digits::new(number).peekable();
    let mut two_same_adjacent = false;
    let mut amount_of_equal_digits = None;
    while let Some(digit) = iter.next() {
        if let Some(next_digit) = iter.peek() {
            if next_digit < &digit {
                return false;
            }

            if next_digit == &digit {
                amount_of_equal_digits = match amount_of_equal_digits {
                    Some(amount) => Some(amount+1),
                    None => Some(2),
                }
            } else if let Some(amount) = amount_of_equal_digits {
                if amount == 2 {
                    two_same_adjacent = true;
                }
                amount_of_equal_digits = None;
            }
        }
    }

    // all digits increase in value, validity depends on two same adjacent digits
    two_same_adjacent || amount_of_equal_digits == Some(2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_1() {
        let input = 111111;
        assert!(is_valid(input));
    }

    #[test]
    fn test_is_valid_2() {
        let input = 223450;
        assert!(!is_valid(input));
    }

    #[test]
    fn test_is_valid_3() {
        let input = 123789;
        assert!(!is_valid(input));
    }

    #[test]
    fn test_is_valid_with_stricter_criteria_1() {
        let input = 112233;
        assert!(is_valid_with_stricter_criteria(input));
    }

    #[test]
    fn test_is_valid_with_stricter_criteria_2() {
        let input = 123444;
        assert!(!is_valid_with_stricter_criteria(input));
    }

    #[test]
    fn test_is_valid_with_stricter_criteria_3() {
        let input = 111122;
        assert!(is_valid_with_stricter_criteria(input));
    }

    #[test]
    fn test_is_valid_with_stricter_criteria_4() {
        let input = 266880;
        assert!(!is_valid_with_stricter_criteria(input));
    }
}
