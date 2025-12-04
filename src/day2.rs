use aoc_runner_derive::aoc;
use tracing::info;

#[aoc(day2, part1)]
fn part1(input: &str) -> usize {
    input
        .trim()
        .split(',')
        .flat_map(|range| {
            let (start, end): (usize, usize) = match range.split('-').collect::<Vec<_>>().as_slice()
            {
                [start, end] => (
                    start
                        .parse()
                        .unwrap_or_else(|_| panic!("Can't parse start {start}")),
                    end.parse()
                        .unwrap_or_else(|_| panic!("Can't parse end {end}")),
                ),
                _ => panic!(),
            };
            start..=end
        })
        .filter(|number| is_invalid(*number))
        .sum()
}

fn is_invalid(number: usize) -> bool {
    let number = number.to_string();
    info!("{number}");
    if !number.len().is_multiple_of(2) {
        info!("\todd..");
        return false;
    }
    let mid = number.len() / 2;
    for i in 0..(mid) {
        if number.chars().nth(i).unwrap() != number.chars().nth(mid + i).unwrap() {
            return false;
        }
    }

    true
}

#[aoc(day2, part2)]
fn part2(input: &str) -> usize {
    input
        .trim()
        .split(',')
        .flat_map(|range| {
            let (start, end): (usize, usize) = match range.split('-').collect::<Vec<_>>().as_slice()
            {
                [start, end] => (
                    start
                        .parse()
                        .unwrap_or_else(|_| panic!("Can't parse start {start}")),
                    end.parse()
                        .unwrap_or_else(|_| panic!("Can't parse end {end}")),
                ),
                _ => panic!(),
            };
            start..=end
        })
        .filter(|number| is_invalid2(*number))
        .sum()
}

fn is_invalid2(number: usize) -> bool {
    let numberx = number.to_string().chars().collect::<Vec<char>>();
    let number = numberx.as_slice();
    info!("{number:?}");

    for length in 1..(number.len()) {
        let mut chunks = number.chunks(length).collect::<Vec<_>>();
        chunks.sort_unstable();
        chunks.dedup();
        info!("\tchunks: {chunks:?}");
        if chunks.len() == 1 {
            info!("\t\tinvalid!");
            return true;
        }
    }
    info!("sadly valid");
    false
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
        let input = include_str!("../input/2025/day2.txt");
        assert_eq!(part1(input), 19605500130);
    }

    #[test]
    fn verify_part2() {
        let _ = tracing_subscriber::fmt()
            .with_max_level(LevelFilter::OFF)
            .try_init();
        let input = include_str!("../input/2025/day2.txt");
        assert_eq!(part2(input), 36862281418);
    }

    #[test]
    fn part1_provided_example() {
        let _ = tracing_subscriber::fmt()
            .with_max_level(tracing::Level::INFO)
            .try_init();

        let result = part1(
            r#"11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"#,
        );

        assert_eq!(result, 1227775554);
    }

    #[test]
    fn part2_provided_example() {
        let _ = tracing_subscriber::fmt()
            .with_max_level(tracing::Level::INFO)
            .try_init();

        assert_eq!(is_invalid2(446446), true);
        assert_eq!(is_invalid2(11), true);
        assert_eq!(is_invalid2(12), false);
        assert_eq!(is_invalid2(1188511885), true);
        assert_eq!(is_invalid2(1188511884), false);
        assert_eq!(is_invalid2(824824824), true);
        assert_eq!(is_invalid2(2121212121), true);
        assert_eq!(is_invalid2(565656), true);
        assert_eq!(is_invalid2(38593859), true);

        let result = part2(
            r#"11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"#,
        );
        assert_eq!(result, 4174379265)
    }
}
