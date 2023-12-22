fn name_to_digit(name: &str) -> Option<u32> {
    match name {
        "0" => Some(0),
        "1" | "one" => Some(1),
        "2" | "two" => Some(2),
        "3" | "three" => Some(3),
        "4" | "four" => Some(4),
        "5" | "five" => Some(5),
        "6" | "six" => Some(6),
        "7" | "seven" => Some(7),
        "8" | "eight" => Some(8),
        "9" | "nine" => Some(9),
        _ => None,
    }
}

fn first_and_last_digit(s: &str) -> Option<u32> {
    let digits = [
        "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five",
        "six", "seven", "eight", "nine",
    ];
    let first = name_to_digit(find_first_element(s, &digits)?)?;
    let last = name_to_digit(find_last_element(s, &digits)?)?;
    format!("{}{}", first, last).parse().ok()
}

fn find_first_element<'a>(s: &str, words: &[&'a str]) -> Option<&'a str> {
    words
        .iter()
        .map(|word| (s.find(word), word))
        .filter_map(|(pos, word)| pos.map(|pos| (pos, *word)))
        .min_by_key(|(pos, _)| *pos)
        .map(|(_, word)| word)
}

fn find_last_element<'a>(s: &str, words: &[&'a str]) -> Option<&'a str> {
    words
        .iter()
        .map(|word| (s.rfind(word), word))
        .filter_map(|(pos, word)| pos.map(|pos| (pos, *word)))
        .max_by_key(|(pos, _)| *pos)
        .map(|(_, word)| word)
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
    fn test_first_and_last_digit() {
        assert_eq!(first_and_last_digit("abc123"), Some(13));
        assert_eq!(first_and_last_digit("abc"), None);
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

    #[test]
    fn test_find_first_of() {
        assert_eq!(find_first_element("abc123", &["abc"]), Some("abc"));
        assert_eq!(find_first_element("abc123", &["123", "b"]), Some("b"));
        assert_eq!(find_first_element("abc123", &["123", "abc"]), Some("abc"));
    }

    #[test]
    fn test_find_last_of() {
        assert_eq!(find_last_element("abc123", &["abc"]), Some("abc"));
        assert_eq!(find_last_element("abc123", &["123", "b"]), Some("123"));
        assert_eq!(find_last_element("abc123", &["123", "abc"]), Some("123"));
    }
}
