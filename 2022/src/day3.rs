use std::collections::HashSet;

pub struct Rucksack {
    left: Compartment,
    right: Compartment,
}

impl Rucksack {
    pub fn new(line: String) -> Rucksack {
        let (left, right) = line.split_at(line.len() / 2);
        let left = Compartment::new(left);
        let right = Compartment::new(right);
        Rucksack { left, right }
    }
    pub fn overlap(&self) -> impl Iterator<Item = &char> + '_ {
        self.left.items.intersection(&self.right.items)
    }
    pub fn overlap_priority(&self) -> i32 {
        self.overlap().map(|c| priority(*c)).sum()
    }
}

fn priority(c: char) -> i32 {
    if ('a'..='z').contains(&c) {
        c as i32 - 'a' as i32 + 1
    } else if ('A'..='Z').contains(&c) {
        c as i32 - 'A' as i32 + 27
    } else {
        panic!("invalid letter")
    }
}

#[derive(Debug)]
struct Compartment {
    items: HashSet<char>,
}
impl Compartment {
    fn new(line: &str) -> Compartment {
        let items = line.chars().collect();
        Compartment { items }
    }
}

pub fn parse_rucksacks(input: impl Iterator<Item = String>) -> impl Iterator<Item = Rucksack> {
    input.map(Rucksack::new)
}

pub fn solve(input: impl Iterator<Item = String>) -> i32 {
    let rucksacks = parse_rucksacks(input);
    rucksacks.map(|r| r.overlap_priority()).sum()
}

pub fn badges(input: impl Iterator<Item = String>) -> i32 {
    let mut sum: i32 = 0;
    let input: Vec<_> = input.collect();
    for i in (0..input.len()).step_by(3) {
        let a: HashSet<char> = input[i].chars().collect();
        let b: HashSet<char> = input[i + 1].chars().collect();
        let c: HashSet<char> = input[i + 2].chars().collect();
        let ab: HashSet<char> = a.intersection(&b).copied().collect();
        let abc = ab.intersection(&c);
        sum += abc.map(|c| priority(*c)).sum::<i32>();
    }
    sum
}

#[cfg(test)]
mod tests {
    use std::{
        fs::File,
        io::{BufRead, BufReader},
    };

    use super::*;

    #[test]
    fn new_compartment_works() {
        let comp = Compartment::new("aA");
        dbg!(&comp);
    }

    #[test]
    fn overlap_1() {
        let r = Rucksack::new("vJrwpWtwJgWrhcsFMMfFFhFp".to_string());
        assert_eq!(r.overlap().collect::<Vec<_>>(), vec![&'p']);
    }

    #[test]
    fn overlap_2() {
        let r = Rucksack::new("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL".to_string());
        assert_eq!(r.overlap().collect::<Vec<_>>(), vec![&'L']);
    }

    #[test]
    fn test_priority() {
        assert_eq!(priority('a'), 1);
        assert_eq!(priority('b'), 2);
        assert_eq!(priority('A'), 27);
    }

    #[test]
    fn example_gives_157() {
        let lines = vec![
            "vJrwpWtwJgWrhcsFMMfFFhFp",
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
            "PmmdzqPrVvPwwTWBwg",
            "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
            "ttgJtRGJQctTZtZT",
            "CrZsJsPPZsGzwwsLwLmpwMDw",
        ];
        assert_eq!(solve(lines.iter().map(|s| s.to_string())), 157);
    }

    #[test]
    fn part1() {
        let file = BufReader::new(File::open("data/day3.txt").unwrap());
        assert_eq!(solve(file.lines().map(|l| l.unwrap())), 8123);
    }

    #[test]
    fn part2() {
        let file = BufReader::new(File::open("data/day3.txt").unwrap());
        assert_eq!(badges(file.lines().map(|l| l.unwrap())), 2620);
    }
}
