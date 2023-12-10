use std::io::BufRead;

fn main() {
    let mut count = 0;
    let stdin = std::io::stdin();
    let lines: Vec<i32> = stdin
        .lock()
        .lines()
        .map(|l| l.unwrap().parse().unwrap())
        .collect();
    let mut previous = None;
    for line in lines.windows(3) {
        let number: i32 = line.iter().sum();
        if let Some(previous) = previous {
            if number > previous {
                count += 1;
            }
        }
        previous = Some(number);
    }
    println!("{}", count);
}
