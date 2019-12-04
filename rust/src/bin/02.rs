use advent_2019::run_intcode;

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

    let program_1 = run_intcode(program_1);

    println!("Answer 1: {}", program_1[0]);

    Ok(())
}

