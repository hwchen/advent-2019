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
    let mut res = None;

    // Inefficient, but sufficient
    for i in 0..=99 {
        for j in 0..=99 {
            let output = intcode.run(i,j);
            if output == 19690720 {
                res = Some((i,j));
            }
        }
    }

    let res = res.unwrap();
    println!("Answer 2: {}", 100 * res.0 + res.1);

    Ok(())
}

