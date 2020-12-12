use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let data = fs::read_to_string("inputs/day7_in.txt")?;
    //cleanup
    let data = data
        .replace("bags", "bag")
        .replace(" bag", "")
        .replace(".", "");

    let mut map = HashMap::<&str, Vec<_>>::new();
    for line in data.lines() {
        let mut split = line.split(" contain ");
        let key = split.next().unwrap();
        let values = split.next().unwrap();
        if values.chars().next().unwrap().is_numeric() {
            let values: Vec<_> = values
                .split(", ")
                .map(|value| {
                    let num = value[0..1].parse::<u32>().unwrap();
                    let color = &value[2..];
                    (num, color)
                })
                .collect();
            map.insert(key, values);
        }
    }
    
    let mut reverse = HashMap::<&str, Vec<&str>>::new();
    for (k, v) in &map {
        for (_, color) in v {
            if let Some(value) = reverse.get_mut(color) {
                value.push(k);
            } else {
                reverse.insert(color, vec![k]);
            }
        }
    }
    // println!("reverse: {:#?}", reverse);
    
    let bags = visit("shiny gold", &reverse);
    let bags: HashSet<_> = bags.iter().collect();
    // println!("bags: {:#?}", bags);
    println!("{} bags can hold shiny gold", bags.len() - 1);
    
    // println!("forward: {:#?}", map);
    let count = count_bags("shiny gold", &map);
    println!("A shiny gold bag holds {} other bags", count - 1);
    
    Ok(())
}

// counts original as well
fn visit<'a>(color: &'a str, reverse: &'a HashMap<&str, Vec<&str>>) -> Vec<&'a str> {
    let mut v = vec![color];
    if let Some(list) = reverse.get(color) {
        let mut nested = list.iter().flat_map(|c| visit(c, reverse)).collect();
        v.append(&mut nested);
    }
    v
}

// counts original as well
fn count_bags<'a>(color: &'a str, map: &'a HashMap<&str, Vec<(u32, &str)>>) -> u32 {
    // println!("checking {}", color);
    if let Some(bags) = map.get(color) {
        let c: u32 = bags.iter().map(|(num, c)| num * count_bags(c, map)).sum();
        // println!("{} can hold {}", color, c);
        c + 1
    } else {
        // println!("{} can't hold more bags", color);
        1
    }
}
