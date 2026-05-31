// https://www.hackerrank.com/challenges/breaking-best-and-worst-records/problem
use std::io::{self, BufRead};

pub fn breaking_records(scores: &[i32]) -> Vec<i32> {
    if scores.is_empty() {
        return vec![0, 0];
    }

    let mut highest = scores[0];
    let mut lowest = scores[0];
    let mut high_count = 0;
    let mut low_count = 0;

    for &score in scores.iter().skip(1) {
        if score > highest {
            highest = score;
            high_count += 1;
        } else if score < lowest {
            lowest = score;
            low_count += 1;
        }
    }

    vec![high_count, low_count]
}

#[allow(dead_code)]
fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let _n = stdin_iterator.next().unwrap().unwrap();

    let scores: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let result = breaking_records(&scores);
    println!("{} {}", result[0], result[1]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_breaking_records_sample1() {
        let scores = vec![10, 5, 20, 20, 4, 5, 2, 25, 1];
        assert_eq!(breaking_records(&scores), vec![2, 4]);
    }

    #[test]
    fn test_breaking_records_sample2() {
        let scores = vec![3, 4, 21, 36, 10, 28, 35, 5, 24, 42];
        assert_eq!(breaking_records(&scores), vec![4, 0]);
    }
}