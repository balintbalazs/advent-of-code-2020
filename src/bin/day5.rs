use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let data = fs::read_to_string("day5_in.txt")?;
    let data = data.replace("B", "1");
    let data = data.replace("F", "0");
    let data = data.replace("R", "1");
    let data = data.replace("L", "0");
    let mut ids: Vec<_> = data
        .split_whitespace()
        .map(|b| u32::from_str_radix(b, 2).unwrap())
        .collect();

    let max = ids.iter().max().unwrap();
    dbg!(max);
    
    ids.sort();
    for i in 0..(ids.len()-2) {
        if ids[i+1] - ids[i] != 1 {
            let missing = ids[i] + 1;
            dbg!(missing);
        }
    }

    Ok(())
}
