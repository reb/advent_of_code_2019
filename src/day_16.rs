/// --- Day 16: Flawed Frequency Transmission ---
///
/// You're 3/4ths of the way through the gas giants. Not only do roundtrip signals to Earth take
/// five hours, but the signal quality is quite bad as well. You can clean up the signal with the
/// Flawed Frequency Transmission algorithm, or FFT.
///
/// As input, FFT takes a list of numbers. In the signal you received (your puzzle input), each
/// number is a single digit: data like 15243 represents the sequence 1, 5, 2, 4, 3.
///
/// FFT operates in repeated phases. In each phase, a new list is constructed with the same length
/// as the input list. This new list is also used as the input for the next phase.
///
/// Each element in the new list is built by multiplying every value in the input list by a value
/// in a repeating pattern and then adding up the results. So, if the input list were 9, 8, 7, 6, 5
/// and the pattern for a given element were 1, 2, 3, the result would be 9*1 + 8*2 + 7*3 + 6*1 +
/// 5*2 (with each input element on the left and each value in the repeating pattern on the right
/// of each multiplication). Then, only the ones digit is kept: 38 becomes 8, -17 becomes 7, and so
/// on.
///
/// While each element in the output array uses all of the same input array elements, the actual
/// repeating pattern to use depends on which output element is being calculated. The base pattern
/// is 0, 1, 0, -1. Then, repeat each value in the pattern a number of times equal to the position
/// in the output list being considered. Repeat once for the first element, twice for the second
/// element, three times for the third element, and so on. So, if the third element of the output
/// list is being calculated, repeating the values would produce: 0, 0, 0, 1, 1, 1, 0, 0, 0, -1,
/// -1, -1.
///
/// When applying the pattern, skip the very first value exactly once. (In other words, offset the
/// whole pattern left by one.) So, for the second element of the output list, the actual pattern
/// used would be: 0, 1, 1, 0, 0, -1, -1, 0, 0, 1, 1, 0, 0, -1, -1, ....
///
/// After using this process to calculate each element of the output list, the phase is complete,
/// and the output list of this phase is used as the new input list for the next phase, if any.
///
/// Given the input signal 12345678, below are four phases of FFT. Within each phase, each output
/// digit is calculated on a single line with the result at the far right; each multiplication
/// operation shows the input digit on the left and the pattern value on the right:
///
/// Input signal: 12345678
///
/// 1*1  + 2*0  + 3*-1 + 4*0  + 5*1  + 6*0  + 7*-1 + 8*0  = 4
/// 1*0  + 2*1  + 3*1  + 4*0  + 5*0  + 6*-1 + 7*-1 + 8*0  = 8
/// 1*0  + 2*0  + 3*1  + 4*1  + 5*1  + 6*0  + 7*0  + 8*0  = 2
/// 1*0  + 2*0  + 3*0  + 4*1  + 5*1  + 6*1  + 7*1  + 8*0  = 2
/// 1*0  + 2*0  + 3*0  + 4*0  + 5*1  + 6*1  + 7*1  + 8*1  = 6
/// 1*0  + 2*0  + 3*0  + 4*0  + 5*0  + 6*1  + 7*1  + 8*1  = 1
/// 1*0  + 2*0  + 3*0  + 4*0  + 5*0  + 6*0  + 7*1  + 8*1  = 5
/// 1*0  + 2*0  + 3*0  + 4*0  + 5*0  + 6*0  + 7*0  + 8*1  = 8
///
/// After 1 phase: 48226158
///
/// 4*1  + 8*0  + 2*-1 + 2*0  + 6*1  + 1*0  + 5*-1 + 8*0  = 3
/// 4*0  + 8*1  + 2*1  + 2*0  + 6*0  + 1*-1 + 5*-1 + 8*0  = 4
/// 4*0  + 8*0  + 2*1  + 2*1  + 6*1  + 1*0  + 5*0  + 8*0  = 0
/// 4*0  + 8*0  + 2*0  + 2*1  + 6*1  + 1*1  + 5*1  + 8*0  = 4
/// 4*0  + 8*0  + 2*0  + 2*0  + 6*1  + 1*1  + 5*1  + 8*1  = 0
/// 4*0  + 8*0  + 2*0  + 2*0  + 6*0  + 1*1  + 5*1  + 8*1  = 4
/// 4*0  + 8*0  + 2*0  + 2*0  + 6*0  + 1*0  + 5*1  + 8*1  = 3
/// 4*0  + 8*0  + 2*0  + 2*0  + 6*0  + 1*0  + 5*0  + 8*1  = 8
///
/// After 2 phases: 34040438
///
/// 3*1  + 4*0  + 0*-1 + 4*0  + 0*1  + 4*0  + 3*-1 + 8*0  = 0
/// 3*0  + 4*1  + 0*1  + 4*0  + 0*0  + 4*-1 + 3*-1 + 8*0  = 3
/// 3*0  + 4*0  + 0*1  + 4*1  + 0*1  + 4*0  + 3*0  + 8*0  = 4
/// 3*0  + 4*0  + 0*0  + 4*1  + 0*1  + 4*1  + 3*1  + 8*0  = 1
/// 3*0  + 4*0  + 0*0  + 4*0  + 0*1  + 4*1  + 3*1  + 8*1  = 5
/// 3*0  + 4*0  + 0*0  + 4*0  + 0*0  + 4*1  + 3*1  + 8*1  = 5
/// 3*0  + 4*0  + 0*0  + 4*0  + 0*0  + 4*0  + 3*1  + 8*1  = 1
/// 3*0  + 4*0  + 0*0  + 4*0  + 0*0  + 4*0  + 3*0  + 8*1  = 8
///
/// After 3 phases: 03415518
///
/// 0*1  + 3*0  + 4*-1 + 1*0  + 5*1  + 5*0  + 1*-1 + 8*0  = 0
/// 0*0  + 3*1  + 4*1  + 1*0  + 5*0  + 5*-1 + 1*-1 + 8*0  = 1
/// 0*0  + 3*0  + 4*1  + 1*1  + 5*1  + 5*0  + 1*0  + 8*0  = 0
/// 0*0  + 3*0  + 4*0  + 1*1  + 5*1  + 5*1  + 1*1  + 8*0  = 2
/// 0*0  + 3*0  + 4*0  + 1*0  + 5*1  + 5*1  + 1*1  + 8*1  = 9
/// 0*0  + 3*0  + 4*0  + 1*0  + 5*0  + 5*1  + 1*1  + 8*1  = 4
/// 0*0  + 3*0  + 4*0  + 1*0  + 5*0  + 5*0  + 1*1  + 8*1  = 9
/// 0*0  + 3*0  + 4*0  + 1*0  + 5*0  + 5*0  + 1*0  + 8*1  = 8
///
/// After 4 phases: 01029498
///
/// Here are the first eight digits of the final output list after 100 phases for some larger
/// inputs:
///
///     80871224585914546619083218645595 becomes 24176176.
///     19617804207202209144916044189917 becomes 73745418.
///     69317163492948606335995924319873 becomes 52432133.
///
/// After 100 phases of FFT, what are the first eight digits in the final output list?
///
/// --- Part Two ---
///
/// Now that your FFT is working, you can decode the real signal.
///
/// The real signal is your puzzle input repeated 10000 times. Treat this new signal as a single
/// input list. Patterns are still calculated as before, and 100 phases of FFT are still applied.
///
/// The first seven digits of your initial input signal also represent the message offset. The
/// message offset is the location of the eight-digit message in the final output list.
/// Specifically, the message offset indicates the number of digits to skip before reading the
/// eight-digit message. For example, if the first seven digits of your initial input signal were
/// 1234567, the eight-digit message would be the eight digits after skipping 1,234,567 digits of
/// the final output list. Or, if the message offset were 7 and your final output list were
/// 98765432109876543210, the eight-digit message would be 21098765. (Of course, your real message
/// offset will be a seven-digit number, not a one-digit number like 7.)
///
/// Here is the eight-digit message in the final output list after 100 phases. The message offset
/// given in each input has been highlighted. (Note that the inputs given below are repeated 10000
/// times to find the actual starting input lists.)
///
///     03036732577212944063491565474664 becomes 84462026.
///     02935109699940807407585447034323 becomes 78725270.
///     03081770884921959731165446850517 becomes 53553731.
///
/// After repeating your input signal 10000 times and running 100 phases of FFT, what is the
/// eight-digit message embedded in the final output list?
use std::iter;

