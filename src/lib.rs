use std::fs;

const PRUNE_BASIS: usize = 16777216;

fn step_mul64(secret_number: usize) -> usize {
    (secret_number ^ (secret_number * 64)) % PRUNE_BASIS
}

fn step_div32(secret_number: usize) -> usize {
    (secret_number ^ (secret_number / 32)) % PRUNE_BASIS
}

fn step_mul2048(secret_number: usize) -> usize {
    (secret_number ^ (secret_number * 2048)) % PRUNE_BASIS
}

fn calculate_next_secret(secret_number: usize) -> usize {
    step_mul2048(step_div32(step_mul64(secret_number)))
}

fn calculate_next_secret_n_times(mut secret_number: usize, n: usize) -> usize {
    for _ in 0..n {
        secret_number = calculate_next_secret(secret_number);
    }
    secret_number
}

fn parse_numbers(content: &str) -> impl Iterator<Item = usize> + '_ {
    content.lines()
        .filter_map(|line| line.trim().parse::<usize>().ok())
}

pub fn run() -> usize {
    const SECRET_NUMBER_ITERATION: usize = 2000;

    let filename = "inputs/day22.txt";
    let content = fs::read_to_string(filename).expect("Can't read file '{filename}'");
    parse_numbers(&content)
        .map(|secret_number| calculate_next_secret_n_times(secret_number, SECRET_NUMBER_ITERATION))
        .sum()
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_step_mul64() {
        assert_eq!(step_mul64(123), 7867);
    }

    #[test]
    fn test_step_mul64_prunes() {
        assert_eq!(step_mul64(PRUNE_BASIS + 123), 7867);
    }

    #[test]
    fn test_step_div32() {
        assert_eq!(step_div32(7867), 7758);
    }

    #[test]
    fn test_step_div32_prunes() {
        assert_eq!(step_div32(PRUNE_BASIS*32 + 7867), 7758);
    }

    #[test]
    fn test_step_mul2048() {
        assert_eq!(step_mul2048(7758), 15887950);
    }

    #[test]
    fn test_step_mul2048_prunes() {
        assert_eq!(step_mul2048(PRUNE_BASIS + 7758), 15887950);
    }

    #[test]
    fn test_calculate_next_secret() {
        assert_eq!(calculate_next_secret(123), 15887950);
    }

    #[test]
    fn test_calculate_next_secret_n_times() {
        assert_eq!(calculate_next_secret_n_times(1, 2000), 8685429);
        assert_eq!(calculate_next_secret_n_times(10, 2000), 4700978);
        assert_eq!(calculate_next_secret_n_times(100, 2000), 15273692);
        assert_eq!(calculate_next_secret_n_times(2024, 2000), 8667524);
    }
}
