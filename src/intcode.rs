pub fn execute(mut program: Vec<i32>, mut inputs: Vec<i32>) -> (Vec<i32>, Vec<i32>) {
    let mut position = 0;
    let mut outputs: Vec<i32> = Vec::new();
    loop {
        match &program[position..] {
            &[99, ..] => break,
            &[1, first, second, target, ..] => {
                program[target as usize] =
                    program[first as usize] + program[second as usize];
            },
            &[2, first, second, target, ..] => {
                program[target as usize] =
                    program[first as usize] * program[second as usize];
            },
            _ => panic!("Unknown instruction"),

        }
        position += 4;
    }
    (program, outputs)
}

pub fn load(input: &str) -> Vec<i32> {
    input.trim()
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
        let input = vec![1,0,0,0,99];
        let output = vec![2,0,0,0,99];

        assert_eq!(execute(input, Vec::new()), (output, Vec::new()));
    }

    #[test]
    fn test_execute_2() {
        let input = vec![2,3,0,3,99];
        let output = vec![2,3,0,6,99];

        assert_eq!(execute(input, Vec::new()), (output, Vec::new()));
    }

    #[test]
    fn test_execute_3() {
        let input = vec![2,4,4,5,99,0];
        let output = vec![2,4,4,5,99,9801];

        assert_eq!(execute(input, Vec::new()), (output, Vec::new()));
    }

    #[test]
    fn test_execute_4() {
        let input = vec![1,1,1,4,99,5,6,0,99];
        let output = vec![30,1,1,4,2,5,6,0,99];

        assert_eq!(execute(input, Vec::new()), (output, Vec::new()));
    }

    #[test]
    fn test_execute_5() {
        let input = vec![1,9,10,3,2,3,11,0,99,30,40,50];
        let output = vec![3500,9,10,70,2,3,11,0,99,30,40,50];

        assert_eq!(execute(input, Vec::new()), (output, Vec::new()));
    }
}
