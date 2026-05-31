// https://www.hackerrank.com/challenges/between-two-sets/problem
use std::io::{self, BufRead};

fn gcd(a: i32, b: i32) -> i32 {
    let mut x = a;
    let mut y = b;
    while y != 0 {
        let temp = y;
        y = x % y;
        x = temp;
    }
    x
}

fn lcm(a: i32, b: i32) -> i32 {
    if a == 0 || b == 0 {
        return 0;
    }
    (a * b) / gcd(a, b)
}

pub fn get_total_x(a: &[i32], b: &[i32]) -> i32 {
    let mut lcm_a = a[0];
    for &val in a.iter().skip(1) {
        lcm_a = lcm(lcm_a, val);
    }

    let mut gcd_b = b[0];
    for &val in b.iter().skip(1) {
        gcd_b = gcd(gcd_b, val);
    }

    let mut count = 0;
    let mut multiple = lcm_a;
    while multiple <= gcd_b {
        if gcd_b % multiple == 0 {
            count += 1;
        }
        multiple += lcm_a;
    }

    count
}

#[allow(dead_code)]
fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let _first_line = stdin_iterator.next().unwrap().unwrap();

    let a: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let b: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let total = get_total_x(&a, &b);
    println!("{}", total);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_total_x() {
        let a = vec![2, 4];
        let b = vec![16, 32, 96];
        assert_eq!(get_total_x(&a, &b), 3);
    }

    #[test]
    fn test_get_total_x_single_elements() {
        let a = vec![2, 6];
        let b = vec![24, 36];
        assert_eq!(get_total_x(&a, &b), 2);
    }
}