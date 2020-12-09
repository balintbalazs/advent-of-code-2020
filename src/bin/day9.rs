use std::collections::HashSet;
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {

    const WINDOW: usize = 25;

    let data = fs::read_to_string("day9_in.txt")?;
    let nums: Vec<_> = data
        .split('\n')
        .map(|l| {
            l.parse::<u64>().unwrap()
        })
        .collect();
    
    let mut target = 0;
    for i in WINDOW..nums.len() {
        target = nums[i];
        if None == find_pairs(target, &nums[i-WINDOW..i]) {
            println!("first invalid value: {}", target);
            break;
        } 
    }

    let mut first = 0;
    let mut last = 1;

    while last < nums.len() {
        let sum: u64 = nums[first..=last].iter().sum();
        if sum > target {
            first = first + 1;
        } else if sum < target {
            last = last + 1;
        } else {
            let smallest  = nums[first..=last].iter().min().unwrap();
            let largest  = nums[first..=last].iter().max().unwrap();
            println!("found sequence, first: {}, last: {}", nums[first], nums[last]);
            println!("smallest = {}, largest = {}", smallest, largest);
            println!("smallest + largest = {}", smallest + largest);
            break;
        }
    }

    Ok(())
}

fn find_pairs(target: u64, nums: &[u64]) -> Option<(u64, u64)> {
    // println!("target: {}", target);
    // println!("previous: {:?}", nums);
    let mut seen = HashSet::<u64>::new();
    for num in nums {
        if let Some(_found) = seen.get(num) {
            return Some((*num, target - num))
        } else {
            if target >= *num {
                seen.insert(target - num);
            }
        }
    }
    None
}
