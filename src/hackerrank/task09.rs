// https://www.hackerrank.com/challenges/migratory-birds/problem
use std::io::{self, BufRead};

pub fn migratory_birds(arr: &[i32]) -> i32 {
    let mut counts = vec![0; 6];
    
    for &bird in arr {
        if bird >= 1 && bird <= 5 {
            counts[bird as usize] += 1;
        }
    }

    let mut max_count = 0;
    let mut result_bird = 1;

    for bird in 1..=5 {
        if counts[bird] > max_count {
            max_count = counts[bird];
            result_bird = bird as i32;
        }
    }

    result_bird
}

#[allow(dead_code)]
fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let _arr_count = stdin_iterator.next().unwrap().unwrap();

    let arr: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let result = migratory_birds(&arr);
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_migratory_birds_sample1() {
        let arr = vec![1, 4, 4, 4, 5, 3];
        assert_eq!(migratory_birds(&arr), 4);
    }

    #[test]
    fn test_migratory_birds_tie() {
        let arr = vec![1, 2, 2, 3, 3, 5];
        assert_eq!(migratory_birds(&arr), 2);
    }
}