pub struct IntCode {
    program: Vec<u64>,
}

impl IntCode {
    pub fn new(program: Vec<u64>) -> Self {
        Self { program }
    }

    pub fn run(&self, input_1: u64, input_2: u64) -> u64 {
        let mut program = self.program.clone();
        program[1] = input_1;
        program[2] = input_2;
        run_intcode(&mut program);

        program[0]
    }

    pub fn run_inplace(&mut self, input_1: u64, input_2: u64) -> u64 {
        self.program[1] = input_1;
        self.program[2] = input_2;
        run_intcode(&mut self.program);

        self.program[0]
    }
}

fn run_intcode(program: &mut [u64]) {
    let mut cursor = 0;

    while let Some(opcode) = program.get(cursor) {
        match opcode {
            1 => {
                let register = program[cursor + 3] as usize;
                program[register] = program[program[cursor + 1] as usize] + program[program[cursor + 2] as usize];
                cursor += 4;
            },
            2 => {
                let register = program[cursor + 3] as usize;
                program[register] = program[program[cursor + 1] as usize] * program[program[cursor + 2] as usize];
                cursor += 4;
            },
            99 => {
                break;
            },
            _ => panic!("logic error"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_intcode() {
        assert_eq!(run_1(vec![1,0,0,0,99]), vec![2,0,0,0,99]);
        assert_eq!(run_1(vec![2,3,0,3,99]), vec![2,3,0,6,99]);
        assert_eq!(run_1(vec![1,1,1,4,99,5,6,0,99]), vec![30,1,1,4,2,5,6,0,99]);
    }
}
