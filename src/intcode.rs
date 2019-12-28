#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Mode {
    Position,
    Immediate,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ExitStatus {
    Finished,
    WaitingForInput(usize),
}

pub type Program = Vec<i32>;
pub type Inputs = Vec<i32>;
pub type Outputs = Vec<i32>;

pub fn start(
    program: Program,
    inputs: Inputs,
) -> (Program, ExitStatus, Outputs) {
    execute(program, 0, inputs)
}

pub fn resume(
    program: Program,
    status: ExitStatus,
    inputs: Inputs,
) -> (Program, ExitStatus, Outputs) {
    match status {
        ExitStatus::WaitingForInput(position) => {
            execute(program, position, inputs)
        }
        _ => panic!("Trying to resume a finished program"),
    }
}

fn execute(
    mut program: Program,
    starting_position: usize,
    inputs_vec: Inputs,
) -> (Program, ExitStatus, Outputs) {
    let mut inputs = inputs_vec.iter();
    let mut position = starting_position;
    let mut outputs = Vec::new();
    loop {
        let (modes, opcode) = extract_modes(program[position]);
        let parameters = &program[position + 1..];

        match (opcode, parameters) {
            (99, _) => break,
            (1, &[first, second, write, ..]) => {
                program[write as usize] =
                    find_value(first, &(modes[0]), &program)
                        + find_value(second, &(modes[1]), &program);
                position += 4;
            }
            (2, &[first, second, write, ..]) => {
                program[write as usize] =
                    find_value(first, &(modes[0]), &program)
                        * find_value(second, &(modes[1]), &program);
                position += 4;
            }
            (3, &[write, ..]) => {
                match inputs.next() {
                    Some(&input) => {
                        program[write as usize] = input;
                        position += 2;
                    }
                    None => {
                        return (
                            program,
                            ExitStatus::WaitingForInput(position),
                            outputs,
                        );
                    }
                };
            }
            (4, &[read, ..]) => {
                outputs.push(find_value(read, &modes[0], &program));
                position += 2;
            }
            (5, &[condition, jump, ..]) => {
                if find_value(condition, &modes[0], &program) != 0 {
                    position = find_value(jump, &modes[1], &program) as usize;
                } else {
                    position += 3
                }
            }
            (6, &[condition, jump, ..]) => {
                if find_value(condition, &modes[0], &program) == 0 {
                    position = find_value(jump, &modes[1], &program) as usize;
                } else {
                    position += 3
                }
            }
            (7, &[first, second, write, ..]) => {
                let value_to_write =
                    match find_value(first, &modes[0], &program)
                        < find_value(second, &modes[1], &program)
                    {
                        true => 1,
                        false => 0,
                    };
                program[write as usize] = value_to_write;
                position += 4;
            }
            (8, &[first, second, write, ..]) => {
                let value_to_write =
                    match find_value(first, &modes[0], &program)
                        == find_value(second, &modes[1], &program)
                    {
                        true => 1,
                        false => 0,
                    };
                program[write as usize] = value_to_write;
                position += 4;
            }
            invalid => panic!("Unknown instruction: {:?}", invalid),
        }
    }
    (program, ExitStatus::Finished, outputs)
}

fn extract_modes(mut instruction: i32) -> (Vec<Mode>, i32) {
    let opcode = instruction % 100;
    instruction /= 100;

    let mut modes = Vec::new();
    for _ in 0..3 {
        let mode = match instruction % 10 {
            0 => Mode::Position,
            1 => Mode::Immediate,
            _ => panic!("Unknown mode given"),
        };
        modes.push(mode);
        instruction /= 10;
    }
    (modes, opcode)
}

fn find_value(number: i32, mode: &Mode, program: &Program) -> i32 {
    match mode {
        Mode::Position => program[number as usize],
        Mode::Immediate => number,
    }
}

