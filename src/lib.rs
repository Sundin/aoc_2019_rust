use std::error::Error;
use std::fs;

mod fuel;
mod intcode;

pub struct Config {
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }

        let day = args[1].clone();
        let filename = format!("input/day{}.txt", day);

        Ok(Config {
            filename,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // ? will return the error rather than panic
    let contents = fs::read_to_string(config.filename)?;

    day2(contents);

    Ok(())
}

fn day1(contents: String) {
    let total_fuel = fuel::get_total_fuel(&contents);
    let total_fuel_recursive = fuel::get_total_fuel_recursive(&contents);
    println!("Answer for day 1 part 1: {}", total_fuel);
    println!("Answer for day 1 part 2: {}", total_fuel_recursive);
}

fn day2(contents: String) {
    let output = intcode::get_answer(&contents);
    println!("Answer for day 2 part 1: {}", output);

    intcode::try_inputs(&contents, 19690720);
}
