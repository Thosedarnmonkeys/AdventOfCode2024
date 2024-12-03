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

        let mut prev = iter.next().unwrap().parse::<i32>().unwrap();
        let mut first_value = true;
        let mut ascending = true;

        loop {
            let next_str = match iter.next() {
                Some(next_str) => next_str,
                None => {
                    safe_count += 1;
                    break;
                }
            };

            let next = next_str.parse::<i32>().unwrap();

            if first_value {
                match next.cmp(&prev) {
                    Equal => break,
                    Less => ascending = false,
                    Greater => ascending = true,
                }

                let diff = diff(&prev, &next);
                if diff < 1 || diff > 3 {
                    break;
                }

                first_value = false;
            } else {
                match next.cmp(&prev) {
                    Equal => break,
                    Less => {
                        if ascending {
                            break;
                        }
                    }
                    Greater => {
                        if !ascending {
                            break;
                        }
                    }
                }

                let diff = diff(&prev, &next);
                if diff < 1 || diff > 3 {
                    break;
                }
            }

            prev = next;
        }
    }

    println!("{}", safe_count)
}

fn diff(a: &i32, b: &i32) -> i32 {
    return match a.cmp(b) {
        Less => b - a,
        Greater | Equal => a - b,
    };
}
