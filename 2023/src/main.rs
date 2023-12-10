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

fn replace_names_with_digit(s: &str) -> String {
    s.replacen("one", "1", 1)
        .replacen("two", "2", 1)
        .replacen("three", "3", 1)
        .replacen("four", "4", 1)
        .replacen("five", "5", 1)
        .replacen("six", "6", 1)
        .replacen("seven", "7", 1)
        .replacen("eight", "8", 1)
        .replacen("nine", "9", 1)
        .chars()
        .rev()
        .collect::<String>()
        .replacen("eno", "1", 1)
        .replacen("owt", "2", 1)
        .replacen("eerht", "3", 1)
        .replacen("ruof", "4", 1)
        .replacen("evif", "5", 1)
        .replacen("xis", "6", 1)
        .replacen("neves", "7", 1)
        .replacen("thgie", "8", 1)
        .replacen("enin", "9", 1)
        .chars()
        .rev()
        .collect::<String>()
}

fn main() {
    let arg = std::env::args().nth(1).expect("Please supply an argument");
    let content = std::fs::read_to_string(&arg).expect("Failed to read file");
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
            "1 2 3 4 5 6 7 8 9"
        );
    }

    #[test]
    fn test_replace_names_with_digit_2() {
        assert_eq!(replace_names_with_digit("two1nine"), "219");
        assert_eq!(replace_names_with_digit("eightwothree"), "823");
        assert_eq!(replace_names_with_digit("abcone2threexyz"), "13xyz");
        assert_eq!(replace_names_with_digit("xtwone3four"), "14");
        assert_eq!(replace_names_with_digit("4nineeightseven2"), "49872");
        assert_eq!(replace_names_with_digit("zoneight234"), "248");
        assert_eq!(replace_names_with_digit("7pqrstsixteen"), "716");
    }

    #[test]
    fn test_first_and_last_digit_2() {
        assert_eq!(first_and_last_digit("two1nine"), Some(29));
        assert_eq!(first_and_last_digit("eightwothree"), Some(83));
        assert_eq!(first_and_last_digit("abcone2threexyz"), Some(13));
        assert_eq!(first_and_last_digit("xtwone3four"), Some(14));
        assert_eq!(first_and_last_digit("4nineeightseven2"), Some(42));
        assert_eq!(first_and_last_digit("zoneight234"), Some(24));
        assert_eq!(first_and_last_digit("7pqrstsixteen"), Some(76));
    }
}
