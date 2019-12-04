fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("../input/02.txt")?;

    let program: Vec<u64> = input.split(',')
        .map(|n| {
            n.trim().parse::<u64>()
        })
        .collect::<Result<_,_>>()?;

    let mut program_1 = program.clone();
    program_1[1] = 12;
    program_1[2] = 2;

    let program_1 = run_1(program_1);

    println!("Answer 1: {}", program_1[0]);

    Ok(())
}

fn run_1(program: Vec<u64>) -> Vec<u64> {
    let mut program = program;
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

    program
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_run_1() {
        assert_eq!(run_1(vec![1,0,0,0,99]), vec![2,0,0,0,99]);
        assert_eq!(run_1(vec![2,3,0,3,99]), vec![2,3,0,6,99]);
        assert_eq!(run_1(vec![1,1,1,4,99,5,6,0,99]), vec![30,1,1,4,2,5,6,0,99]);
    }
}
