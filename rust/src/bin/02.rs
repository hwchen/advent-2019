use advent_2019::IntCode;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("../input/02.txt")?;

    let program: Vec<u64> = input.split(',')
        .map(|n| {
            n.trim().parse::<u64>()
        })
        .collect::<Result<_,_>>()?;

    let intcode = IntCode::new(program);
    let res_1 = intcode.run(12, 2);

    println!("Answer 1: {}", res_1);

    // part 2, search for program inputs that result in in output 19690720
    

    Ok(())
}

