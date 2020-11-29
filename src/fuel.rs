use std::cmp;

pub fn get_total_fuel(contents: &str) -> i32 {
    let mut fuel = 0;
    for line in contents.lines() {
        fuel += get_fuel_requirements(to_number(line));
    }
    fuel
}

pub fn get_total_fuel_recursive(contents: &str) -> i32 {
    let mut fuel = 0;
    for line in contents.lines() {
        fuel += get_fuel_requirements_recursive(to_number(line));
    }
    fuel
}

fn to_number(line: &str) -> i32 {
    let line: i32 = match line
        .trim()
        .parse() {
            Ok(num) => num,
            // _ = match all Err values (place for proper error handling)
            Err(_) => {
                // TODO: error handling
                println!("Please type a number!");
                0
            }
        };
    line
}

fn get_fuel_requirements(mass: i32) -> i32 {
    mass / 3 - 2
}

fn get_fuel_requirements_recursive(mass: i32) -> i32 {
    let mut fuel = get_fuel_requirements(mass);
    if fuel > 0 {
        fuel += cmp::max(0, get_fuel_requirements_recursive(fuel));
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

    #[test]
    fn string_to_number() {
        assert_eq!(666, to_number("666"));
    }
}