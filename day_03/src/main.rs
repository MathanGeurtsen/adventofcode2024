use clap::Parser as ArgParser;
use regex::Regex;
use std::fs::read_to_string;

#[derive(Debug)]
struct InputStruct {
    muls: Vec<(i32, i32)>,
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
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut input = InputStruct { muls: Vec::new() };

    for cap in re.captures_iter(input_str) {
        let num1: i32 = cap[1].parse().unwrap();
        let num2: i32 = cap[2].parse().unwrap();
        input.muls.push((num1, num2));
    }
    input
}

fn pt1(input: &InputStruct) -> Result<(), Box<dyn std::error::Error>> {
    let total: i32 = input.muls.iter().map(|&(a, b)| a * b).sum();
    println!("{:#?}", total);
    Ok(())
}

/// much easier to just parse and calculate at the same time so use input string
fn pt2(input: &str) -> Result<(), Box<dyn std::error::Error>> {
    let re = Regex::new(r"(don't\(\)|do\(\)|mul\((\d+),(\d+)\))").unwrap();
    let mut enabled = true;
    let mut total = 0;

    for cap in re.captures_iter(input) {
        match &cap[0] {
            "don't()" => enabled = false,
            "do()" => enabled = true,
            _ if enabled => {
                if let (Some(num1), Some(num2)) = (
                    cap.get(2).and_then(|m| m.as_str().parse::<i32>().ok()),
                    cap.get(3).and_then(|m| m.as_str().parse::<i32>().ok()),
                ) {
                    total += num1 * num2;
                }
            }
            _ => (),
        }
    }
    println!("total: {:#?}", total);

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let example_input_str = read_to_string(args.example_path)?;
    let real_input_str = read_to_string(args.input_path)?;

    let input = parse_input(&real_input_str);

    let _ = pt1(&input);
    let _ = pt2(&real_input_str);

    Ok(())
}
