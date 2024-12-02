use clap::Parser as ArgParser;
use combine::parser::char::{digit, newline, string};
use combine::parser::repeat::sep_by1;
use combine::stream::Stream;
use combine::Parser;
use combine::{many1, EasyParser};
use std::fs::read_to_string;

#[derive(Debug)]
struct InputStruct {
    reports: Vec<Vec<i64>>,
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

fn parse_input(
    input: &str,
) -> std::result::Result<
    InputStruct,
    combine::easy::Errors<char, String, combine::stream::PointerOffset<str>>,
> {
    let result = (int_list())
        .map(|location_lists| InputStruct {
            reports: location_lists,
        })
        .easy_parse(input);

    match result {
        Ok((result, _rest)) => Ok(result),
        Err(err) => Err(err.map_range(|err: &str| err.to_owned())),
    }
}

/// Parse a list of integers separated
fn int_list<Input>() -> impl Parser<Input, Output = Vec<Vec<i64>>>
where
    Input: Stream<Token = char>,
{
    many1::<Vec<Vec<i64>>, _, _>(sep_by1(integer_parser(), string(" ")).skip(newline()))
}

/// Parse a single integer
fn integer_parser<Input>() -> impl Parser<Input, Output = i64>
where
    Input: Stream<Token = char>,
{
    many1(digit()).map(|digits: String| digits.parse::<i64>().unwrap())
}

fn pt1(input: &InputStruct) -> Result<(), Box<dyn std::error::Error>> {
    let mut safe_reports = 0;
    for report in &input.reports {
        let mut always_increasing = true;
        let mut always_decreasing = true;
        let mut small_diff = true;
        for i in 0..report.len() - 1 {
            let diff = report[i + 1] - report[i];

            always_increasing &= diff > 0;
            always_decreasing &= diff < 0;
            small_diff &= diff.abs() <= 3;
        }
        if (always_increasing | always_decreasing) & small_diff {
            safe_reports += 1;
        }
    }
    println!("{:#?}", safe_reports);

    Ok(())
}

fn pt2(input: &InputStruct) -> Result<(), Box<dyn std::error::Error>> {
    let mut safe_reports = 0;
    for report in &input.reports {
        let mut new_reports = Vec::new();
        let mut is_safe = false;

        for i in 0..report.len() {
            let mut new_report = report.clone();
            new_report.remove(i);
            new_reports.push(new_report);
        }

        for new_report in new_reports {
            let mut always_increasing = true;
            let mut always_decreasing = true;
            let mut small_diff = true;
            for i in 0..new_report.len() - 1 {
                let diff = new_report[i + 1] - new_report[i];

                always_increasing &= diff > 0;
                always_decreasing &= diff < 0;
                small_diff &= diff.abs() <= 3;
            }
            if (always_increasing | always_decreasing) & small_diff {
                is_safe = true;
                break;
            }
        }
        if is_safe {
            safe_reports += 1;
        }
    }
    println!("{:#?}", safe_reports);

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let example_input_str = read_to_string(args.example_path)?;
    let real_input_str = read_to_string(args.input_path)?;
    let example_input = parse_input(example_input_str.as_str())?;
    let input = parse_input(real_input_str.as_str())?;
    println!("{:#?}", example_input);
    let _ = pt1(&input);
    let _ = pt2(&input);
    Ok(())
}
