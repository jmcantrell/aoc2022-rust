use std::collections::{HashSet, VecDeque};
use std::fs;

fn find_unique_window(s: &str, size: usize) -> Option<usize> {
    let mut window: VecDeque<char> = Default::default();
    let mut count: usize = 0;

    for c in s.chars() {
        window.push_back(c);
        count += 1;

        if window.len() > size {
            window.pop_front().unwrap();
        }

        if window.len() == size {
            let set: HashSet<char> = HashSet::from_iter(window.iter().cloned());
            if set.len() == size {
                return Some(count);
            }
        }
    }

    None
}

fn find_datastream_message_start(buffer: &str) -> Option<usize> {
    find_unique_window(buffer, 14)
}

fn main() -> anyhow::Result<()> {
    let input = fs::read_to_string("input.txt")?;

    let count = find_datastream_message_start(&input).unwrap();

    dbg!(count);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        macro_rules! test {
            ($input:expr, $expected:expr) => {
                assert_eq!(find_datastream_message_start($input), $expected);
            };
        }

        test!("mjqjpqmgbljsphdztnvjfqwrcgsmlb", Some(19));
        test!("bvwbjplbgvbhsrlpgdmjqwftvncz", Some(23));
        test!("nppdvjthqldpwncqszvftbrmjlhg", Some(23));
        test!("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", Some(29));
        test!("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", Some(26));
    }
}
