use aoc_runner_derive::aoc;

#[aoc(dayX, part1)]
fn part1(input: &str) -> usize {
    todo!()
}

// #[aoc(dayX, part2)]
// fn part2(input: &str) -> usize {
//     todo!()
// }

#[cfg(test)]
mod tests {
    // #[test]
    // fn verify_part1() {
    //     let _ = tracing_subscriber::fmt()
    //         .with_max_level(LevelFilter::OFF)
    //         .try_init();
    //     let input = include_str!("../input/2025/dayX.txt");
    //     assert_eq!(part1(input), 0);
    // }

    // #[test]
    // fn verify_part2() {
    //     let _ = tracing_subscriber::fmt()
    //         .with_max_level(LevelFilter::OFF)
    //         .try_init();
    //     let input = include_str!("../input/2025/dayX.txt");
    //     assert_eq!(part2(input), 0);
    // }

    #[test]
    fn part1_provided_example() {
        let _ = tracing_subscriber::fmt()
            .with_max_level(tracing::Level::INFO)
            .try_init();
        let result = part1(r#""#);

        assert_eq!(result, 0)
    }

    // #[test]
    // fn part2_provided_example() {
    //     let _ = tracing_subscriber::fmt()
    //         .with_max_level(tracing::Level::INFO)
    //         .try_init();
    //     let result = part2(
    //         r#""#,
    //     );
    //
    //     assert_eq!(result, 0)
    // }
}
