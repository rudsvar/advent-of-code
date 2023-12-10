fn number_in_alphabet(c: u8) -> u32 {
    c as u32 - 'a' as u32
}

fn find_marker(mut bytes: &[u8]) -> Option<usize> {
    let mut buf: u32 = 0;
    let mut count = 0;
    // Set bytes for each item
    for idx in 0..4 {
        count += 1;
        let num = number_in_alphabet(bytes[idx]);
        println!("Set {}", num);
        buf |= 1 << num;
    }
    for idx in 4..bytes.len() {
        // println!("Buf {buf:b}");
        if buf.count_ones() == 4 {
            return Some(count);
        }
        count += 1;
        // Remove old bit
        let num = number_in_alphabet(bytes[idx - 4]);
        println!("Unset {}", num);
        buf &= (u32::MAX - 1) << num;
        // Set new bit
        let num = number_in_alphabet(bytes[idx]);
        println!("Set {}", num);
        buf |= 1 << num;
    }
    None
}

#[cfg(test)]
mod tests {
    use super::find_marker;

    #[test]
    fn example_1() {
        assert_eq!(Some(5), find_marker(b"bvwbjplbgvbhsrlpgdmjqwftvncz"))
    }

    #[test]
    fn example_2() {
        assert_eq!(Some(6), find_marker(b"nppdvjthqldpwncqszvftbrmjlhg"))
    }

    #[test]
    fn example_3() {
        assert_eq!(Some(10), find_marker(b"nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"))
    }

    #[test]
    fn example_4() {
        assert_eq!(Some(11), find_marker(b"zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"))
    }
}
