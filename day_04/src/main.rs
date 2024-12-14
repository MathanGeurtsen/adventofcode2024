use clap::Parser as ArgParser;
use itertools::Itertools;
use std::collections::HashSet;
use std::fs::read_to_string;

#[derive(Debug)]
struct InputStruct {
    char_field: Vec<Vec<char>>,
}

#[derive(ArgParser)]
#[command(author, version, about)]
/// cli arguments
struct Args {
    /// example file path
    #[arg(long, env = "EXAMPLE_FILE", default_value = "example.txt")]
    example_path: std::path::PathBuf,

    /// input file path
    #[arg(long, env = "INPUT_FILE", default_value = "input.txt")]
    input_path: std::path::PathBuf,
}

fn parse_input(input_str: &str) -> InputStruct {
    let mut input = InputStruct {
        char_field: Vec::new(),
    };
    for line in input_str.lines() {
        input.char_field.push(line.chars().collect());
    }
    input
}

fn isxmas(char1: char, char2: char, char3: char, char4: char) -> i32 {
    if (char1.to_ascii_lowercase() == 'x')
        && (char2.to_ascii_lowercase() == 'm')
        && char3.to_ascii_lowercase() == 'a'
        && char4.to_ascii_lowercase() == 's'
    {
        1
    } else {
        0
    }
}

fn pt1(input: &InputStruct) -> Result<(), Box<dyn std::error::Error>> {
    let mut xmasses = 0;
    let y_len = input.char_field.len();
    let x_len = input.char_field[0].len();

    for y in 0..y_len {
        for x in 0..x_len {
            //horizontal forward
            if x + 3 < x_len {
                xmasses += isxmas(
                    input.char_field[y][x],
                    input.char_field[y][x + 1],
                    input.char_field[y][x + 2],
                    input.char_field[y][x + 3],
                );
            }
            //horizontal backward
            if x >= 3 {
                xmasses += isxmas(
                    input.char_field[y][x],
                    input.char_field[y][x - 1],
                    input.char_field[y][x - 2],
                    input.char_field[y][x - 3],
                );
            }
            //vertical downward
            if y + 3 < y_len {
                xmasses += isxmas(
                    input.char_field[y][x],
                    input.char_field[y + 1][x],
                    input.char_field[y + 2][x],
                    input.char_field[y + 3][x],
                );
            }
            //vertical upward
            if y >= 3 {
                xmasses += isxmas(
                    input.char_field[y][x],
                    input.char_field[y - 1][x],
                    input.char_field[y - 2][x],
                    input.char_field[y - 3][x],
                );
            }

            //diagonal right down
            if x + 3 < x_len && y + 3 < y_len {
                xmasses += isxmas(
                    input.char_field[y][x],
                    input.char_field[y + 1][x + 1],
                    input.char_field[y + 2][x + 2],
                    input.char_field[y + 3][x + 3],
                );
            }

            //diagonal right up
            if x + 3 < x_len && y >= 3 {
                xmasses += isxmas(
                    input.char_field[y][x],
                    input.char_field[y - 1][x + 1],
                    input.char_field[y - 2][x + 2],
                    input.char_field[y - 3][x + 3],
                );
            }

            //diagonal left down
            if x >= 3 && y + 3 < y_len {
                xmasses += isxmas(
                    input.char_field[y][x],
                    input.char_field[y + 1][x - 1],
                    input.char_field[y + 2][x - 2],
                    input.char_field[y + 3][x - 3],
                );
            }

            //diagonal left up
            if x >= 3 && y >= 3 {
                xmasses += isxmas(
                    input.char_field[y][x],
                    input.char_field[y - 1][x - 1],
                    input.char_field[y - 2][x - 2],
                    input.char_field[y - 3][x - 3],
                );
            }
        }
    }
    println!("xmasses{:#?}", xmasses);
    Ok(())
}

fn is_mas(chars: (char, char, char)) -> bool {
    chars.0 == 'M' && chars.1 == 'A' && chars.2 == 'S'
}

/// M..
/// .A.
/// ..S
fn down_right_pattern(x: usize, y: usize, char_field: &[Vec<char>]) -> (char, char, char) {
    (
        char_field[y][x],
        char_field[y + 1][x + 1],
        char_field[y + 2][x + 2],
    )
}

/// ..M
/// .A.
/// S..
fn down_left_pattern(x: usize, y: usize, char_field: &[Vec<char>]) -> (char, char, char) {
    (
        char_field[y][x + 2],
        char_field[y + 1][x + 1],
        char_field[y + 2][x],
    )
}

/// ..S
/// .A.
/// M..
fn up_right_pattern(x: usize, y: usize, char_field: &[Vec<char>]) -> (char, char, char) {
    (
        char_field[y + 2][x],
        char_field[y + 1][x + 1],
        char_field[y][x + 2],
    )
}

/// S..
/// .A.
/// ..M
fn up_left_pattern(x: usize, y: usize, char_field: &[Vec<char>]) -> (char, char, char) {
    (
        char_field[y + 2][x + 2],
        char_field[y + 1][x + 1],
        char_field[y][x],
    )
}

fn pt2(input: &InputStruct) -> Result<(), Box<dyn std::error::Error>> {
    let y_len = input.char_field.len();
    let x_len = input.char_field[0].len();

    let mut matches: HashSet<(usize, usize)> = HashSet::new();

    for y in 0..(y_len - 2) {
        for x in 0..(x_len - 2) {
            // Retrieve character tuples for each pattern orientation
            let patterns = [
                down_right_pattern,
                down_left_pattern,
                up_right_pattern,
                up_left_pattern,
            ]
            .iter()
            .combinations(2);

            for pattern_pair in patterns {
                if is_mas(pattern_pair[0](x, y, &input.char_field))
                    && is_mas(pattern_pair[1](x, y, &input.char_field))
                {
                    matches.insert((x, y));

                    println!("{:#?}", pattern_pair);
                }
            }
        }
    }

    let f = &input.char_field;
    for (x, y) in matches.iter().sorted() {
        println!("{:#?},{:#?}", *x, *y);
        println!("{:#?},{:#?},{:#?}", f[*y][*x], f[*y][*x + 1], f[*y][*x + 2]);
        println!(
            "{:#?},{:#?},{:#?}",
            f[*y + 1][*x],
            f[*y + 1][*x + 1],
            f[*y + 1][*x + 2]
        );
        println!(
            "{:#?},{:#?},{:#?}",
            f[*y + 2][*x],
            f[*y + 2][*x + 1],
            f[*y + 2][*x + 2]
        );
    }
    println!("total: {:#?}", matches.len());
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let example_input_str = read_to_string(args.example_path)?;
    let real_input_str = read_to_string(args.input_path)?;

    let input = parse_input(&real_input_str);

    let _ = pt1(&input);
    let _ = pt2(&input);

    Ok(())
}
