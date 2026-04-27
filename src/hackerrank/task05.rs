// https://www.hackerrank.com/challenges/apple-and-orange/problem
use std::io::{self, BufRead};
pub fn count_apples_and_oranges(s: i32, t: i32, a: i32, b: i32, apples: &[i32], oranges: &[i32]) -> (i32, i32) {
    let apple_count = apples.iter()
        .map(|&d| a + d)
        .filter(|&pos| pos >= s && pos <= t)
        .count() as i32;

    let orange_count = oranges.iter()
        .map(|&d| b + d)
        .filter(|&pos| pos >= s && pos <= t)
        .count() as i32;

    (apple_count, orange_count)
}

#[allow(dead_code)]
fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let first_line = stdin_iterator.next().unwrap().unwrap();
    let st: Vec<i32> = first_line.split_whitespace().map(|s| s.parse().unwrap()).collect();
    let (s, t) = (st[0], st[1]);

    let second_line = stdin_iterator.next().unwrap().unwrap();
    let ab: Vec<i32> = second_line.split_whitespace().map(|s| s.parse().unwrap()).collect();
    let (a, b) = (ab[0], ab[1]);

    let _third_line = stdin_iterator.next().unwrap().unwrap();

    let apples: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .split_whitespace().map(|s| s.parse().unwrap()).collect();

    let oranges: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .split_whitespace().map(|s| s.parse().unwrap()).collect();

    let (res_a, res_o) = count_apples_and_oranges(s, t, a, b, &apples, &oranges);
    println!("{}", res_a);
    println!("{}", res_o);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_fruits() {
        let s = 7;
        let t = 11;
        let a = 5;
        let b = 15;
        let apples = vec![-2, 2, 1];
        let oranges = vec![5, -6];

        let (apples_on_house, oranges_on_house) = count_apples_and_oranges(s, t, a, b, &apples, &oranges);
        
        assert_eq!(apples_on_house, 1);
        assert_eq!(oranges_on_house, 1);
    }
}