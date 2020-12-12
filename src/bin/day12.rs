use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let data = fs::read_to_string("day12_in.txt")?;
    let instructions : Vec<_> = data.lines().map(|l| {
        let instr: char = l.chars().next().unwrap();
        let num: i32 = l[1..].parse().unwrap();
        (instr, num)
    }).collect();

    let mut north = 0;
    let mut east = 0;
    let mut heading = 0;

    for (ref i, ref n) in &instructions {
        match i {
            'N' => north = north + n,
            'S' => north = north - n,
            'E' => east = east + n,
            'W' => east = east - n,
            'R' => heading = (heading - n + 360) % 360,
            'L' => heading = (heading + n + 360) % 360,
            'F' => match heading {
                0 => east = east + n,
                90 => north = north + n,
                180 => east = east - n,
                270 => north = north - n,
                _ => panic!("heading: {}", heading),
            },
            _ => panic!(),
        }
    }

    dbg!(north, east, heading);
    dbg!(north.abs() + east.abs());

    let mut north = 0;
    let mut east = 0;
    let mut heading = (10, 1);

    for (i, n) in instructions {
        match i {
            'N' => heading.1 = heading.1 + n,
            'S' => heading.1 = heading.1 - n,
            'E' => heading.0 = heading.0 + n,
            'W' => heading.0 = heading.0 - n,
            'R' => heading = rotate(heading, -n),
            'L' => heading = rotate(heading, n),
            'F' => {
                north = north + n * heading.1;
                east = east + n * heading.0;
            }
            _ => panic!(),
        }
        // dbg!(north, east);
    }

    dbg!(north, east, heading);
    dbg!(north.abs() + east.abs());

    Ok(())
}

fn rotate(heading: (i32, i32), angle: i32) -> (i32, i32) {
    // let s =
    match angle {
        0 => heading,
        90 | -270 => (-heading.1, heading.0),
        -90 | 270 => (heading.1, -heading.0),
        180 | -180 => (-heading.0, -heading.1),
        _ => panic!("angle: {}", angle),
    }
}
