use aoc_runner_derive::aoc;

#[aoc(day6, part1)]
fn part1(input: &str) -> usize {
    let parsed = input
        .lines()
        .map(|line| line.split_ascii_whitespace().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    println!("Parsed: {parsed:?}");

    let problems = parsed[0].len();
    let operator_position = parsed.len() - 1;

    let mut answer = 0;
    #[allow(clippy::needless_range_loop)]
    for problem in 0..problems {
        match parsed[operator_position][problem] {
            "+" => {
                let first = parsed[0][problem].parse::<usize>().unwrap();
                let second = parsed[1][problem].parse::<usize>().unwrap();
                let third = parsed[2][problem].parse::<usize>().unwrap();
                let fourth = parsed[3][problem].parse::<usize>().unwrap();
                let result = first + second + third + fourth;
                println!("solving {first} + {second} + {third} + {fourth} = {result}");
                answer += result;
            }
            "*" => {
                let first = parsed[0][problem].parse::<usize>().unwrap();
                let second = parsed[1][problem].parse::<usize>().unwrap();
                let third = parsed[2][problem].parse::<usize>().unwrap();
                let fourth = parsed[3][problem].parse::<usize>().unwrap();
                let result = first * second * third * fourth;
                println!("solving {first} * {second} * {third} * {fourth} = {result}");
                answer += result;
            }
            _ => todo!(
                "unimplemented operator  at column {problem}: {}",
                parsed[operator_position][problem]
            ),
        }
    }
    answer
}

#[aoc(day6, part2)]
fn part2(input: &str) -> usize {
    let parsed = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut result = 0usize;
    let mut i = 0;
    loop {
        if i >= parsed.last().unwrap().len() {
            break;
        }
        let operator = parsed.last().unwrap()[i];
        println!("Operator is {operator}");
        let (next_operator, _) = parsed
            .last()
            .unwrap()
            .iter()
            .enumerate()
            .skip(i + 1)
            .find(|(_, maybe_operator)| **maybe_operator == '+' || **maybe_operator == '*')
            .unwrap_or_else(|| (parsed.last().unwrap().len() + 1, &' '));
        let next_operator = next_operator - 1;
        println!("Next operator is at {next_operator}");
        let numbers = (i..next_operator)
            .rev()
            .map(|column| {
                let tmp = (0..(parsed.len() - 1))
                    .map(|row| parsed.get(row).unwrap().get(column).unwrap_or(&' '))
                    .collect::<String>();
                tmp.trim()
                    .parse::<usize>()
                    .unwrap_or_else(|_| panic!("unable to parse '{tmp}'"))
            })
            .collect::<Vec<_>>();
        println!("numbers {numbers:?}");
        let res = match operator {
            '+' => numbers.iter().sum::<usize>(),
            '*' => numbers.iter().product::<usize>(),
            _ => todo!(),
        };
        println!("numbers: {numbers:?} {operator} = {res}");
        result += res;
        i = next_operator + 1;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use tracing_subscriber::filter::LevelFilter;
    #[test]
    fn verify_part1() {
        let _ = tracing_subscriber::fmt()
            .with_max_level(LevelFilter::OFF)
            .try_init();
        let input = include_str!("../input/2025/day6.txt");
        assert_eq!(part1(input), 4412382293768);
    }

    #[test]
    fn verify_part2() {
        let _ = tracing_subscriber::fmt()
            .with_max_level(LevelFilter::OFF)
            .try_init();
        let input = include_str!("../input/2025/day6.txt");
        assert_eq!(part2(input), 7858808482092);
    }

    //     #[test]
    //     fn part1_provided_example() {
    //         let _ = tracing_subscriber::fmt()
    //             .with_max_level(tracing::Level::INFO)
    //             .try_init();
    //         let result = part1(r#"123 328  51 64
    //  45 64  387 23
    //   6 98  215 314
    // *   +   *   +  "#);
    //
    //         assert_eq!(result, 4277556)
    //     }

    #[test]
    fn part2_provided_example() {
        let _ = tracing_subscriber::fmt()
            .with_max_level(tracing::Level::INFO)
            .try_init();
        let result = part2(
            r#"123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +  "#,
        );

        assert_eq!(result, 3263827)
    }
}
