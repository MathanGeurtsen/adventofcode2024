use anyhow::{anyhow, Result};
use std::collections::HashSet;
use std::collections::HashMap;
use clap::Parser as ArgParser;
use std::fs::read_to_string;

#[derive(Debug)]
struct Input {
    page_order_rules: Vec<Vec<i64>>,
    update_pages: Vec<Vec<i64>>,
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

/// Parses a line of integers split by a string into a vec of integers.
///
/// # Examples
///
/// ```
/// let vec_of_ints = parse_line_of_split_ints("1,2,3", ",");
/// assert_eq!(vec_of_ints, vec![1,2,3]);
/// ```
fn parse_line_of_split_ints(line: &str, split_pattern: &str) -> Option<Vec<i64>> {
    line.split(&split_pattern)
        .map(|s| s.parse::<i64>().ok())
        .collect()
}

fn parse_input(input: &str) -> Input {
    let mut input_struct = Input {
        page_order_rules: vec![],
        update_pages: vec![],
    };
    for line in input.lines() {
        if let Some(page_order_rule) = parse_line_of_split_ints(line, "|") {
            input_struct.page_order_rules.push(page_order_rule);
        } else if line.is_empty() {
            continue;
        } else if let Some(pages) = parse_line_of_split_ints(line, ",") {
            input_struct.update_pages.push(pages);
        }
    }
    input_struct
}

fn valid_for_rule(update_page: Vec<i64>, page_order_rule: Vec<i64>) -> bool {
    let mut found_tail = false;
    let mut result = true;
    for nr in update_page.iter() {
        if *nr == page_order_rule[1] {
            found_tail = true;
        }
        if *nr == page_order_rule[0] && found_tail {
            println!("rule {:#?} fails for page {:#?}",page_order_rule, update_page);
            result = false;
        }
    }
    result
}


fn pt1(input: &Input) -> Result<i64> {
    let mut lines_failing_rules = HashMap::new();
    for line_nr in 0..input.update_pages.len(){
        let mut fails = vec![];
        for rule_nr in 0..input.page_order_rules.len() {
            let mut found_second = false;
            for nr in input.update_pages[line_nr].iter(){
                if *nr == input.page_order_rules[rule_nr][1] {
                    found_second = true;
                }
                if *nr == input.page_order_rules[rule_nr][0] && found_second {
                    fails.push(rule_nr);
                }
            }
        }
        lines_failing_rules.insert(line_nr, fails);
    }
    //println!("{:#?}", lines_failing_rules);

    let mut mids = vec![];
    for (line_nr, fails) in lines_failing_rules.iter() {
        if fails.is_empty() {
        mids.push(input.update_pages[*line_nr][ input.update_pages[*line_nr].len() / 2]);
        }
    }

    Ok(mids.iter().sum())
}

fn pt2(input: &Input) -> Result<i64> {
    let mut lines_failing_rules = HashMap::new();
    for line_nr in 0..input.update_pages.len() {
        let mut fails = vec![];
        for rule_nr in 0..input.page_order_rules.len() {
            let mut found_second = false;
            for nr in input.update_pages[line_nr].iter() {
                if *nr == input.page_order_rules[rule_nr][1] {
                    found_second = true;
                }
                if *nr == input.page_order_rules[rule_nr][0] && found_second {
                    fails.push(rule_nr);
                }
            }
        }
        lines_failing_rules.insert(line_nr, fails);
    }

    let mut mids = vec![];
    for (line_nr, fails) in lines_failing_rules.iter() {
        if !fails.is_empty() {
            let mut update_page = input.update_pages[*line_nr].clone();
            let mut ordered = false;
            while !ordered {
                ordered = true;
                for rule in input.page_order_rules.iter() {
                    if !valid_for_rule(update_page.clone(), rule.clone()) {
                        ordered = false;
                        let index_head = update_page.iter().position(|&r| r == rule[0]).unwrap();
                        let index_tail = update_page.iter().position(|&r| r == rule[1]).unwrap();
                        update_page.swap(index_head, index_tail);
                    }
                }
            }
            mids.push(update_page[update_page.len() / 2]);
        }
    }

    Ok(mids.iter().sum())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let example_input_str = read_to_string(args.example_path)?;
    let real_input_str = read_to_string(args.input_path)?;

    let example_input = parse_input(&example_input_str);
    println!("{:#?}", example_input);
    let real_input = parse_input(&real_input_str);

    let pt1_example_res = pt1(&example_input);
    println!("pt1 example, expected 143, result: {:#?}", pt1_example_res);
    let pt2_example_res = pt2(&example_input);
    println!("pt2 example, expected 123, result: {:#?}", pt2_example_res);


    let pt1_real_res = pt1(&real_input);
    println!("pt1 real: {:#?}", pt1_real_res);

    let pt2_real_res = pt2(&real_input);
    println!("pt2 real: {:#?}", pt2_real_res);


    Ok(())
}
