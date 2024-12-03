use std::{
    fs::File,
    io::{BufReader, Read},
};

enum Progress {
    M,
    U,
    L,
    Open,
    A,
    Comma,
    B,
    Close,
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let mut reader = BufReader::new(file);
    let mut input = String::new();
    let _ = reader.read_to_string(&mut input);

    let mut a_str = String::new();
    let mut b_str = String::new();
    let mut total = 0;
    let mut i = 0;
    let mut iter: std::iter::Peekable<std::str::Chars<'_>> = input.chars().peekable();
    let max = input.chars().count();
    let mut step = Progress::M;

    while i < max {
        let char = iter.next().unwrap();

        match step {
            Progress::M => check_letter(&char, 'm', Progress::U, &mut step),
            Progress::U => check_letter(&char, 'u', Progress::L, &mut step),
            Progress::L => check_letter(&char, 'l', Progress::Open, &mut step),
            Progress::Open => check_letter(&char, '(', Progress::A, &mut step),
            Progress::A => check_num(&char, &mut a_str, &mut step, &mut iter, &mut i),
            Progress::Comma => check_letter(&char, ',', Progress::B, &mut step),
            Progress::B => check_num(&char, &mut b_str, &mut step, &mut iter, &mut i),
            Progress::Close => {
                if char == ')' {
                    let a = i32::from_str_radix(&a_str.as_str(), 10).unwrap();
                    let b = i32::from_str_radix(&b_str.as_str(), 10).unwrap();
                    total += a * b;
                }
                step = Progress::M;
            }
        }

        i += 1;
    }

    println!("{}", total);
}

fn check_letter(to_check: &char, example: char, if_correct_value: Progress, to_set: &mut Progress) {
    if *to_check == example {
        *to_set = if_correct_value;
    } else {
        *to_set = Progress::M;
    }
}

fn check_num(
    first: &char,
    num_str: &mut String,
    current_step: &mut Progress,
    iter: &mut std::iter::Peekable<std::str::Chars<'_>>,
    i: &mut usize,
) {
    if !first.is_digit(10) {
        *current_step = Progress::M;
        return;
    }

    let &second = iter.peek().unwrap();
    if !second.is_digit(10) {
        if second == ',' && matches!(*current_step, Progress::A) {
            *num_str = String::from(*first);
            *current_step = Progress::Comma;
        } else if second == ')' && matches!(*current_step, Progress::B) {
            *num_str = String::from(*first);
            *current_step = Progress::Close;
        } else {
            *current_step = Progress::M;
        }

        return;
    }
    iter.next();
    *i += 1;

    let &third = iter.peek().unwrap();
    if !third.is_digit(10) {
        if third == ',' && matches!(*current_step, Progress::A) {
            let vec = Vec::from([*first, second]);
            *num_str = vec.iter().collect();
            *current_step = Progress::Comma;
        } else if third == ')' && matches!(*current_step, Progress::B) {
            let vec = Vec::from([*first, second]);
            *num_str = vec.iter().collect();
            *current_step = Progress::Close;
        } else {
            *current_step = Progress::M;
        }

        return;
    }
    iter.next();
    *i += 1;

    let vec = Vec::from([*first, second, third]);
    *num_str = vec.iter().collect();

    match current_step {
        Progress::A => *current_step = Progress::Comma,
        Progress::B => *current_step = Progress::Close,
        _ => (),
    }
}
