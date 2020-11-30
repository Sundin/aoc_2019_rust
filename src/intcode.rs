
pub fn get_answer(contents: &str) -> i32 {
    let output = get_output(&mut [1,0,0,0,99], 0);
    output
}

fn get_output(intcodes: &mut[i32], pointer: usize) -> i32 {
    println!("{:?} pointer {}", intcodes, pointer);

    let operation = intcodes[pointer];

    match operation {
        1 => {
            let parameter1 = intcodes[intcodes[pointer+1] as usize];
            let parameter2 = intcodes[intcodes[pointer+2] as usize];
            let destination = intcodes[pointer+3];
            println!("{} + {} = {} => pos {}", parameter1, parameter2, parameter1+parameter2, destination);
            intcodes[destination as usize] = parameter1 + parameter2;
        },
        2 => println!("two"),
        99 => {
            println!("Finished!");
            return intcodes[0];
        },
        _ => println!("something else"),
    }
    println!("{:?}", intcodes);
    get_output(intcodes, pointer+4)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_addition() {
        assert_eq!(2, get_output(&mut [1,0,0,0,99], 0));
    }
}