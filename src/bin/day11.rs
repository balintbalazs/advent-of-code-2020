use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let data = fs::read_to_string("inputs/day11_in.txt")?;
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
        .map(|row| {
            row.iter()
                .map(|c| if c == &'#' { 1 } else { 0 })
                .sum::<i32>()
        })
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
                } else if seats[i][j] == '#' && occupied >= 5 {
                    next[i][j] = 'L';
                }
            }
        }
    }
    next
}

fn occupant_count(seats: &Vec<Vec<char>>, i: usize, j: usize) -> u32 {
    let mut count = 0;

    let height = seats.len();
    let width = seats[0].len();

    let mut di = 1;
    while i + di < height {
        if seats[i + di][j] == '#' {
            count = count + 1;
            break;
        }
        if seats[i + di][j] == 'L' {
            break;
        }
        di = di + 1;
    }

    let mut di = 1;
    while i >= di {
        if seats[i - di][j] == '#' {
            count = count + 1;
            break;
        }
        if seats[i - di][j] == 'L' {
            break;
        }
        di = di + 1;
    }

    let mut dj = 1;
    while j + dj < width {
        if seats[i][j + dj] == '#' {
            count = count + 1;
            break;
        }
        if seats[i][j + dj] == 'L' {
            break;
        }
        dj = dj + 1;
    }

    let mut dj = 1;
    while j >= dj {
        if seats[i][j - dj] == '#' {
            count = count + 1;
            break;
        }
        if seats[i][j - dj] == 'L' {
            break;
        }
        dj = dj + 1;
    }

    let (mut di, mut dj) = (1, 1);
    while i + di < height && j + dj < width {
        if seats[i + di][j + dj] == '#' {
            count = count + 1;
            break;
        }
        if seats[i + di][j + dj] == 'L' {
            break;
        }
        di = di + 1;
        dj = dj + 1;
    }

    let (mut di, mut dj) = (1, 1);
    while i >= di && j + dj < width {
        if seats[i - di][j + dj] == '#' {
            count = count + 1;
            break;
        }
        if seats[i - di][j + dj] == 'L' {
            break;
        }
        di = di + 1;
        dj = dj + 1;
    }

    let (mut di, mut dj) = (1, 1);
    while i + di < height && j >= dj {
        if seats[i + di][j - dj] == '#' {
            count = count + 1;
            break;
        }
        if seats[i + di][j - dj] == 'L' {
            break;
        }
        di = di + 1;
        dj = dj + 1;
    }

    let (mut di, mut dj) = (1, 1);
    while i >= di && j >= dj {
        if seats[i - di][j - dj] == '#' {
            count = count + 1;
            break;
        }
        if seats[i - di][j - dj] == 'L' {
            break;
        }
        di = di + 1;
        dj = dj + 1;
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
