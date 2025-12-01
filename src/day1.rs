use aoc_runner_derive::aoc;
use tracing::info;

#[aoc(day1, part1)]
fn part1(input: &str) -> usize {
    let mut zeroes = 0;
    let mut dial = 50;
    for operation in input.lines() {
        let dial_orig = dial;
        dial = move_dial(dial, operation);
        info!("{dial_orig} => {operation} => {dial}");
        if dial == 0 {
            info!("\tZero!");
            zeroes += 1;
        }
    }

    zeroes
}

fn move_dial(mut dial: i32, operation: &str) -> i32 {
    info!("\t{dial}, {operation}");
    let (direction, steps) = operation.split_at(1);
    let steps = steps
        .parse::<i32>()
        .unwrap_or_else(|_| panic!("Couldn't parse {steps} to i32"));
    match direction {
        "L" => dial -= steps,
        "R" => dial += steps,
        _ => panic!(),
    }
    while dial > 99 {
        dial -= 100;
    }
    while dial < 0 {
        dial += 100;
    }
    dial
}

#[aoc(day1, part2)]
fn part2(input: &str) -> usize {
    let mut zeroes = 0;
    let mut dial = 50;
    for operation in input.lines() {
        let dial_orig = dial;
        let (direction, steps) = operation.split_at(1);
        let mut steps = steps
            .parse::<i32>()
            .unwrap_or_else(|_| panic!("Couldn't parse {steps} to i32"));
        let direction_multiplier_thing = match direction {
            "R" => 1,
            "L" => -1,
            _ => panic!(),
        };
        while steps > 0 {
            dial += direction_multiplier_thing;
            steps -= 1;
            while dial > 99 {
                dial -= 100;
            }
            while dial < 0 {
                dial += 100;
            }
            if dial == 0 {
                zeroes += 1;
            }
        }
        info!("{dial_orig} => {operation} => {dial}");
    }
    zeroes
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
        let input = include_str!("../input/2025/day1.txt");
        assert_ne!(part1(input), 482);
        assert_eq!(part1(input), 1086);
    }

    #[test]
    fn verify_part2() {
        let _ = tracing_subscriber::fmt()
            .with_max_level(LevelFilter::OFF)
            .try_init();
        let input = include_str!("../input/2025/day1.txt");
        assert_eq!(part2(input), 6268);
    }

    #[test]
    fn part1_provided_example() {
        let _ = tracing_subscriber::fmt()
            .with_max_level(tracing::Level::INFO)
            .try_init();
        let result = part1(
            r#"L68
L30
R48
L5
R60
L55
L1
L99
R14
L82"#,
        );

        assert_eq!(result, 3)
    }

    #[test]
    fn test_part1_edge_cases() {
        let _ = tracing_subscriber::fmt()
            .with_max_level(tracing::Level::INFO)
            .try_init();
        assert_eq!(move_dial(99, "R1"), 0);
        assert_eq!(move_dial(98, "R1"), 99);
        assert_eq!(move_dial(1, "L1"), 0);
        assert_eq!(move_dial(0, "L1"), 99);

        assert_eq!(move_dial(11, "R8"), 19);
        assert_eq!(move_dial(19, "L19"), 0);
        assert_eq!(move_dial(5, "L10"), 95);
        assert_eq!(move_dial(95, "R5"), 0);
        assert_eq!(move_dial(51, "R44"), 95);
    }

    #[test]
    fn part2_provided_example() {
        let _ = tracing_subscriber::fmt()
            .with_max_level(tracing::Level::INFO)
            .try_init();
        let result = part2(
            r#"L68
L30
R48
L5
R60
L55
L1
L99
R14
L82"#,
        );

        assert_eq!(result, 6)
    }
}
