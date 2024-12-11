use std::fs;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Stone {
    value: usize
}

fn count_digits(x: usize) -> usize {
    let mut count = 0;
    let mut x = x;
    while x > 0 {
        x /= 10;
        count += 1;
    }
    count
}

fn split_digits_in_half(x: usize) -> (usize, usize) {
    let count = count_digits(x);
    assert!(count % 2 == 0);
    let divisor = 10usize.pow(u32::try_from(count / 2).expect("value to fit"));
    (x / divisor, x % divisor)
}

impl Stone {
    pub fn new(value: usize) -> Stone {
        Stone { value }
    }

    pub fn blink(&self) -> Vec<Stone> {
        if self.value == 0 {
            vec![Stone::new(1)]
        } else if count_digits(self.value) % 2 == 0 {
            let (a, b) = split_digits_in_half(self.value);
            vec![Stone::new(a), Stone::new(b)]
        } else {
            vec![Stone::new(self.value*2024)]
        }
    }
}

fn blink_many_times(stones: &[Stone], times_to_blink: usize) -> Vec<Stone> {
    let mut stones = stones.to_vec();
    for _ in 0..times_to_blink {
        let mut new_stones = Vec::with_capacity(stones.len() * 2);
        for stone in stones {
            new_stones.extend(stone.blink());
        }
        stones = new_stones;
    }
    stones
}

pub fn run() -> usize {
    let filename = "inputs/day11.txt";
    let content = fs::read_to_string(filename).expect("Can't read file '{filename}'");
    let stones: Vec<Stone> = content.split(' ')
        .map(|x| x.parse::<usize>().expect("positive integer"))
        .map(Stone::new)
        .collect();
    let mut total_count = 0usize;
    for stone in stones {
        total_count += blink_many_times(&[stone], 25).len();
        println!("stone={:?}, total_count={total_count}", stone);
    }
    total_count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_digits() {
        assert_eq!(count_digits(1), 1);
        assert_eq!(count_digits(11), 2);
        assert_eq!(count_digits(111), 3);
    }

    #[test]
    fn test_split_digits_in_half() {
        assert_eq!(split_digits_in_half(12), (1,2));
        assert_eq!(split_digits_in_half(1234), (12, 34));
    }

    #[test]
    fn test_blink() {
        assert_eq!(Stone::new(0).blink(), vec![Stone::new(1)]);
        assert_eq!(Stone::new(12).blink(), vec![Stone::new(1), Stone::new(2)]);
        assert_eq!(Stone::new(1).blink(), vec![Stone::new(2024)]);
    }
}
