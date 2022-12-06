use std::collections::HashSet;

fn main() {
    let input = aoc::read_input_string();
    println!("Part 1: {}", find_marker(4, &input));
    println!("Part 2: {}", find_marker(14, &input));
}

fn find_marker(n: usize, input: &str) -> usize {
    input
        .chars()
        .collect::<Vec<_>>()
        .windows(n)
        .enumerate()
        .find(|(_, window)| HashSet::<_>::from_iter(window.iter().copied()).len() == n)
        .map(|(i, _)| i + n)
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_marker_4() {
        assert_eq!(find_marker(4, "mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 7);
        assert_eq!(find_marker(4, "bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(find_marker(4, "nppdvjthqldpwncqszvftbrmjlhg"), 6);
        assert_eq!(find_marker(4, "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
        assert_eq!(find_marker(4, "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
    }

    #[test]
    fn find_marker_14() {
        assert_eq!(find_marker(14, "mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
        assert_eq!(find_marker(14, "bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
        assert_eq!(find_marker(14, "nppdvjthqldpwncqszvftbrmjlhg"), 23);
        assert_eq!(find_marker(14, "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
        assert_eq!(find_marker(14, "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);
    }
}
