// https://www.hackerrank.com/challenges/drawing-book
use std::io::{self, BufRead};
use std::cmp::min;

pub fn page_count(n: i32, p: i32) -> i32 {
    let from_front = p / 2;
    let from_back = (n / 2) - (p / 2);
    
    min(from_front, from_back)
}

#[allow(dead_code)]
fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();
    let p = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let result = page_count(n, p);
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_page_count_sample1() {
        assert_eq!(page_count(6, 2), 1);
    }

    #[test]
    fn test_page_count_sample2() {
        assert_eq!(page_count(5, 4), 0);
    }

    #[test]
    fn test_page_count_exact_back() {
        assert_eq!(page_count(6, 5), 1);
    }
}