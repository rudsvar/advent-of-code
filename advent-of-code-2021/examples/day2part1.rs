use std::io::BufRead;

fn main() {
    let stdin = std::io::stdin();
    let lines = stdin.lock().lines();
    let mut horizontal_position = 0;
    let mut depth = 0;
    for line in lines {
        let line = line.unwrap();
        let mut line = line.split_whitespace();
        let command = line.next().unwrap();
        let number: i32 = line.next().unwrap().parse().unwrap();
        match command {
            "forward" => {
                horizontal_position += number;
            }
            "down" => {
                depth += number;
            }
            "up" => {
                depth -= number;
            }
            _ => {}
        }
    }
    println!("{}", horizontal_position * depth);
}
