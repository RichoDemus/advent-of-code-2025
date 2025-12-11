use aoc_runner_derive::aoc;
use itertools::Itertools;
use std::collections::{HashSet, VecDeque};
use std::fmt::{Display, Formatter};

#[aoc(day10, part1)]
fn part1(input: &str) -> usize {
    let machines = input.lines().map(Into::into).collect::<Vec<Machine>>();

    let mut result = 0;
    for machine in machines {
        // println!("{i}: {machine}");
        // let best_score = Arc::new(Mutex::new(10));
        // let visited_compinations = Arc::new(DashSet::new());
        // let buttons = machine.button_wiring_schematics.len();
        // let machines = (0..buttons).map(|button_index|solve_machine_rec(machine.clone(), button_index, 1, best_score.clone(), visited_compinations.clone()))
        //     .flatten()
        //     .min();

        // println!("\tSolvable in {machines:?} steps");
        result += solve_machine_bfs(machine);
    }
    result
}
fn solve_machine_bfs(machine: Machine) -> usize {
    let buttons = machine.button_wiring_schematics.len();
    let mut work = VecDeque::new();
    work.push_front(machine);

    let mut best_score = 10;
    let mut visited_states: HashSet<Vec<bool>> = HashSet::new();

    while !work.is_empty() {
        let machine = work.pop_front().unwrap();
        if visited_states.contains(&machine.indicator_lights) {
            continue;
        }
        visited_states.insert(machine.indicator_lights.clone());
        if machine.pressed_buttons > best_score {
            continue;
        }
        for button in 0..buttons {
            let mut new_machine = machine.clone();
            new_machine.press_button(button);
            if new_machine.is_correctly_configured() {
                best_score = best_score.min(new_machine.pressed_buttons);
            }
            work.push_back(new_machine);
        }
    }

    best_score
}

#[derive(Clone)]
struct Machine {
    indicator_lights: Vec<bool>,
    desired_indicator_lights: Vec<bool>,
    button_wiring_schematics: Vec<Button>,
    // joltages: Vec<usize>,
    joltage_requirements: Vec<usize>,
    pressed_buttons: usize,
}

impl Machine {
    fn press_button(&mut self, button: usize) {
        let button = &self.button_wiring_schematics[button];
        for light_index in &button.lights {
            self.indicator_lights[*light_index] = !self.indicator_lights[*light_index];
        }
        self.pressed_buttons += 1;
    }
    fn is_correctly_configured(&self) -> bool {
        self.indicator_lights == self.desired_indicator_lights
    }
}

impl From<&str> for Machine {
    fn from(value: &str) -> Self {
        let split1 = value.split(']').collect::<Vec<_>>();
        let lights = &split1[0][1..];
        let desired_lights = lights.chars().map(|c| c == '#').collect::<Vec<_>>();
        let lights = (0..desired_lights.len()).map(|_| false).collect::<Vec<_>>();

        let split2 = split1[1].split('{').collect::<Vec<_>>();
        let wirings = split2[0].trim();
        let buttons = wirings
            .split_ascii_whitespace()
            .map(|wiring| {
                let wiring = &wiring[1..];
                let wiring = &wiring[0..(wiring.len() - 1)];
                let wiring: Vec<usize> = wiring
                    .split(',')
                    .map(|c| c.parse().unwrap_or_else(|_| panic!("Unable to parse{c}")))
                    .collect_vec();
                Button { lights: wiring }
            })
            .collect_vec();

        let joltage = &split2[1][0..(split2[1].len() - 1)];
        let joltage: Vec<usize> = joltage
            .split(',')
            .map(|c| c.parse().unwrap_or_else(|_| panic!("Unable to parse{c}")))
            .collect_vec();

        Self {
            indicator_lights: lights,
            desired_indicator_lights: desired_lights,
            button_wiring_schematics: buttons,
            joltage_requirements: joltage,
            // joltages: joltage.into_iter().map(|_|0).collect_vec(),
            pressed_buttons: 0,
        }
    }
}

impl Display for Machine {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let lights = self
            .indicator_lights
            .iter()
            .map(|l| if *l { '#' } else { '.' })
            .join("");
        let desired_lights = self
            .desired_indicator_lights
            .iter()
            .map(|l| if *l { '#' } else { '.' })
            .join("");
        let buttons = self
            .button_wiring_schematics
            .iter()
            .map(|button| format!("({})", button.lights.iter().join(",")))
            .join(" ");
        let joltage = self.joltage_requirements.iter().join(",");
        write!(f, "[{lights}]/[{desired_lights}] {buttons} {{{joltage}}}")
    }
}

#[derive(Clone)]
struct Button {
    lights: Vec<usize>,
}

// #[aoc(day10, part2)]
// fn part2(input: &str) -> usize {
//
// }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn verify_part1() {
        let input = include_str!("../input/2025/day10.txt");
        assert_eq!(part1(input), 441);
    }

    // #[test]
    // fn verify_part2() {
    //     let input = include_str!("../input/2025/day10.txt");
    //     assert_eq!(part2(input), 0);
    // }

    #[test]
    fn part1_provided_example() {
        let result = part1(
            r#"[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}"#,
        );

        assert_eq!(result, 7)
    }

    //     #[test]
    //     fn part2_provided_example() {
    //         let result = part2(
    //             r#"[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
    // [...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
    // [.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}"#,
    //         );
    //
    //         assert_eq!(result, 33)
    //     }
}
