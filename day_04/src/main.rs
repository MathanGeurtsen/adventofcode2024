use clap::Parser as ArgParser;
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
    println!("{:#?}{:#?}{:#?}{:#?}", char1, char2, char3, char4);
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

    let mut char1: char;
    let mut char2: char;
    let mut char3: char;
    let mut char4: char;

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
            if x > 3 {
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
            if y > 3 {
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
            if x + 3 < x_len && y > 3 {
                xmasses += isxmas(
                    input.char_field[y][x],
                    input.char_field[y - 1][x + 1],
                    input.char_field[y - 2][x + 2],
                    input.char_field[y - 3][x + 3],
                );
            }

            //diagonal left down
            if x > 3 && y + 3 < y_len {
                xmasses += isxmas(
                    input.char_field[y][x],
                    input.char_field[y + 1][x - 1],
                    input.char_field[y + 2][x - 2],
                    input.char_field[y + 3][x - 3],
                );
            }

            //diagonal left up
            if x + 3 < x_len && y > 3 {
                xmasses += isxmas(
                    input.char_field[y][x],
                    input.char_field[y - 1][x - 1],
                    input.char_field[y - 2][x - 2],
                    input.char_field[y - 3][x - 3],
                );
            }

        }
    }
    println!("{:#?}", xmasses);
    Ok(())
}

fn pt2(input: &InputStruct) -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let example_input_str = read_to_string(args.example_path)?;
    let real_input_str = read_to_string(args.input_path)?;

    let input = parse_input(&example_input_str);

    let _ = pt1(&input);
    let _ = pt2(&input);

    Ok(())
}
