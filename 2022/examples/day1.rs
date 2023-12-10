use std::{
    error::Error,
    fs::File,
    io::{BufReader, Read},
};

fn group(buf: &str) -> Result<Vec<i32>, Box<dyn Error>> {
    let mut groups: Vec<i32> = buf
        .split("\n\n")
        .map(|group| {
            group
                .trim()
                .split('\n')
                .map(|line| line.parse::<i32>().expect("invalid number"))
                .sum::<i32>()
        })
        .collect();

    groups.sort();

    Ok(groups)
}

fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("data/day1.txt")?;
    let mut f = BufReader::new(f);
    let mut buf = String::new();
    f.read_to_string(&mut buf)?;

    let groups = group(&buf)?;

    let top_three: Vec<_> = groups.iter().rev().take(3).collect();
    dbg!(&top_three);
    dbg!(top_three.into_iter().sum::<i32>());

    Ok(())
}
