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

    let mut total: i64 = 0;
    let mut i2: usize = 0;

    for i in 0..nums1.len() {
        let mut multiplier: i32 = 0;
        while nums1[i] > nums2[i2] {
            i2 += 1;
        }

        while nums1[i] == nums2[i2] {
            multiplier += 1;
            i2 += 1;
        }

        total += i64::from(nums1[i] * multiplier);
    }

    println!("{}", total)
}
