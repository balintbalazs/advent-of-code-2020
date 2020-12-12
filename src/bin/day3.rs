use std::error::Error;

use advent_of_code_2020::read_lines;

fn main() -> Result<(), Box<dyn Error>> {
    let slopes = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let width = 31;
    let mut line_number = 0;
    let mut trees = [0; 5];
    for line in read_lines("inputs/day3_in.txt")? {
        let line = line?;
        let line = line.as_bytes();
        for i in 0..5 {
            if line_number % slopes[i].1 == 0 {
                if line[(slopes[i].0 * line_number / slopes[i].1) % width] == '#' as u8 {
                    trees[i] = trees[i] + 1;
                }
            }
        }
        line_number = line_number + 1;
    }
    println!("Found {:#?} trees", trees);
    println!("Product is {}", trees.iter().product::<i64>());
    Ok(())
}
