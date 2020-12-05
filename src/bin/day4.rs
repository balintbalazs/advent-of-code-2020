use pp::from_str;
use regex::Regex;
use serde::Deserialize;
use std::error::Error;
use std::fs;

#[derive(Debug, Deserialize)]
struct Passport {
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<String>,
}

impl Passport {
    fn valid(&self) -> bool {
        if self.byr.is_none() {
            return false;
        }
        if self.iyr.is_none() {
            return false;
        }
        if self.eyr.is_none() {
            return false;
        }
        if self.hgt.is_none() {
            return false;
        }
        if self.hcl.is_none() {
            return false;
        }
        if self.ecl.is_none() {
            return false;
        }
        if self.pid.is_none() {
            return false;
        }
        true
    }

    fn strictly_valid(&self) -> bool {
        if !valid_year(&self.byr, 1920, 2002) {
            return false;
        }
        if !valid_year(&self.iyr, 2010, 2020) {
            return false;
        }
        if !valid_year(&self.eyr, 2020, 2030) {
            return false;
        }
        // height
        if let Some(hgt) = &self.hgt {
            match &hgt.as_str()[(hgt.len() - 2)..] {
                "cm" => {
                    let hgt = &hgt.as_str()[..(hgt.len() - 2)];
                    if let Ok(hgt) = hgt.parse::<u32>() {
                        if hgt < 150 || hgt > 193 {
                            return false;
                        }
                    }
                }
                "in" => {
                    let hgt = &hgt.as_str()[..(hgt.len() - 2)];
                    if let Ok(hgt) = hgt.parse::<u32>() {
                        if hgt < 59 || hgt > 76 {
                            return false;
                        }
                    }
                }
                _ => return false,
            }
        } else {
            return false;
        }
        // hair color
        let re = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
        if let Some(hcl) = &self.hcl {
            if !re.is_match(hcl) {
                return false;
            }
        } else {
            return false;
        }

        // eye color
        let colors = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
        if let Some(ecl) = &self.ecl {
            if !colors.contains(&ecl.as_str()) {
                return false;
            }
        } else {
            return false;
        }

        // passport id - 9 digits
        let re = Regex::new(r"^\d{9}$").unwrap();
        if let Some(pid) = &self.pid {
            if !re.is_match(pid) {
                return false;
            }
        } else {
            return false;
        }

        true
    }
}

fn valid_year(byr: &Option<String>, min: u32, max: u32) -> bool {
    if let Some(byr) = &byr {
        if byr.len() != 4 {
            return false;
        }
        if let Ok(byr) = byr.parse::<u32>() {
            if byr < min || byr > max {
                return false;
            }
        } else {
            return false;
        }
    } else {
        return false;
    }
    true
}

fn main() -> Result<(), Box<dyn Error>> {
    let data = fs::read_to_string("day4_in.txt")?;
    let data: Vec<Passport> = from_str(&data)?;

    let num_valid: i32 = data.iter().map(|pp| if pp.valid() { 1 } else { 0 }).sum();
    println!("valid passports: {}", num_valid);

    let num_valid: i32 = data
        .iter()
        .map(|pp| if pp.strictly_valid() { 1 } else { 0 })
        .sum();
    println!("strictly valid passports: {}", num_valid);
    Ok(())
}
