use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let data = fs::read_to_string("inputs/day13_in.txt")?;
    let mut lines = data.lines();
    let target: i32 = lines.next().unwrap().parse().unwrap();
    let (id, wait) = lines
        .next()
        .unwrap()
        .split(',')
        .filter_map(|c| c.parse::<i32>().ok())
        .map(|n| (n, ((n - target) % n) + n))
        .fold(
            (0, i32::MAX),
            |acc, (id, wait)| if wait < acc.1 { (id, wait) } else { acc },
        );

    dbg!(id, wait);
    dbg!(id * wait);
    
    // part 2
    let mut lines = data.lines();
    let _ = lines.next();
    let in2 = lines
        .next()
        .unwrap();
    // let in2 = "1789,37,47,1889";
    let ids: Vec<_> = in2.split(',')
        .zip(0..)
        .filter_map(|(id, idx)| id.parse::<u64>().map(|id| (id, idx)).ok())
        .collect();
    dbg!(&ids);

    let mut time: u64= 0;
    let mut step = ids[0].0;
    let mut curr = 1;
    while curr < ids.len() {
        while (time + ids[curr].1) % ids[curr].0 != 0 {
            time = time + step;
        }
        // found correct time for buses 0..curr
        step = step * ids[curr].0;
        curr = curr + 1;
    }
    dbg!(time);
    Ok(())
}
