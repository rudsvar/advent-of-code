use std::{
    fmt::Debug,
    fs::File,
    io::{BufRead, BufReader, Read},
    str::FromStr,
};

pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;

pub fn content(path: &str) -> String {
    let mut buf = String::new();
    let mut file = File::open(path).expect("no such file");
    file.read_to_string(&mut buf).unwrap();
    buf
}

pub fn lines(path: &str) -> impl Iterator<Item = String> {
    let file = File::open(path).expect("no such file");
    let reader = BufReader::new(file);
    reader.lines().map(|l| l.unwrap())
}

pub fn items_from_str<T>(input: &str) -> impl Iterator<Item = T> + '_
where
    T: FromStr,
    T::Err: Debug,
{
    input.lines().map(|l| T::from_str(l).unwrap())
}

pub fn items<T>(path: &str) -> impl Iterator<Item = T>
where
    T: FromStr,
    T::Err: Debug,
{
    lines(path).map(|l| T::from_str(&l).unwrap())
}
