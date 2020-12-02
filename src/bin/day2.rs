use scan_fmt::scan_fmt;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut valid = (0, 0);
    if let Ok(lines) = read_lines("day2_in.txt") {
        for line in lines {
            if let Ok(line) = line {
                if check_rule1(&line) {
                    valid.0 = valid.0 + 1;
                }
                if check_rule2(&line) {
                    valid.1 = valid.1 + 1;
                }
            }
        }
    }
    println!("{} passwords are valid with rule 1", valid.0);
    println!("{} passwords are valid with rule 2", valid.1);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
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
    (s[i-1] == c) ^ (s[j-1] == c)
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
