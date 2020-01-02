use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Mode {
    Position,
    Immediate,
    Relative,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ExitStatus {
    Finished,
    WaitingForInput(i64, i64),
}

pub type Program = HashMap<i64, i64>;
pub type Inputs = Vec<i64>;
pub type Outputs = Vec<i64>;

pub fn start(
    program: Program,
    inputs: Inputs,
) -> (Program, ExitStatus, Outputs) {
    execute(program, 0, 0, inputs)
}

pub fn resume(
    program: Program,
    status: ExitStatus,
    inputs: Inputs,
) -> (Program, ExitStatus, Outputs) {
    match status {
        ExitStatus::WaitingForInput(position, base) => {
            execute(program, position, base, inputs)
        }
        _ => panic!("Trying to resume a finished program"),
    }
}

fn execute(
    mut program: Program,
    starting_position: i64,
    starting_base: i64,
    inputs_vec: Inputs,
) -> (Program, ExitStatus, Outputs) {
    let mut inputs = inputs_vec.iter();
    let mut position = starting_position;
    let mut base = starting_base;
    let mut outputs = Vec::new();
    loop {
        let (modes, opcode) = extract_modes(program[&position]);

        match opcode {
            // exit the program
            99 => {
                return (program, ExitStatus::Finished, outputs);
            }
            // add the first and the second parameter, write to the third
            1 => {
                let first = find_value(position + 1, &modes[0], base, &program);
                let second =
                    find_value(position + 2, &modes[1], base, &program);
                let write =
                    writing_position(position + 3, &modes[2], base, &program);
                program.insert(write, first + second);
                position += 4;
            }
            // multiply the first and the second parameter, write to the third
            2 => {
                let first = find_value(position + 1, &modes[0], base, &program);
                let second =
                    find_value(position + 2, &modes[1], base, &program);
                let write =
                    writing_position(position + 3, &modes[2], base, &program);
                program.insert(write, first * second);
                position += 4;
            }
            // get an input, write it to the first parameter
            3 => {
                match inputs.next() {
                    Some(&input) => {
                        let write = writing_position(
                            position + 1,
                            &modes[0],
                            base,
                            &program,
                        );
                        program.insert(write, input);
                        position += 2;
                    }
                    None => {
                        return (
                            program,
                            ExitStatus::WaitingForInput(position, base),
                            outputs,
                        );
                    }
                };
            }
            // write the first parameter to output
            4 => {
                let read = find_value(position + 1, &modes[0], base, &program);
                outputs.push(read);
                position += 2;
            }
            // test the if the first parameter is not 0, if so jump to the second
            5 => {
                let condition =
                    find_value(position + 1, &modes[0], base, &program);
                let jump = find_value(position + 2, &modes[1], base, &program);
                if condition != 0 {
                    position = jump;
                } else {
                    position += 3
                }
            }
            // test the if the first parameter is 0, if so jump to the second
            6 => {
                let condition =
                    find_value(position + 1, &modes[0], base, &program);
                let jump = find_value(position + 2, &modes[1], base, &program);
                if condition == 0 {
                    position = jump;
                } else {
                    position += 3
                }
            }
            // test the if the first parameter is smaller than the second,
            // if so write 1 to the third, otherwise write 0
            7 => {
                let first = find_value(position + 1, &modes[0], base, &program);
                let second =
                    find_value(position + 2, &modes[1], base, &program);
                let write =
                    writing_position(position + 3, &modes[2], base, &program);
                let value_to_write = match first < second {
                    true => 1,
                    false => 0,
                };
                program.insert(write, value_to_write);
                position += 4;
            }
            // test the if the first parameter is equal to the second,
            // if so write 1 to the third, otherwise write 0
            8 => {
                let first = find_value(position + 1, &modes[0], base, &program);
                let second =
                    find_value(position + 2, &modes[1], base, &program);
                let write =
                    writing_position(position + 3, &modes[2], base, &program);
                let value_to_write = match first == second {
                    true => 1,
                    false => 0,
                };
                program.insert(write, value_to_write);
                position += 4;
            }
            // mutate the relative base with the first parameter
            9 => {
                let mutation =
                    find_value(position + 1, &modes[0], base, &program);
                base += mutation;
                position += 2;
            }
            invalid => {
                panic!("Unknown instruction: {:?}", invalid);
            }
        }
    }
}

fn extract_modes(mut instruction: i64) -> (Vec<Mode>, i64) {
    let opcode = instruction % 100;
    instruction /= 100;

    let mut modes = Vec::new();
    for _ in 0..3 {
        let mode = match instruction % 10 {
            0 => Mode::Position,
            1 => Mode::Immediate,
            2 => Mode::Relative,
            _ => panic!("Unknown mode given"),
        };
        modes.push(mode);
        instruction /= 10;
    }
    (modes, opcode)
}

fn find_value(position: i64, mode: &Mode, base: i64, program: &Program) -> i64 {
    let number = *program.get(&position).unwrap_or(&0);
    match mode {
        Mode::Position => *program.get(&number).unwrap_or(&0),
        Mode::Immediate => number,
        Mode::Relative => *program.get(&(base + number)).unwrap_or(&0),
    }
}

fn writing_position(
    position: i64,
    mode: &Mode,
    base: i64,
    program: &Program,
) -> i64 {
    let number = *program.get(&position).unwrap_or(&0);
    match mode {
        Mode::Position => number,
        Mode::Immediate => {
            panic!("Writing parameter is not allowed to be immediate")
        }
        Mode::Relative => number + base,
    }
}

pub fn load(input: &str) -> Program {
    input
        .trim()
        .split(',')
        .map(|number| number.parse())
        .filter_map(Result::ok)
        .enumerate()
        .map(|(index, number)| (index as i64, number))
        .collect()
}

#[cfg(test)]
#[macro_use]
mod tests {
    use super::*;

    macro_rules! program {
        ( $( $x:expr ),* ) => {
            {
            let mut temp_program = HashMap::new();
            let mut index = -1;
            $(
                index += 1;
                temp_program.insert(index, $x);
            )*
            temp_program
            }
        };
    }

    #[test]
    fn test_start_1() {
        let input = program![1, 0, 0, 0, 99];
        let output = program![2, 0, 0, 0, 99];

        assert_eq!(
            start(input, Vec::new()),
            (output, ExitStatus::Finished, Vec::new())
        );
    }

    #[test]
    fn test_start_2() {
        let input = program![2, 3, 0, 3, 99];
        let output = program![2, 3, 0, 6, 99];

        assert_eq!(
            start(input, Vec::new()),
            (output, ExitStatus::Finished, Vec::new())
        );
    }

    #[test]
    fn test_start_3() {
        let input = program![2, 4, 4, 5, 99, 0];
        let output = program![2, 4, 4, 5, 99, 9801];

        assert_eq!(
            start(input, Vec::new()),
            (output, ExitStatus::Finished, Vec::new())
        );
    }

    #[test]
    fn test_start_4() {
        let input = program![1, 1, 1, 4, 99, 5, 6, 0, 99];
        let output = program![30, 1, 1, 4, 2, 5, 6, 0, 99];

        assert_eq!(
            start(input, Vec::new()),
            (output, ExitStatus::Finished, Vec::new())
        );
    }

    #[test]
    fn test_start_5() {
        let input = program![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
        let output = program![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50];

        assert_eq!(
            start(input, Vec::new()),
            (output, ExitStatus::Finished, Vec::new())
        );
    }

    #[test]
    fn test_start_waiting_for_input_exit_status() {
        let program = program![3, 0, 4, 0, 99];
        let inputs = Vec::new();
        let (_, status, _) = start(program, inputs);
        assert_eq!(status, ExitStatus::WaitingForInput(0, 0));
    }

    #[test]
    fn test_resume() {
        let input = program![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
        let exit_status = ExitStatus::WaitingForInput(8, 0);
        assert_eq!(
            resume(input.clone(), exit_status, Vec::new()),
            (input, ExitStatus::Finished, Vec::new())
        );
    }
    #[test]
    #[should_panic]
    fn test_resume_on_finished_program() {
        let input = program![99];
        let exit_status = ExitStatus::Finished;
        resume(input.clone(), exit_status, Vec::new());
    }

    #[test]
    fn test_start_opcode_3_and_4() {
        let input_program = program![3, 0, 4, 0, 99];
        let inputs = vec![1];
        let output_program = program![1, 0, 4, 0, 99];
        let outputs = vec![1];
        assert_eq!(
            start(input_program, inputs),
            (output_program, ExitStatus::Finished, outputs)
        );
    }

    #[test]
    fn test_start_opcode_8_position_mode_true() {
        // consider whether the input is equal to 8, output 1 if it is
        let input_program = program![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];
        let inputs = vec![8];

        let (_, status, outputs) = start(input_program, inputs);
        assert_eq!(outputs, vec![1]);
        assert_eq!(status, ExitStatus::Finished);
    }

    #[test]
    fn test_start_opcode_8_position_mode_false() {
        // consider whether the input is equal to 8, output 0 if it is not
        let input_program = program![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];
        let inputs = vec![7];

        let (_, status, outputs) = start(input_program, inputs);
        assert_eq!(outputs, vec![0]);
        assert_eq!(status, ExitStatus::Finished);
    }

    #[test]
    fn test_start_opcode_7_position_mode_true() {
        // consider whether the input is less than to 8, output 1 if it is
        let input_program = program![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8];
        let inputs = vec![7];

        let (_, status, outputs) = start(input_program, inputs);
        assert_eq!(outputs, vec![1]);
        assert_eq!(status, ExitStatus::Finished);
    }

    #[test]
    fn test_start_opcode_7_position_mode_false() {
        // consider whether the input is less than to 8, output 0 if it is not
        let input_program = program![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8];
        let inputs = vec![8];

        let (_, status, outputs) = start(input_program, inputs);
        assert_eq!(outputs, vec![0]);
        assert_eq!(status, ExitStatus::Finished);
    }

    #[test]
    fn test_start_opcode_8_immediate_mode_true() {
        // consider whether the input is equal to 8, output 1 if it is
        let input_program = program![3, 3, 1108, -1, 8, 3, 4, 3, 99];
        let inputs = vec![8];

        let (_, status, outputs) = start(input_program, inputs);
        assert_eq!(outputs, vec![1]);
        assert_eq!(status, ExitStatus::Finished);
    }

    #[test]
    fn test_start_opcode_8_immediate_mode_false() {
        // consider whether the input is equal to 8, output 0 if it is not
        let input_program = program![3, 3, 1108, -1, 8, 3, 4, 3, 99];
        let inputs = vec![7];

        let (_, status, outputs) = start(input_program, inputs);
        assert_eq!(outputs, vec![0]);
        assert_eq!(status, ExitStatus::Finished);
    }

    #[test]
    fn test_start_opcode_7_immediate_mode_true() {
        // consider whether the input is less than to 8, output 1 if it is
        let input_program = program![3, 3, 1107, -1, 8, 3, 4, 3, 99];
        let inputs = vec![7];

        let (_, status, outputs) = start(input_program, inputs);
        assert_eq!(outputs, vec![1]);
        assert_eq!(status, ExitStatus::Finished);
    }

    #[test]
    fn test_start_opcode_7_immediate_mode_false() {
        // consider whether the input is less than to 8, output 0 if it is not
        let input_program = program![3, 3, 1107, -1, 8, 3, 4, 3, 99];
        let inputs = vec![8];

        let (_, status, outputs) = start(input_program, inputs);
        assert_eq!(outputs, vec![0]);
        assert_eq!(status, ExitStatus::Finished);
    }

    #[test]
    fn test_start_opcode_5_and_6_position_mode_0() {
        // take an input, output 0 if the input was 0
        let input_program =
            program![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9];
        let inputs = vec![0];

        let (_, status, outputs) = start(input_program, inputs);
        assert_eq!(outputs, vec![0]);
        assert_eq!(status, ExitStatus::Finished);
    }

    #[test]
    fn test_start_opcode_5_and_6_position_mode_1() {
        // take an input, output 1 if the input was not 0
        let input_program =
            program![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9];
        let inputs = vec![2];

        let (_, status, outputs) = start(input_program, inputs);
        assert_eq!(outputs, vec![1]);
        assert_eq!(status, ExitStatus::Finished);
    }

    #[test]
    fn test_start_opcode_5_and_6_immediate_mode_0() {
        // take an input, output 0 if the input was 0
        let input_program =
            program![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1];
        let inputs = vec![0];

        let (_, status, outputs) = start(input_program, inputs);
        assert_eq!(outputs, vec![0]);
        assert_eq!(status, ExitStatus::Finished);
    }

    #[test]
    fn test_start_opcode_5_and_6_immediate_mode_1() {
        // take an input, output 1 if the input was not 0
        let input_program =
            program![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1];
        let inputs = vec![2];

        let (_, status, outputs) = start(input_program, inputs);
        assert_eq!(outputs, vec![1]);
        assert_eq!(status, ExitStatus::Finished);
    }

    #[test]
    fn test_start_larger_example_999() {
        // take an input, output 999 if the input was below 8
        let input_program = program![
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31,
            1106, 0, 36, 98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104,
            999, 1105, 1, 46, 1101, 1000, 1, 20, 4, 20, 1105, 1, 46, 98, 99
        ];
        let inputs = vec![7];

        let (_, status, outputs) = start(input_program, inputs);
        assert_eq!(outputs, vec![999]);
        assert_eq!(status, ExitStatus::Finished);
    }

    #[test]
    fn test_start_larger_example_1000() {
        // take an input, output 1000 if the input was equal to 8
        let input_program = program![
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31,
            1106, 0, 36, 98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104,
            999, 1105, 1, 46, 1101, 1000, 1, 20, 4, 20, 1105, 1, 46, 98, 99
        ];
        let inputs = vec![8];

        let (_, status, outputs) = start(input_program, inputs);
        assert_eq!(outputs, vec![1000]);
        assert_eq!(status, ExitStatus::Finished);
    }

    #[test]
    fn test_start_larger_example_1001() {
        // take an input, output 1001 if the input was greater than 8
        let input_program = program![
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31,
            1106, 0, 36, 98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104,
            999, 1105, 1, 46, 1101, 1000, 1, 20, 4, 20, 1105, 1, 46, 98, 99
        ];
        let inputs = vec![9];

        let (_, status, outputs) = start(input_program, inputs);
        assert_eq!(outputs, vec![1001]);
        assert_eq!(status, ExitStatus::Finished);
    }

    #[test]
    fn test_start_opcode_9() {
        // take no input and produce a copy of itself as outputs
        let input_program = program![
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101,
            0, 99
        ];
        let expected_outputs = vec![
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101,
            0, 99,
        ];

        let (_, _, actual_outputs) = start(input_program, Vec::new());
        assert_eq!(actual_outputs, expected_outputs);
    }

    #[test]
    fn test_start_large_number_example_1() {
        // should output a 16-digit number
        let input_program = program![1102, 34915192, 34915192, 7, 4, 7, 99, 0];

        let (_, _, outputs) = start(input_program, Vec::new());
        assert_eq!(outputs[0], 1219070632396864);
    }

    #[test]
    fn test_start_large_number_example_2() {
        // should output a 16-digit number
        let input_program = program![104, 1125899906842624, 99];

        let (_, _, outputs) = start(input_program, Vec::new());
        assert_eq!(outputs[0], 1125899906842624);
    }

    #[test]
    fn test_start_modes() {
        let input = program![1002, 4, 3, 4, 33];
        let output = program![1002, 4, 3, 4, 99];

        assert_eq!(
            start(input, Vec::new()),
            (output, ExitStatus::Finished, Vec::new())
        );
    }

    #[test]
    fn test_extract_modes() {
        let input = 1002;
        let output = (vec![Mode::Position, Mode::Immediate, Mode::Position], 2);

        assert_eq!(extract_modes(input), output);
    }

    #[test]
    fn test_find_value_position_mode() {
        let program = &program![1002, 4, 3, 4, 33];
        let mode = &Mode::Position;
        let position = 1;

        let output = 33;
        assert_eq!(find_value(position, mode, 0, program), output);
    }

    #[test]
    fn test_find_value_immediate_mode() {
        let program = &program![1002, 4, 3, 4, 33];
        let mode = &Mode::Immediate;
        let position = 2;

        let output = 3;
        assert_eq!(find_value(position, mode, 0, program), output);
    }

    #[test]
    fn test_find_value_relative_mode() {
        let program = &program![2002, 4, 3, 4, 33];
        let mode = &Mode::Relative;
        let position = 2;

        let output = 4;
        assert_eq!(find_value(position, mode, -2, program), output);
    }

    #[test]
    fn test_writing_position_position_mode() {
        let program = program![3, 3, 99];
        let mode = &Mode::Position;
        let position = 1;
        let base = 0;

        let expected = 3;
        assert_eq!(writing_position(position, mode, base, &program), expected)
    }

    #[test]
    #[should_panic]
    fn test_writing_position_immediate_mode() {
        let program = program![103, 0, 99];
        let mode = &Mode::Immediate;
        let position = 1;
        let base = 0;

        writing_position(position, mode, base, &program);
    }

    #[test]
    fn test_writing_position_relative_mode() {
        let program = program![203, 0, 99];
        let mode = &Mode::Relative;
        let position = 1;
        let base = 3;

        let expected = 3;
        assert_eq!(writing_position(position, mode, base, &program), expected)
    }
}
