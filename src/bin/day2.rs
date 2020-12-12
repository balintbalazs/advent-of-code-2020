use std::error::Error;

use advent_of_code_2020::read_lines;
use scan_fmt::scan_fmt;

fn main() -> Result<(), Box<dyn Error>> {
    let mut valid = (0, 0);
    for line in read_lines("inputs/day2_in.txt")? {
        if let Ok(line) = line {
            if check_rule1(&line) {
                valid.0 = valid.0 + 1;
            }
            if check_rule2(&line) {
                valid.1 = valid.1 + 1;
            }
        }
    }
    println!("{} passwords are valid with rule 1", valid.0);
    println!("{} passwords are valid with rule 2", valid.1);
    Ok(())
}

fn check_rule1(input: &str) -> bool {
    let (min, max, c, s) = scan_fmt!(input, "{d}-{d} {}: {}", u8, u8, char, String).unwrap();
    let mut count = 0;
    for char in s.chars() {
        if char == c {
            count = count + 1;
        }
    }
    count >= min && count <= max
}

fn check_rule2(input: &str) -> bool {
    let (i, j, c, s) = scan_fmt!(input, "{d}-{d} {}: {}", usize, usize, char, String).unwrap();
    let c = c as u8;
    let s = s.as_bytes();
    (s[i - 1] == c) ^ (s[j - 1] == c)
}

#[test]
fn can_check() {
    assert!(check_rule1("1-3 a: abcde"));
    assert!(!check_rule1("1-3 b: cdefg"));
    assert!(check_rule1("2-9 c: ccccccccc"));
}

#[test]
fn can_check2() {
    assert!(check_rule2("1-3 a: abcde"));
    assert!(!check_rule2("1-3 b: cdefg"));
    assert!(!check_rule2("2-9 c: ccccccccc"));
}
