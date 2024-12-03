use std::cmp::Ordering::{Equal, Greater, Less};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut nums1: Vec<i32> = Vec::new();
    let mut nums2: Vec<i32> = Vec::new();

    for line in reader.lines() {
        let line_str = line.unwrap();
        let mut iter = line_str.split_whitespace();
        let num1 = iter.next().unwrap().parse::<i32>().unwrap();
        let num2 = iter.next().unwrap().parse::<i32>().unwrap();

        nums1.push(num1);
        nums2.push(num2);
    }

    nums1.sort();
    nums2.sort();

    let mut total_distance: i64 = 0;

    for i in 0..nums1.len() {
        let a = nums1[i];
        let b = nums2[i];
        let value = match a.cmp(&b) {
            Less => b - a,
            Greater | Equal => a - b,
        };

        total_distance += i64::from(value);
    }

    println!("{}", total_distance)
}
