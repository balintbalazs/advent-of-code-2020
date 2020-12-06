use std::collections::HashSet;
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let data = fs::read_to_string("day6_in.txt")?;
    let data = data.split("\n\n");
    let sum1: usize = data
        .clone()
        .map(|d| {
            let mut set = HashSet::<char>::new();
            for c in d.chars().filter(|c| c != &'\n') {
                set.insert(c);
            }
            // println!("{}", set.len());
            set.len()
        })
        .sum();
    println!("sum of counts (anyone): {}", sum1);

    let sum2: usize = data
        .map(|group| {
            let group = group.split_whitespace();
            let sets: Vec<_> = group
                .map(|line| {
                    let mut chars = HashSet::<char>::new();
                    for c in line.chars() {
                        chars.insert(c);
                    }
                    chars
                })
                .collect();
            let mut sets = sets.into_iter();
            let first = sets.next().unwrap();
            let every = sets.fold(first, |a, b| a.intersection(&b).cloned().collect());
            // println!("{}", every.len());
            every.len()
        })
        .sum();
    println!("sum of counts (everyone): {}", sum2);

    Ok(())
}