const INPUT: &str = include_str!("../input/day_16.txt");

pub fn run() {
    let original_signal = load_signal(INPUT);

    // run a 100 phases of FFT on the signal
    let mut signal = original_signal.clone();
    for _ in 0..100 {
        signal = execute_phase(signal);
    }

    // take the first eight digits
    let first_8 =
        signal.iter().take(8).map(|i| i.to_string()).collect::<String>();
    println!(
        "The first eight digits of the final output list are: {}",
        first_8
    );

    // reset the signal
    let mut extended_signal = original_signal
        .iter()
        .cycle()
        .take(10_000 * signal.len())
        .cloned()
        .collect();

    // take the offset
    let offset = get_offset(&extended_signal);
    // run a 100 phases of the FFT with the shortened algorithm
    for _ in 0..100 {
        extended_signal = execute_phase_only_from(extended_signal, offset);
    }
    let offsetted_8 = extended_signal
        .iter()
        .skip(offset)
        .take(8)
        .map(|i| i.to_string())
        .collect::<String>();
    println!(
        "With the real signal the offsetted eight digits of the final output list are: {}",
        offsetted_8
    );
}

type Signal = Vec<i32>;

fn get_offset(signal: &Signal) -> usize {
    signal.iter().take(7).fold(0, |acc, n| (acc * 10) + n) as usize
}

