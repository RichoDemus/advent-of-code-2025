use aoc_runner_derive::aoc;
use num_bigint::BigUint;
use tracing::info;

#[aoc(day3, part1)]
fn part1(input: &str) -> usize {
    input.lines().map(calc_max_voltage).sum()
}

fn calc_max_voltage(bank: &str) -> usize {
    let bank = bank.chars().collect::<Vec<_>>();
    let mut max = 0;
    for i in 0..(bank.len() - 1) {
        for j in (i + 1)..bank.len() {
            max = max.max(bank[i].to_digit(10).unwrap() * 10 + bank[j].to_digit(10).unwrap());
        }
    }

    max as usize
}

#[aoc(day3, part2)]
fn part2(input: &str) -> BigUint {
    let mut result = BigUint::ZERO;
    for (i, bank) in input.lines().enumerate() {
        result += calc_max_voltage_part2(bank);
        info!("Bank {i} done");
    }

    info!("Result: {result}");
    result
}

fn calc_max_voltage_part2(bank: &str) -> BigUint {
    let bank = bank
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<_>>();

    highest_rec(&bank, BigUint::ZERO, 0).unwrap()
}

fn highest_rec(bank: &[u32], acc: BigUint, index: usize) -> Option<BigUint> {
    info!(
        "{bank:?}: acc: {acc} ({}), index: {index}",
        acc.to_string().len()
    );

    if acc.to_string().len() > 11 {
        return Some(acc);
    }
    //seek the leftmost highest number
    for number in (1..=9).rev() {
        info!("\tlooking for {number}");
        for j in (index)..bank.len() {
            if bank[j] == number {
                // found target number
                info!("\tfound {} at {j}", bank[j]);
                let mut new_acc = acc.clone();
                new_acc *= 10u8;
                new_acc += bank[j];
                info!("acc is now {acc}");
                let candidate = highest_rec(bank, new_acc, j + 1);
                if candidate.is_some() {
                    return candidate;
                }
            }
        }
    }
    info!("dead track");
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;
    use tracing_subscriber::filter::LevelFilter;
    #[test]
    fn verify_part1() {
        let _ = tracing_subscriber::fmt()
            .with_max_level(LevelFilter::OFF)
            .try_init();
        let input = include_str!("../input/2025/day3.txt");
        assert_eq!(part1(input), 17034);
    }

    #[test]
    fn verify_part2() {
        let _ = tracing_subscriber::fmt()
            .with_max_level(LevelFilter::OFF)
            .try_init();
        let input = include_str!("../input/2025/day3.txt");
        assert_eq!(part2(input), BigUint::from_str("168798209663590").unwrap());
    }

    #[test]
    fn part1_provided_example() {
        let _ = tracing_subscriber::fmt()
            .with_max_level(tracing::Level::INFO)
            .try_init();
        let result = part1(
            r#"987654321111111
811111111111119
234234234234278
818181911112111"#,
        );

        assert_eq!(result, 357)
    }

    #[test]
    fn part2_provided_example() {
        let _ = tracing_subscriber::fmt()
            .with_max_level(tracing::Level::INFO)
            .try_init();

        assert_eq!(
            part2("987654321111111"),
            BigUint::from_str("987654321111").unwrap()
        );
        assert_eq!(
            part2("811111111111119"),
            BigUint::from_str("811111111119").unwrap()
        );
        assert_eq!(
            part2("234234234234278"),
            BigUint::from_str("434234234278").unwrap()
        );
        assert_eq!(
            part2("818181911112111"),
            BigUint::from_str("888911112111").unwrap()
        );

        let result = part2(
            r#"987654321111111
811111111111119
234234234234278
818181911112111"#,
        );

        assert_eq!(result, BigUint::from_str("3121910778619").unwrap())
    }
}
