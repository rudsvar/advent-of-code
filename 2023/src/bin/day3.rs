fn height(s: &str) -> usize {
    s.lines().count()
}

fn width(s: &str) -> usize {
    s.lines().next().unwrap().len() + 1
}

fn char_at(s: &str, x: usize, y: usize) -> Option<char> {
    s.chars().nth(y * width(s) + x)
}

fn surrounding_chars(s: &str, x: usize, y: usize) -> Vec<char> {
    let mut cs = Vec::new();
    for dy in -1i32..=1 {
        for dx in -1i32..=1 {
            if dx == 0 && dy == 0 {
                continue;
            }
            let x = x as i32 + dx;
            let y = y as i32 + dy;
            if x < 0 || (width(s) as i32) <= x {
                continue;
            }
            if y < 0 || (height(s) as i32) <= y {
                continue;
            }
            let x = x as usize;
            let y = y as usize;
            if let Some(c) = char_at(s, x, y) {
                cs.push(c)
            }
        }
    }
    cs
}

fn main() {
    let path = std::env::args().nth(1).unwrap();
    let content = std::fs::read_to_string(path).unwrap();
    println!(
        "Height = {}, widght = {}",
        height(&content),
        width(&content)
    );
    let mut sum: usize = 0;
    let mut i = 0;
    while i < content.len() {
        let substr = &content[i..];
        let digits: String = substr.chars().take_while(|c| c.is_ascii_digit()).collect();
        if digits.is_empty() {
            i += 1;
            continue;
        }
        let y = i / width(&content);
        let x = i % width(&content);

        println!("Checking {}, pos {:?}", digits, (x, y));
        let has_adjacent_symbol = (0..digits.len()).any(|offset| {
            let surrounding_chars = surrounding_chars(&content, x + offset, y);
            println!("Surrounding chars {:?}", surrounding_chars);
            surrounding_chars
                .iter()
                .any(|&c| !c.is_ascii_digit() && c != '.' && !c.is_whitespace())
        });
        if has_adjacent_symbol {
            println!("{} has adjacent symbol", digits);
            sum += digits.parse::<usize>().unwrap();
        } else {
            println!("{} does not have adjacent symbol", digits);
        }
        i += digits.len();
    }
    println!("{}", sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_height() {
        let s = r"abc
def
ghi";
        assert_eq!(3, height(s));
    }

    #[test]
    fn test_width() {
        let s = r"abc123
def456
ghi789";
        assert_eq!(7, width(s));
    }

    #[test]
    fn test_char_at() {
        let s = r"abc123
def456
ghi789";
        assert_eq!(Some('4'), char_at(s, 3, 1));
    }

    #[test]
    fn test_surrounding_chars() {
        let s = r"abc123
def456
ghi789";
        assert_eq!(
            vec!['c', '1', '2', 'f', '5', 'i', '7', '8'],
            surrounding_chars(s, 3, 1)
        );
    }

    #[test]
    fn test_surrounding_chars_2() {
        let s = r"abc123
def456
ghi789";
        assert_eq!(vec!['2', '\n', '5', '6', '\n'], surrounding_chars(s, 5, 0));
    }

    #[test]
    fn test_surrounding_chars_3() {
        let s = r"abc123
def456
ghi789
";
        assert_eq!(vec!['5', '6', '\n', '8', '\n'], surrounding_chars(s, 5, 2));
    }
}