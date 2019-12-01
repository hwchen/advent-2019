fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input= std::fs::read_to_string("../input/01.txt")?;

    let answer_1: u64 = input.lines()
        .map(|line| {
            let mass = line.parse::<u64>().expect("invalid input");
            fuel_needed(mass)
        })
        .sum();

    println!("Answer 1: {}", answer_1);

    let answer_2: u64 = input.lines()
        .map(|line| {
            let mass = line.parse::<u64>().expect("invalid input");
            fuel_needed_recursive(mass)
        })
        .sum();
    println!("Answer 2: {}", answer_2);
    Ok(())
}

fn fuel_needed(mass: u64) -> u64 {
    (mass / 3) - 2
}

fn fuel_needed_recursive(mass: u64) -> u64 {
    let mut total = 0;
    let mut current_total = mass;

    while current_total > 0 {
        current_total = match (current_total / 3).checked_sub(2) {
            Some(n) => n,
            None => break,
        };
        total += current_total;
    }

    total
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_fuel_needed() {
        assert_eq!(fuel_needed(12), 2);
        assert_eq!(fuel_needed(14), 2);
        assert_eq!(fuel_needed(1969), 654);
        assert_eq!(fuel_needed(100756), 33583);
    }

    #[test]
    fn test_fuel_needed_recursive() {
        assert_eq!(fuel_needed_recursive(14), 2);
        assert_eq!(fuel_needed_recursive(1969), 966);
    }
}
