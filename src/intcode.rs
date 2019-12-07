#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Mode {
    Position,
    Immediate,
}

pub fn execute(mut program: Vec<i32>, mut inputs: Vec<i32>) -> (Vec<i32>, Vec<i32>) {
    let mut position = 0;
    let mut outputs: Vec<i32> = Vec::new();
    loop {
        let (modes, opcode) = extract_modes(program[position]);
        let parameters = &program[position + 1..];

        match (opcode, parameters) {
            (99, _) => break,
            (1, &[first, second, write, ..]) => {
                program[write as usize] = find_value(first, &(modes[0]), &program)
                    + find_value(second, &(modes[1]), &program);
                position += 4;
            }
            (2, &[first, second, write, ..]) => {
                program[write as usize] = find_value(first, &(modes[0]), &program)
                    * find_value(second, &(modes[1]), &program);
                position += 4;
            }
            (3, &[write, ..]) => {
                program[write as usize] = inputs.remove(0);
                position += 2;
            }
            (4, &[read, ..]) => {
                outputs.push(find_value(read, &modes[0], &program));
                position += 2;
            }
            invalid => panic!("Unknown instruction: {:?}", invalid),
        }
    }
    (program, outputs)
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

fn find_value(number: i32, mode: &Mode, program: &Vec<i32>) -> i32 {
    match mode {
        Mode::Position => program[number as usize],
        Mode::Immediate => number,
    }
}

pub fn load(input: &str) -> Vec<i32> {
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

        assert_eq!(execute(input, Vec::new()), (output, Vec::new()));
    }

    #[test]
    fn test_execute_2() {
        let input = vec![2, 3, 0, 3, 99];
        let output = vec![2, 3, 0, 6, 99];

        assert_eq!(execute(input, Vec::new()), (output, Vec::new()));
    }

    #[test]
    fn test_execute_3() {
        let input = vec![2, 4, 4, 5, 99, 0];
        let output = vec![2, 4, 4, 5, 99, 9801];

        assert_eq!(execute(input, Vec::new()), (output, Vec::new()));
    }

    #[test]
    fn test_execute_4() {
        let input = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
        let output = vec![30, 1, 1, 4, 2, 5, 6, 0, 99];

        assert_eq!(execute(input, Vec::new()), (output, Vec::new()));
    }

    #[test]
    fn test_execute_5() {
        let input = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
        let output = vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50];

        assert_eq!(execute(input, Vec::new()), (output, Vec::new()));
    }

    #[test]
    fn test_execute_opcode_3_and_4() {
        let input_program = vec![3, 0, 4, 0, 99];
        let inputs = vec![1];
        let output_program = vec![1, 0, 4, 0, 99];
        let outputs = vec![1];
        assert_eq!(execute(input_program, inputs), (output_program, outputs));
    }

    #[test]
    fn test_execute_opcode_8_position_mode_true() {
        // consider whether the input is equal to 8, output 1 if it is
        let input_program = vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];
        let inputs = vec![8];

        let (_, outputs) = execute(input_program, inputs);
        assert_eq!(outputs, vec![1]);
    }

    #[test]
    fn test_execute_opcode_8_position_mode_false() {
        // consider whether the input is equal to 8, output 0 if it is not
        let input_program = vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];
        let inputs = vec![7];

        let (_, outputs) = execute(input_program, inputs);
        assert_eq!(outputs, vec![0]);
    }

    #[test]
    fn test_execute_opcode_7_position_mode_true() {
        // consider whether the input is less than to 8, output 1 if it is
        let input_program = vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8];
        let inputs = vec![7];

        let (_, outputs) = execute(input_program, inputs);
        assert_eq!(outputs, vec![1]);
    }

    #[test]
    fn test_execute_opcode_7_position_mode_false() {
        // consider whether the input is less than to 8, output 0 if it is not
        let input_program = vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8];
        let inputs = vec![8];

        let (_, outputs) = execute(input_program, inputs);
        assert_eq!(outputs, vec![0]);
    }

    #[test]
    fn test_execute_opcode_8_immediate_mode_true() {
        // consider whether the input is equal to 8, output 1 if it is
        let input_program = vec![3, 3, 1108, -1, 8, 3, 4, 3, 99];
        let inputs = vec![8];

        let (_, outputs) = execute(input_program, inputs);
        assert_eq!(outputs, vec![1]);
    }

    #[test]
    fn test_execute_opcode_8_immediate_mode_false() {
        // consider whether the input is equal to 8, output 0 if it is not
        let input_program = vec![3, 3, 1108, -1, 8, 3, 4, 3, 99];
        let inputs = vec![7];

        let (_, outputs) = execute(input_program, inputs);
        assert_eq!(outputs, vec![0]);
    }

    #[test]
    fn test_execute_opcode_7_immediate_mode_true() {
        // consider whether the input is less than to 8, output 1 if it is
        let input_program = vec![3, 3, 1107, -1, 8, 3, 4, 3, 99];
        let inputs = vec![7];

        let (_, outputs) = execute(input_program, inputs);
        assert_eq!(outputs, vec![1]);
    }

    #[test]
    fn test_execute_opcode_7_immediate_mode_false() {
        // consider whether the input is less than to 8, output 0 if it is not
        let input_program = vec![3, 3, 1107, -1, 8, 3, 4, 3, 99];
        let inputs = vec![8];

        let (_, outputs) = execute(input_program, inputs);
        assert_eq!(outputs, vec![0]);
    }

    #[test]
    fn test_execute_opcode_5_and_6_position_mode_0() {
        // take an input, output 0 if the input was 0
        let input_program = vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9];
        let inputs = vec![0];

        let (_, outputs) = execute(input_program, inputs);
        assert_eq!(outputs, vec![0]);
    }

    #[test]
    fn test_execute_opcode_5_and_6_position_mode_1() {
        // take an input, output 1 if the input was not 0
        let input_program = vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9];
        let inputs = vec![2];

        let (_, outputs) = execute(input_program, inputs);
        assert_eq!(outputs, vec![1]);
    }

    #[test]
    fn test_execute_opcode_5_and_6_immediate_mode_0() {
        // take an input, output 0 if the input was 0
        let input_program = vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1];
        let inputs = vec![0];

        let (_, outputs) = execute(input_program, inputs);
        assert_eq!(outputs, vec![0]);
    }

    #[test]
    fn test_execute_opcode_5_and_6_immediate_mode_1() {
        // take an input, output 1 if the input was not 0
        let input_program = vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1];
        let inputs = vec![2];

        let (_, outputs) = execute(input_program, inputs);
        assert_eq!(outputs, vec![1]);
    }

    #[test]
    fn test_execute_modes() {
        let input = vec![1002, 4, 3, 4, 33];
        let output = vec![1002, 4, 3, 4, 99];

        assert_eq!(execute(input, Vec::new()), (output, Vec::new()));
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
