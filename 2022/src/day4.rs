use std::str::FromStr;

pub struct Pair {
    left: (i32, i32),
    right: (i32, i32),
}

impl Pair {
    pub fn has_complete_overlap(&self) -> bool {
        self.left.0 <= self.right.0 && self.right.1 <= self.left.1
            || self.right.0 <= self.left.0 && self.left.1 <= self.right.1
    }
    pub fn has_partial_overlap(&self) -> bool {
        let left_start = self.left.0;
        let left_end = self.left.1;
        let right_start = self.right.0;
        let right_end = self.right.1;
        right_start <= left_start && left_start <= right_end
            || right_start <= left_end && left_end <= right_end
            || left_start <= right_start && right_start <= left_end
            || left_start <= right_end && right_end <= left_end
    }
}

impl FromStr for Pair {
    type Err = ();
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (left, right) = input.split_once(',').unwrap();
        let (l1, l2) = left.split_once('-').unwrap();
        let (r1, r2) = right.split_once('-').unwrap();
        Ok(Pair {
            left: (l1.parse().unwrap(), l2.parse().unwrap()),
            right: (r1.parse().unwrap(), r2.parse().unwrap()),
        })
    }
}

pub fn completely_overlapping_pairs(pairs: impl Iterator<Item = Pair>) -> i32 {
    pairs
        .map(|pair| i32::from(pair.has_complete_overlap()))
        .sum()
}

pub fn partially_overlapping_pairs(pairs: impl Iterator<Item = Pair>) -> i32 {
    pairs
        .map(|pair| i32::from(pair.has_partial_overlap()))
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::items_from_str;

    use super::*;

    #[test]
    fn example_works() {
        let items = items_from_str(
            r#"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"#,
        );
        assert_eq!(completely_overlapping_pairs(items), 2);
    }

    #[test]
    fn part1() {
        let items = crate::items::<Pair>("./data/day4.txt");
        assert_eq!(completely_overlapping_pairs(items), 536);
    }

    #[test]
    fn part2() {
        let items = crate::items::<Pair>("./data/day4.txt");
        assert_eq!(partially_overlapping_pairs(items), 845);
    }
}
