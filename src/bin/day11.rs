use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let data = fs::read_to_string("day11_in.txt")?;
    let mut seats: Vec<_> = data
        .lines()
        .map(|l| {
            let row: Vec<_> = l.chars().collect();
            row
        })
        .collect();

    // print(&seats);
    // let seats = iterate(&seats);
    // print(&seats);
    // let seats = iterate(&seats);
    // print(&seats);
    // let seats = iterate(&seats);
    // print(&seats);
    let mut counter = 0;
    loop {
        let next = iterate(&seats);
        counter = counter + 1;
        if next == seats {
            break;
        }
        seats = next;
    }
    println!("did {} iterations", counter);

    let occupied: i32 = seats
        .iter()
        .map(|row| row.iter().map(|c| if c == &'#' { 1 } else { 0 }).sum::<i32>())
        .sum();

    println!("occupied seats: {}", occupied);

    Ok(())
}

fn iterate(seats: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut next = seats.clone();
    for i in 0..next.len() {
        for j in 0..next[i].len() {
            if seats[i][j] != '.' {
                let occupied = occupant_count(&seats, i, j);
                if seats[i][j] == 'L' && occupied == 0 {
                    next[i][j] = '#';
                } else if seats[i][j] == '#' && occupied >= 4 {
                    next[i][j] = 'L';
                }
            }
        }
    }
    next
}

fn occupant_count(seats: &Vec<Vec<char>>, i: usize, j: usize) -> u32 {
    let mut count = 0;
    let i = i as i32;
    let j = j as i32;
    for di in vec![-1, 0, 1] {
        for dj in vec![-1, 0, 1] {
            if di == 0 && dj == 0 {
                continue;
            }
            // top or left edge
            if i + di < 0 || j + dj < 0 {
                continue;
            }
            // bottom or right edge
            if (i + di) as usize >= seats.len() || (j + dj) as usize >= seats[i as usize].len() {
                continue;
            }
            if seats[(i + di) as usize][(j + dj) as usize] == '#' {
                count = count + 1;
            }
        }
    }
    count
}

fn print(a: &Vec<Vec<char>>) {
    for line in a {
        for c in line {
            print!("{}", c);
        }
        println!();
    }
    println!();
}
