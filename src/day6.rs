use aoc_runner_derive::aoc;

#[aoc(day6, part1)]
fn part1(input: &str) -> usize {
    let parsed = input
        .lines()
        .map(|line| line.split_ascii_whitespace().collect::<Vec<_>>())
        .collect::<Vec<_>>();

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
                answer += result;
            }
            "*" => {
                let first = parsed[0][problem].parse::<usize>().unwrap();
                let second = parsed[1][problem].parse::<usize>().unwrap();
                let third = parsed[2][problem].parse::<usize>().unwrap();
                let fourth = parsed[3][problem].parse::<usize>().unwrap();
                let result = first * second * third * fourth;
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
        let (next_operator, _) = parsed
            .last()
            .unwrap()
            .iter()
            .enumerate()
            .skip(i + 1)
            .find(|(_, maybe_operator)| **maybe_operator == '+' || **maybe_operator == '*')
            .unwrap_or_else(|| (parsed.last().unwrap().len() + 1, &' '));
        let next_operator = next_operator - 1;
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
        let res = match operator {
            '+' => numbers.iter().sum::<usize>(),
            '*' => numbers.iter().product::<usize>(),
            _ => todo!(),
        };
        result += res;
        i = next_operator + 1;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn verify_part1() {
        let input = include_str!("../input/2025/day6.txt");
        assert_eq!(part1(input), 4412382293768);
    }

    #[test]
    fn verify_part2() {
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
        let result = part2(
            r#"123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +  "#,
        );

        assert_eq!(result, 3263827)
    }
}