pub fn load(input: &str) -> Program {
    input
        .trim()
        .split(',')
        .map(|number| number.parse())
        .filter_map(Result::ok)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_execute_1() {
        let input = vec![1, 0, 0, 0, 99];
        let output = vec![2, 0, 0, 0, 99];

        assert_eq!(
            execute(input, 0, Vec::new()),
            (output, ExitStatus::Finished, Vec::new())
        );
    }

    #[test]
    fn test_execute_2() {
        let input = vec![2, 3, 0, 3, 99];
        let output = vec![2, 3, 0, 6, 99];

        assert_eq!(
            execute(input, 0, Vec::new()),
            (output, ExitStatus::Finished, Vec::new())
        );
    }

    #[test]
    fn test_execute_3() {
        let input = vec![2, 4, 4, 5, 99, 0];
        let output = vec![2, 4, 4, 5, 99, 9801];

        assert_eq!(
            execute(input, 0, Vec::new()),
            (output, ExitStatus::Finished, Vec::new())
        );
    }

    #[test]
    fn test_execute_4() {
        let input = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
        let output = vec![30, 1, 1, 4, 2, 5, 6, 0, 99];

        assert_eq!(
            execute(input, 0, Vec::new()),
            (output, ExitStatus::Finished, Vec::new())
        );
    }

    #[test]
    fn test_execute_5() {
        let input = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
        let output = vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50];

        assert_eq!(
            execute(input, 0, Vec::new()),
            (output, ExitStatus::Finished, Vec::new())
        );
    }

    #[test]
    fn test_execute_waiting_for_input_exit_status() {
        let program = vec![3, 0, 4, 0, 99];
        let inputs = Vec::new();
        let (_, status, _) = execute(program, 0, inputs);
        assert_eq!(status, ExitStatus::WaitingForInput(0));
    }

    #[test]
    fn test_execute_resume_from_other_start() {
        let input = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
        let starting_position = 8;
        assert_eq!(
            execute(input.clone(), starting_position, Vec::new()),
            (input, ExitStatus::Finished, Vec::new())
        );
    }

    #[test]
    fn test_execute_opcode_3_and_4() {
        let input_program = vec![3, 0, 4, 0, 99];
        let inputs = vec![1];
        let output_program = vec![1, 0, 4, 0, 99];
        let outputs = vec![1];
        assert_eq!(
            execute(input_program, 0, inputs),
            (output_program, ExitStatus::Finished, outputs)
        );
    }

    #[test]
    fn test_execute_opcode_8_position_mode_true() {
        // consider whether the input is equal to 8, output 1 if it is
        let input_program = vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];
        let inputs = vec![8];

        let (_, status, outputs) = execute(input_program, 0, inputs);
        assert_eq!(outputs, vec![1]);
        assert_eq!(status, ExitStatus::Finished);
    }

    #[test]
    fn test_execute_opcode_8_position_mode_false() {
        // consider whether the input is equal to 8, output 0 if it is not
        let input_program = vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];
        let inputs = vec![7];

        let (_, status, outputs) = execute(input_program, 0, inputs);
        assert_eq!(outputs, vec![0]);
        assert_eq!(status, ExitStatus::Finished);
    }

    #[test]
    fn test_execute_opcode_7_position_mode_true() {
        // consider whether the input is less than to 8, output 1 if it is
        let input_program = vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8];
        let inputs = vec![7];

        let (_, status, outputs) = execute(input_program, 0, inputs);
        assert_eq!(outputs, vec![1]);
        assert_eq!(status, ExitStatus::Finished);
    }

    #[test]
    fn test_execute_opcode_7_position_mode_false() {
        // consider whether the input is less than to 8, output 0 if it is not
        let input_program = vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8];
        let inputs = vec![8];

        let (_, status, outputs) = execute(input_program, 0, inputs);
        assert_eq!(outputs, vec![0]);
        assert_eq!(status, ExitStatus::Finished);
    }

    #[test]
    fn test_execute_opcode_8_immediate_mode_true() {
        // consider whether the input is equal to 8, output 1 if it is
        let input_program = vec![3, 3, 1108, -1, 8, 3, 4, 3, 99];
        let inputs = vec![8];

        let (_, status, outputs) = execute(input_program, 0, inputs);
        assert_eq!(outputs, vec![1]);
        assert_eq!(status, ExitStatus::Finished);
    }

    #[test]
    fn test_execute_opcode_8_immediate_mode_false() {
        // consider whether the input is equal to 8, output 0 if it is not
        let input_program = vec![3, 3, 1108, -1, 8, 3, 4, 3, 99];
        let inputs = vec![7];

        let (_, status, outputs) = execute(input_program, 0, inputs);
        assert_eq!(outputs, vec![0]);
        assert_eq!(status, ExitStatus::Finished);
    }

    #[test]
    fn test_execute_opcode_7_immediate_mode_true() {
        // consider whether the input is less than to 8, output 1 if it is
        let input_program = vec![3, 3, 1107, -1, 8, 3, 4, 3, 99];
        let inputs = vec![7];

        let (_, status, outputs) = execute(input_program, 0, inputs);
        assert_eq!(outputs, vec![1]);
        assert_eq!(status, ExitStatus::Finished);
    }

    #[test]
    fn test_execute_opcode_7_immediate_mode_false() {
        // consider whether the input is less than to 8, output 0 if it is not
        let input_program = vec![3, 3, 1107, -1, 8, 3, 4, 3, 99];
        let inputs = vec![8];

        let (_, status, outputs) = execute(input_program, 0, inputs);
        assert_eq!(outputs, vec![0]);
        assert_eq!(status, ExitStatus::Finished);
    }

    #[test]
    fn test_execute_opcode_5_and_6_position_mode_0() {
        // take an input, output 0 if the input was 0
        let input_program =
            vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9];
        let inputs = vec![0];

        let (_, status, outputs) = execute(input_program, 0, inputs);
        assert_eq!(outputs, vec![0]);
        assert_eq!(status, ExitStatus::Finished);
    }

    #[test]
    fn test_execute_opcode_5_and_6_position_mode_1() {
        // take an input, output 1 if the input was not 0
        let input_program =
            vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9];
        let inputs = vec![2];

        let (_, status, outputs) = execute(input_program, 0, inputs);
        assert_eq!(outputs, vec![1]);
        assert_eq!(status, ExitStatus::Finished);
    }

    #[test]
    fn test_execute_opcode_5_and_6_immediate_mode_0() {
        // take an input, output 0 if the input was 0
        let input_program =
            vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1];
        let inputs = vec![0];

        let (_, status, outputs) = execute(input_program, 0, inputs);
        assert_eq!(outputs, vec![0]);
        assert_eq!(status, ExitStatus::Finished);
    }

    #[test]
    fn test_execute_opcode_5_and_6_immediate_mode_1() {
        // take an input, output 1 if the input was not 0
        let input_program =
            vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1];
        let inputs = vec![2];

        let (_, status, outputs) = execute(input_program, 0, inputs);
        assert_eq!(outputs, vec![1]);
        assert_eq!(status, ExitStatus::Finished);
    }

    #[test]
    fn test_execute_larger_example_999() {
        // take an input, output 999 if the input was below 8
        let input_program = vec![
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31,
            1106, 0, 36, 98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104,
            999, 1105, 1, 46, 1101, 1000, 1, 20, 4, 20, 1105, 1, 46, 98, 99,
        ];
        let inputs = vec![7];

        let (_, status, outputs) = execute(input_program, 0, inputs);
        assert_eq!(outputs, vec![999]);
        assert_eq!(status, ExitStatus::Finished);
    }

    #[test]
    fn test_execute_larger_example_1000() {
        // take an input, output 1000 if the input was equal to 8
        let input_program = vec![
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31,
            1106, 0, 36, 98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104,
            999, 1105, 1, 46, 1101, 1000, 1, 20, 4, 20, 1105, 1, 46, 98, 99,
        ];
        let inputs = vec![8];

        let (_, status, outputs) = execute(input_program, 0, inputs);
        assert_eq!(outputs, vec![1000]);
        assert_eq!(status, ExitStatus::Finished);
    }

    #[test]
    fn test_execute_larger_example_1001() {
        // take an input, output 1001 if the input was greater than 8
        let input_program = vec![
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31,
            1106, 0, 36, 98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104,
            999, 1105, 1, 46, 1101, 1000, 1, 20, 4, 20, 1105, 1, 46, 98, 99,
        ];
        let inputs = vec![9];

        let (_, status, outputs) = execute(input_program, 0, inputs);
        assert_eq!(outputs, vec![1001]);
        assert_eq!(status, ExitStatus::Finished);
    }

    #[test]
    fn test_execute_modes() {
        let input = vec![1002, 4, 3, 4, 33];
        let output = vec![1002, 4, 3, 4, 99];

        assert_eq!(
            execute(input, 0, Vec::new()),
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
        let program = &vec![1002, 4, 3, 4, 33];
        let mode = &Mode::Position;
        let number = 4;

        let output = 33;
        assert_eq!(find_value(number, mode, program), output);
    }

    #[test]
    fn test_find_value_immediate_mode() {
        let program = &vec![1002, 4, 3, 4, 33];
        let mode = &Mode::Immediate;
        let number = 3;

        let output = 3;
        assert_eq!(find_value(number, mode, program), output);
    }
}
