use aoc_runner_derive::aoc;

type Output = usize;

#[aoc(day2, part1)]
pub fn solve_part_1(input: &str) -> Output {
    let mut bytes = get_input(input, 12, 2);

    for index in (0..bytes.len()).step_by(4) {
        match &bytes[index] {
            &1 => {
                let result = bytes[bytes[index + 1]] + bytes[bytes[index + 2]];
                let output_index = bytes[index + 3];

                bytes[output_index] = result;
            }
            &2 => {
                let result = bytes[bytes[index + 1]] * bytes[bytes[index + 2]];
                let output_index = bytes[index + 3];

                bytes[output_index] = result;
            }
            &99 => break,
            _ => unreachable!(),
        }
    }

    bytes[0]
}

#[aoc(day2, part2)]
pub fn solve_part_2(input: &str) -> Output {
    for noun in 0..99 {
        for verb in 0..99 {
            let mut bytes = get_input(input, noun, verb);

            for index in (0..bytes.len()).step_by(4) {
                if handle_index(index, &mut bytes) {
                    break;
                }
            }

            if bytes[0] == 19690720 {
                return 100 * noun + verb;
            }
        }
    }

    unreachable!()
}

fn handle_index(index: usize, bytes: &mut [usize]) -> bool {
    match &bytes[index] {
        &1 => {
            let result = bytes[bytes[index + 1]] + bytes[bytes[index + 2]];
            let output_index = bytes[index + 3];

            bytes[output_index] = result;
        }
        &2 => {
            let result = bytes[bytes[index + 1]] * bytes[bytes[index + 2]];
            let output_index = bytes[index + 3];

            bytes[output_index] = result;
        }
        &99 => return true,
        _ => unreachable!(),
    }

    return false;
}

fn get_input(input: &str, replace_1: usize, replace_2: usize) -> Vec<usize> {
    let mut bytes = input
        .split(',')
        .map(|char| char.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    bytes[1] = replace_1;
    bytes[2] = replace_2;

    bytes
}
