
pub fn get_answer(contents: &str) -> i32 {
    let mut intcodes = contents.split(",").map(|x| to_number(x)).collect::<Vec<i32>>();
    let output = get_output(&mut intcodes, 0);
    output
}

pub fn try_inputs(contents: &str, desired_result: i32) {
    for noun in 0..100 {
        for verb in 0..100 {
            let mut intcodes = contents.split(",").map(|x| to_number(x)).collect::<Vec<i32>>();
            intcodes[1] = noun;
            intcodes[2] = verb;
            let output = get_output(&mut intcodes, 0);
            if output == desired_result {
                println!("Answer for day 2 part 2: {}", noun * 100 + verb);
                return
            }
        }
    }
    println!("Not found...");
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

fn get_output(intcodes: &mut Vec<i32>, pointer: usize) -> i32 {
    let operation = intcodes[pointer];

    match operation {
        1 => {
            let parameter1 = intcodes[intcodes[pointer+1] as usize];
            let parameter2 = intcodes[intcodes[pointer+2] as usize];
            let destination = intcodes[pointer+3];
            intcodes[destination as usize] = parameter1 + parameter2;
        },
        2 => {
            let parameter1 = intcodes[intcodes[pointer+1] as usize];
            let parameter2 = intcodes[intcodes[pointer+2] as usize];
            let destination = intcodes[pointer+3];
            intcodes[destination as usize] = parameter1 * parameter2;
        },
        99 => {
            return intcodes[0];
        },
        _ => println!("something else"),
    }
    get_output(intcodes, pointer+4)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_addition() {
        let mut v: Vec<i32> = vec![1,0,0,0,99];
        assert_eq!(2, get_output(&mut v, 0));
    }

    #[test]
    fn test_multiplication() {
        let mut v: Vec<i32> = vec![2,3,0,3,99];
        assert_eq!(2, get_output(&mut v, 0));
        let mut v: Vec<i32> = vec![2,4,4,5,99,0];
        assert_eq!(2, get_output(&mut v, 0))
    }

    #[test]
    fn test_multiple_rounds() {
        let mut v: Vec<i32> = vec![1,1,1,4,99,5,6,0,99];
        assert_eq!(30, get_output(&mut v, 0));
    }
}