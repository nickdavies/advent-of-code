use std::{
    fs::{File, OpenOptions},
    io::Write,
    process,
};

use crate::Day;

const MODULE_TEMPLATE: &str = r#"#![allow(unused_imports)]
advent_of_code::solution!(DAY_NUMBER);

use advent_of_code::template::RunType;

pub fn part_one(_input: &str, _run_type: RunType) -> Result<Option<u32>, anyhow::Error> {
    Ok(None)
}

pub fn part_two(_input: &str, _run_type: RunType) -> Result<Option<u32>, anyhow::Error> {
    Ok(None)
}

#[cfg(test)]
mod tests_day_DAY_NUMBER {
    use super::*;

    #[test]
    fn test_part_one() -> anyhow::Result<()> {
        let expected = None;
        let input = &advent_of_code::template::read_file_part("examples", DAY, 1);
        assert!(expected.is_none() || !input.is_empty(), "example 1 empty!");
        let result = part_one(input, RunType::Example)?;
        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn test_part_two() -> anyhow::Result<()> {
        let expected = None;
        let input = &advent_of_code::template::read_file_part("examples", DAY, 2);
        assert!(expected.is_none() || !input.is_empty(), "example 2 empty!");
        let result = part_two(input, RunType::Example)?;
        assert_eq!(result, expected);
        Ok(())
    }
}
"#;

fn safe_create_file(path: &str) -> Result<File, std::io::Error> {
    OpenOptions::new().write(true).create_new(true).open(path)
}

fn create_file(path: &str) -> Result<File, std::io::Error> {
    OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(false)
        .open(path)
}

pub fn handle(day: Day) {
    let input_path = format!("data/inputs/{day}.txt");
    let example_path_1 = format!("data/examples/{day}-1.txt");
    let example_path_2 = format!("data/examples/{day}-2.txt");
    let module_path = format!("src/bin/{day}.rs");

    let mut file = match safe_create_file(&module_path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Failed to create module file: {e}");
            process::exit(1);
        }
    };

    match file.write_all(
        MODULE_TEMPLATE
            .replace("DAY_NUMBER", &day.into_inner().to_string())
            .as_bytes(),
    ) {
        Ok(()) => {
            println!("Created module file \"{}\"", &module_path);
        }
        Err(e) => {
            eprintln!("Failed to write module contents: {e}");
            process::exit(1);
        }
    }

    match create_file(&input_path) {
        Ok(_) => {
            println!("Created empty input file \"{}\"", &input_path);
        }
        Err(e) => {
            eprintln!("Failed to create input file: {e}");
            process::exit(1);
        }
    }

    match create_file(&example_path_1) {
        Ok(_) => {
            println!("Created empty part 1 example file \"{}\"", &example_path_1);
        }
        Err(e) => {
            eprintln!("Failed to create example file: {e}");
            process::exit(1);
        }
    }

    match std::os::unix::fs::symlink(format!("{day}-1.txt"), &example_path_2) {
        Ok(_) => {
            println!(
                "Symlinked part 2 example file to parts 1 file \"{}\"",
                &example_path_2
            );
        }
        Err(e) => {
            eprintln!("Failed to create example 2 symlink: {e}");
            process::exit(1);
        }
    }

    println!("---");
    println!("🎄 Type `cargo solve {}` to run your solution.", day);
}
