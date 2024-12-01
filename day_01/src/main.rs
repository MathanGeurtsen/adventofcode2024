use clap::Parser as ArgParser;
use combine::parser::char::{digit, newline, string};
use combine::parser::repeat::sep_by1;
use combine::stream::Stream;
use combine::Parser;
use combine::{many1, EasyParser};
use std::fs::read_to_string;

#[derive(Debug)]
struct InputStruct {
    location_lists: Vec<Vec<i64>>,
}

#[derive(Debug)]
struct LocationLists {
    first: Vec<i64>,
    second: Vec<i64>,
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
    let result = (two_ints_list())
        .map(|location_lists| InputStruct { location_lists })
        .easy_parse(input);

    match result {
        Ok((result, _rest)) => Ok(result),
        Err(err) => Err(err.map_range(|err: &str| err.to_owned())),
    }
}

fn two_ints<Input>() -> impl Parser<Input, Output = Vec<i64>>
where
    Input: Stream<Token = char>,
{
    sep_by1(integer_parser(), string("   "))
}

fn two_ints_list<Input>() -> impl Parser<Input, Output = Vec<Vec<i64>>>
where
    Input: Stream<Token = char>,
{
    many1::<Vec<Vec<i64>>, _, _>(two_ints().skip(newline()))
}

fn integer_parser<Input>() -> impl Parser<Input, Output = i64>
where
    Input: Stream<Token = char>,
{
    // Parse one or more digits, then combine them into an integer
    many1(digit()).map(|digits: String| digits.parse::<i64>().unwrap())
}

fn separate_lists(input: &InputStruct) -> LocationLists {
    let mut location_lists = LocationLists {
        first: vec![],
        second: vec![],
    };
    for line in &input.location_lists {
        location_lists.first.push(line[0]);
        location_lists.second.push(line[1]);
    }

    location_lists
}

fn pt1(input: &InputStruct) -> Result<(), Box<dyn std::error::Error>> {
    let mut location_lists = separate_lists(input);
    location_lists.first.sort();
    location_lists.second.sort();

    let diff: Vec<i64> = location_lists
        .first
        .iter()
        .zip(location_lists.second.iter())
        .map(|(first, second)| (first - second).abs())
        .collect();
    let total: i64 = diff.iter().sum();
    println!("total diff: {:#?}", total);

    Ok(())
}

fn pt2(input: &InputStruct) -> Result<(), Box<dyn std::error::Error>> {
    let location_lists = separate_lists(input);
    let mut sim_score = 0;
    for nr in location_lists.first.iter() {
        let tot = location_lists.second.iter().filter(|&&x| x == *nr).count();
        sim_score += tot as i64 * *nr;
    }

    println!("{:#?}", sim_score);
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
