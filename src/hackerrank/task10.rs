// https://www.hackerrank.com/challenges/sock-merchant/problem
use std::io::{self, BufRead};
use std::collections::HashMap;

pub fn sock_merchant(_n: i32, ar: &[i32]) -> i32 {
    let mut counts = HashMap::new();
    
    for &sock in ar {
        *counts.entry(sock).or_insert(0) += 1;
    }

    let mut pairs = 0;
    for &count in counts.values() {
        pairs += count / 2;
    }

    pairs
}

#[allow(dead_code)]
fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let ar: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let result = sock_merchant(n, &ar);
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sock_merchant_sample() {
        let ar = vec![10, 20, 20, 10, 10, 30, 50, 10, 20];
        assert_eq!(sock_merchant(9, &ar), 3);
    }

    #[test]
    fn test_sock_merchant_no_pairs() {
        let ar = vec![1, 2, 3, 4, 5];
        assert_eq!(sock_merchant(5, &ar), 0);
    }
}