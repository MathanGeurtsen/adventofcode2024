use anyhow::{anyhow, Result};
use clap::Parser as ArgParser;
use std::fs::read_to_string;

#[derive(Debug)]
struct Input {
    grid: Vec<Vec<char>>,
}

fn parse_input(input_str: &str) -> Result<Input> {
    let mut grid = Vec::new();

    for line in input_str.lines() {
        let mut row = Vec::new();
        for c in line.chars() {
            row.push(c);
        }
        grid.push(row);
    }

    Ok(Input { grid })
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
fn move_guard(input: &Input) -> Option<(usize,usize)> {
    let cur_pos = input
        .grid
        .iter()
        .enumerate()
        .find_map(|(row_id, row)| {
            row.iter()
                .enumerate()
                .find_map(|(col_id, &c)| if vec!['^', '>', '<', 'v'].contains(&c) { Some((row_id, col_id, c)) } else { None })
        });

    let new_pos = match cur_pos?.2 {
        '^' => Some((cur_pos?.0 - 1, cur_pos?.1)),
        'v' => Some((cur_pos?.0 + 1, cur_pos?.1)),
        '<' => Some((cur_pos?.0,     cur_pos?.1 - 1)),
        '>' => Some((cur_pos?.0,     cur_pos?.1 + 1)),
        _ => None
    };

    Some()
} 

fn pt1(input: &Input) -> Result<i64> {


    Err(anyhow!(""))
}

fn pt2(input: &Input) -> Result<i64> {
    Err(anyhow!(""))
}

fn main() -> Result<()> {
    let args = Args::parse();
    let example_input_str = read_to_string(args.example_path)?;
    let real_input_str = read_to_string(args.input_path)?;

    let example_input = parse_input(&example_input_str)?;
    println!("{:#?}", example_input);
    let real_input = parse_input(&real_input_str)?;

    let pt1_example_res = pt1(&example_input);
    println!("pt1 example, expected 143, result: {:#?}", pt1_example_res);
    let pt2_example_res = pt2(&example_input);
    println!("pt2 example, expected 123, result: {:#?}", pt2_example_res);

    //
    //let pt1_real_res = pt1(&real_input);
    //println!("pt1 real: {:#?}", pt1_real_res);
    //
    //let pt2_real_res = pt2(&real_input);
    //println!("pt2 real: {:#?}", pt2_real_res);
    //

    Ok(())
}
