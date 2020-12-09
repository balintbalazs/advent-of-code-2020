use std::collections::HashSet;
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let data = fs::read_to_string("day8_in.txt")?;
    let instructions: Vec<_> = data
        .split('\n')
        .map(|l| {
            let instr = &l[0..3];
            let num = l[4..].parse::<isize>().unwrap();
            (instr, num)
        })
        .collect();

    let (_term, accumulator) = run_code(&instructions);

    println!("state of accumulator: {}", accumulator);

    // part 2
    for i in 0..instructions.len() {
        if instructions[i].0 == "acc" {
            continue 
        }
        let mut i2 = instructions.clone();
        if i2[i].0 == "nop" {
            i2[i].0 = "jmp"
        } else if i2[i].0 == "jmp" {
            i2[i].0 = "nop";
        }
        let (term, acc) = run_code(&i2);
        if term {
            println!("accumulator: {}", acc);
            break;
        }
    }

    Ok(())
}

fn run_code(instructions: &Vec<(&str, isize)>) -> (bool, isize) {
    let mut instruction_pointer = 0;
    let mut accumulator = 0;
    let mut visited_instructions = HashSet::<usize>::new();

    loop {
        if visited_instructions.insert(instruction_pointer) {
            if instruction_pointer >= instructions.len() {
                println!("program terminated");
                return (true, accumulator);
            }
            // println!("instruction: {:?}", instructions[instruction_pointer]);
            match instructions[instruction_pointer].0 {
                "nop" => instruction_pointer = instruction_pointer + 1,
                "acc" => {
                    accumulator = accumulator + instructions[instruction_pointer].1;
                    instruction_pointer = instruction_pointer + 1;
                }
                "jmp" => {
                    instruction_pointer = ((instruction_pointer as isize)
                        + instructions[instruction_pointer].1)
                        as usize;
                }
                _ => panic!("unknown instruction"),
            }
        } else {
            println!(
                "infinite loop found, instruction {} was executed twice",
                instruction_pointer
            );
            return (false, accumulator);
        }
    }
}
