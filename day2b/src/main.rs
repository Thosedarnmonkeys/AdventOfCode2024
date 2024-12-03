use std::cmp::Ordering::{Equal, Greater, Less};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut safe_count = 0;

    for line in reader.lines() {
        let line_str = line.unwrap();
        let mut iter = line_str.split_whitespace();
        let mut nums = Vec::new();
        loop {
            let next_str = match iter.next() {
                Some(next_str) => next_str,
                None => {
                    break;
                }
            };

            let next = next_str.parse::<i32>().unwrap();
            nums.push(next);
        }

        let (pass, _) = test(&nums);
        if pass {
            safe_count += 1;
            continue;
        }

        for i in 0..nums.len() {
            let mut clone = nums.clone();
            clone.remove(i);
            let (modded_pass, _) = test(&clone);
            if modded_pass {
                safe_count += 1;
                break;
            }
        }
    }

    println!("{}", safe_count)
}

fn test(nums: &Vec<i32>) -> (bool, usize) {
    let mut prev = nums[0];
    let mut ascending = true;

    for i in 1..nums.len() {
        let next = nums[i];
        if i == 1 {
            match next.cmp(&prev) {
                Equal => return (false, i),
                Less => ascending = false,
                Greater => ascending = true,
            }

            let diff = diff(&prev, &next);
            if diff < 1 || diff > 3 {
                return (false, i);
            }
        } else {
            match next.cmp(&prev) {
                Equal => return (false, i),
                Less => {
                    if ascending {
                        return (false, i);
                    }
                }
                Greater => {
                    if !ascending {
                        return (false, i);
                    }
                }
            }

            let diff = diff(&prev, &next);
            if diff < 1 || diff > 3 {
                return (false, i);
            }
        }

        prev = next;
    }

    return (true, 0);
}

fn diff(a: &i32, b: &i32) -> i32 {
    return match a.cmp(b) {
        Less => b - a,
        Greater | Equal => a - b,
    };
}
