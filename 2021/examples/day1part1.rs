use std::io::BufRead;

fn main() {
    let mut count = 0;
    let stdin = std::io::stdin();
    let lines = stdin.lock().lines();
    let mut previous = None;
    for line in lines {
        let number: i32 = line.unwrap().trim().parse().unwrap();
        if let Some(previous) = previous {
            if number > previous {
                count += 1;
            }
        }
        previous = Some(number);
    }
    println!("{}", count);
}
