use std::collections::HashMap;

fn first_digit(s: &str) -> Option<char> {
    s.chars().find(|c| c.is_ascii_digit())
}

fn last_digit(s: &str) -> Option<char> {
    s.chars().rev().find(|c| c.is_ascii_digit())
}

fn first_and_last_digit(s: &str) -> Option<u32> {
    let s = replace_names_with_digit(s);
    let first = first_digit(&s)?;
    let last = last_digit(&s)?;
    format!("{}{}", first, last).parse().ok()
}

fn find_earliest_element<'a>(s: &str, words: &[&'a str]) -> Option<(usize, &'a str)> {
    words
        .iter()
        .map(|word| (s.find(word), word))
        .filter_map(|(pos, word)| pos.map(|pos| (pos, *word)))
        .min_by_key(|(pos, _)| *pos)
}

fn replace_names_with_digit(s: &str) -> String {
    let words = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let word_to_idx: HashMap<&str, usize> = words
        .iter()
        .enumerate()
        .map(|(i, word)| (*word, i + 1))
        .collect();

    let mut s = s.to_string();
    if let Some((pos, word)) = find_earliest_element(&s, &words) {
        let idx = word_to_idx[word];
        s = s.replace(&s[pos..pos + word.len()], &idx.to_string())
    }

    s = s.chars().rev().collect::<String>();
    let words_rev = words.map(|word| word.chars().rev().collect::<String>());
    let words_rev: Vec<&str> = words_rev.iter().map(|w| w.as_ref()).collect();

    if let Some((pos, word)) = find_earliest_element(&s, &words_rev) {
        let word = word.chars().rev().collect::<String>();
        let idx = word_to_idx[word.as_str()];
        s = s.replace(&s[pos..pos + word.len()], &idx.to_string())
    }
    s = s.chars().rev().collect::<String>();

    s
}

fn main() {
    let arg = std::env::args().nth(1).expect("Please supply an argument");
    let content = std::fs::read_to_string(arg).expect("Failed to read file");
    let mut sum: u32 = 0;
    for line in content.lines() {
        let first_and_last_digit = first_and_last_digit(line);
        sum += first_and_last_digit.expect("Failed to parse line");
        println!("{}: {}", line, first_and_last_digit.unwrap_or(0));
    }
    println!("{}", sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_digit() {
        assert_eq!(first_digit("abc123"), Some('1'));
        assert_eq!(first_digit("abc"), None);
    }

    #[test]
    fn test_last_digit() {
        assert_eq!(last_digit("abc123"), Some('3'));
        assert_eq!(last_digit("abc"), None);
    }

    #[test]
    fn test_first_and_last_digit() {
        assert_eq!(first_and_last_digit("abc123"), Some(13));
        assert_eq!(first_and_last_digit("abc"), None);
    }

    #[test]
    fn test_replace_names_with_digit() {
        assert_eq!(
            replace_names_with_digit("one two three four five six seven eight nine"),
            "1 two three four five six seven eight 9"
        );
    }

    #[test]
    fn test_replace_names_with_digit_2() {
        assert_eq!(replace_names_with_digit("two1nine"), "219");
        assert_eq!(replace_names_with_digit("eightwothree"), "8wo3");
        assert_eq!(replace_names_with_digit("abcone2threexyz"), "abc123xyz");
        assert_eq!(replace_names_with_digit("xtwone3four"), "x2ne34");
        assert_eq!(replace_names_with_digit("4nineeightseven2"), "49eight72");
        assert_eq!(replace_names_with_digit("zoneight234"), "z1ight234");
        assert_eq!(replace_names_with_digit("7pqrstsixteen"), "7pqrst6teen");
    }

    #[test]
    fn test_first_and_last_digit_2() {
        assert_eq!(first_and_last_digit("two1nine"), Some(29));
        assert_eq!(first_and_last_digit("eightwothree"), Some(83));
        assert_eq!(first_and_last_digit("abcone2threexyz"), Some(13));
        assert_eq!(first_and_last_digit("xtwone3four"), Some(24));
        assert_eq!(first_and_last_digit("4nineeightseven2"), Some(42));
        assert_eq!(first_and_last_digit("zoneight234"), Some(14));
        assert_eq!(first_and_last_digit("7pqrstsixteen"), Some(76));
    }
}