fn execute_phase_only_from(mut signal: Signal, start: usize) -> Signal {
    // this only works if the signal is in the second half of the signal
    assert!(start > (signal.len() / 2));

    let mut sum = 0;
    for i in (start..signal.len()).rev() {
        sum += signal[i];
        signal[i] = sum % 10;
    }
    signal
}

fn execute_phase(signal: Signal) -> Signal {
    let pattern = vec![0, 1, 0, -1];

    signal
        .iter()
        .enumerate()
        .map(|(index, _)| {
            (signal
                .iter()
                .zip(generate_pattern(index, &pattern))
                .map(|(list_element, pattern_value)| {
                    list_element * pattern_value
                })
                .sum::<i32>()
                % 10)
                .abs()
        })
        .collect()
}

fn generate_pattern<'a>(
    position: usize,
    pattern: &'a Signal,
) -> impl Iterator<Item = i32> + 'a {
    // create the pattern values
    let mut pattern_iterator = pattern
        .iter()
        .zip(iter::repeat(position + 1))
        .flat_map(|(value, take)| {
            iter::repeat(value.clone()).take(take.clone())
        })
        .cycle();
    // skip the first value
    pattern_iterator.next();
    pattern_iterator
}

fn load_signal(input: &str) -> Signal {
    input.chars().filter_map(|c| c.to_digit(10).map(|i| i as i32)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_signal() {
        let input = "12345678";

        let expected_signal = vec![1, 2, 3, 4, 5, 6, 7, 8];

        let actual_signal = load_signal(input);
        assert_eq!(actual_signal, expected_signal);
    }

    #[test]
    fn test_generate_pattern() {
        let actual_pattern: Vec<i32> =
            generate_pattern(1, &vec![0, 1, 0, -1]).take(8).collect();
        let expected_pattern = vec![0, 1, 1, 0, 0, -1, -1, 0];

        assert_eq!(actual_pattern, expected_pattern);
    }

    #[test]
    fn test_execute_phase_simple() {
        // starting signal
        let mut signal = vec![1, 2, 3, 4, 5, 6, 7, 8];

        // after 1
        signal = execute_phase(signal);
        assert_eq!(signal, vec![4, 8, 2, 2, 6, 1, 5, 8]);

        // after 2
        signal = execute_phase(signal);
        assert_eq!(signal, vec![3, 4, 0, 4, 0, 4, 3, 8]);

        // after 3
        signal = execute_phase(signal);
        assert_eq!(signal, vec![0, 3, 4, 1, 5, 5, 1, 8]);

        // after 4
        signal = execute_phase(signal);
        assert_eq!(signal, vec![0, 1, 0, 2, 9, 4, 9, 8]);
    }

    #[test]
    fn test_execute_phase_100_times_1() {
        // 80871224585914546619083218645595 becomes 24176176.
        let mut signal = vec![
            8, 0, 8, 7, 1, 2, 2, 4, 5, 8, 5, 9, 1, 4, 5, 4, 6, 6, 1, 9, 0, 8,
            3, 2, 1, 8, 6, 4, 5, 5, 9, 5,
        ];
        for _ in 0..100 {
            signal = execute_phase(signal);
        }
        let first_8: Signal = signal.into_iter().take(8).collect();
        let expected_first_8 = vec![2, 4, 1, 7, 6, 1, 7, 6];
        assert_eq!(first_8, expected_first_8);
    }

    #[test]
    fn test_execute_phase_100_times_2() {
        // 19617804207202209144916044189917 becomes 73745418.
        let mut signal = vec![
            1, 9, 6, 1, 7, 8, 0, 4, 2, 0, 7, 2, 0, 2, 2, 0, 9, 1, 4, 4, 9, 1,
            6, 0, 4, 4, 1, 8, 9, 9, 1, 7,
        ];
        for _ in 0..100 {
            signal = execute_phase(signal);
        }
        let first_8: Signal = signal.into_iter().take(8).collect();
        let expected_first_8 = vec![7, 3, 7, 4, 5, 4, 1, 8];
        assert_eq!(first_8, expected_first_8);
    }

    #[test]
    fn test_execute_phase_100_times_3() {
        // 69317163492948606335995924319873 becomes 52432133.
        let mut signal = vec![
            6, 9, 3, 1, 7, 1, 6, 3, 4, 9, 2, 9, 4, 8, 6, 0, 6, 3, 3, 5, 9, 9,
            5, 9, 2, 4, 3, 1, 9, 8, 7, 3,
        ];
        for _ in 0..100 {
            signal = execute_phase(signal);
        }
        let first_8: Signal = signal.into_iter().take(8).collect();
        let expected_first_8 = vec![5, 2, 4, 3, 2, 1, 3, 3];
        assert_eq!(first_8, expected_first_8);
    }

    #[test]
    fn test_execute_phase_only_from_100_times_1() {
        // 03036732577212944063491565474664 becomes 84462026.
        let mut signal = vec![
            0, 3, 0, 3, 6, 7, 3, 2, 5, 7, 7, 2, 1, 2, 9, 4, 4, 0, 6, 3, 4, 9,
            1, 5, 6, 5, 4, 7, 4, 6, 6, 4,
        ];
        let offset = get_offset(&signal);
        signal = signal
            .iter()
            .cycle()
            .take(10_000 * signal.len())
            .cloned()
            .collect();
        for _ in 0..100 {
            signal = execute_phase_only_from(signal, offset);
        }
        let offsetted_8: Signal =
            signal.into_iter().skip(offset).take(8).collect();
        let expected_offsetted_8 = vec![8, 4, 4, 6, 2, 0, 2, 6];
        assert_eq!(offsetted_8, expected_offsetted_8);
    }

    #[test]
    fn test_execute_phase_only_from_100_times_2() {
        // 02935109699940807407585447034323 becomes 78725270.
        let mut signal = vec![
            0, 2, 9, 3, 5, 1, 0, 9, 6, 9, 9, 9, 4, 0, 8, 0, 7, 4, 0, 7, 5, 8,
            5, 4, 4, 7, 0, 3, 4, 3, 2, 3,
        ];
        let offset = get_offset(&signal);
        signal = signal
            .iter()
            .cycle()
            .take(10_000 * signal.len())
            .cloned()
            .collect();
        for _ in 0..100 {
            signal = execute_phase_only_from(signal, offset);
        }
        let offsetted_8: Signal =
            signal.into_iter().skip(offset).take(8).collect();
        let expected_offsetted_8 = vec![7, 8, 7, 2, 5, 2, 7, 0];
        assert_eq!(offsetted_8, expected_offsetted_8);
    }

    #[test]
    fn test_execute_phase_only_from_100_times_3() {
        // 03081770884921959731165446850517 becomes 53553731.
        let mut signal = vec![
            0, 3, 0, 8, 1, 7, 7, 0, 8, 8, 4, 9, 2, 1, 9, 5, 9, 7, 3, 1, 1, 6,
            5, 4, 4, 6, 8, 5, 0, 5, 1, 7,
        ];
        let offset = get_offset(&signal);
        signal = signal
            .iter()
            .cycle()
            .take(10_000 * signal.len())
            .cloned()
            .collect();
        for _ in 0..100 {
            signal = execute_phase_only_from(signal, offset);
        }
        let offsetted_8: Signal =
            signal.into_iter().skip(offset).take(8).collect();
        let expected_offsetted_8 = vec![5, 3, 5, 5, 3, 7, 3, 1];
        assert_eq!(offsetted_8, expected_offsetted_8);
    }
}
