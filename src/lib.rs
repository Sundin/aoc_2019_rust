use std::cmp;
use std::error::Error;
use std::fs;

pub struct Config {
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }

        let filename = "input/day".to_owned() + &args[1].clone() + ".txt";

        Ok(Config {
            filename,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // ? will return the error rather than panic
    let contents = fs::read_to_string(config.filename)?;

    let total_fuel = get_total_fuel(&contents);
    let total_fuel_recursive = get_total_fuel_recursive(&contents);
    println!("Answer for day 1 part 1: {}", total_fuel);
    println!("Answer for day 1 part 2: {}", total_fuel_recursive);

    Ok(())
}

fn get_total_fuel(contents: &str) -> i32 {
    let mut fuel = 0;
    for line in contents.lines() {
        let line: i32 = match line
            .trim()
            .parse() {
                Ok(num) => num,
                // _ = match all Err values (place for proper error handling)
                Err(_) => {
                    println!("Please type a number!");
                    continue;
                }
            };
        fuel = fuel + get_fuel_requirements(line);
    }
    fuel
}

fn get_total_fuel_recursive(contents: &str) -> i32 {
    let mut fuel = 0;
    for line in contents.lines() {
        let line: i32 = match line
            .trim()
            .parse() {
                Ok(num) => num,
                // _ = match all Err values (place for proper error handling)
                Err(_) => {
                    println!("Please type a number!");
                    continue;
                }
            };
        fuel = fuel + get_fuel_requirements_recursive(line);
    }
    fuel
}

fn get_fuel_requirements(mass: i32) -> i32 {
    mass / 3 - 2
}

fn get_fuel_requirements_recursive(mass: i32) -> i32 {
    let mut fuel = get_fuel_requirements(mass);
    if (fuel > 0) {
        fuel = fuel + cmp::max(0, get_fuel_requirements_recursive(fuel));
    }
    fuel
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fuel_requirements() {
        assert_eq!(2, get_fuel_requirements(12));
        assert_eq!(2, get_fuel_requirements(14));
        assert_eq!(654, get_fuel_requirements(1969));
        assert_eq!(33583, get_fuel_requirements(100756));
    }

    #[test]
    fn fuel_requirements_recursive() {
        assert_eq!(2, get_fuel_requirements_recursive(14));
        assert_eq!(966, get_fuel_requirements_recursive(1969));
        assert_eq!(50346, get_fuel_requirements_recursive(100756));
    }
}