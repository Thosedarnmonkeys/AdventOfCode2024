use std::{
    fs::File,
    io::{BufReader, Read},
};

enum P {
    MorD,
    U,
    L,
    MulOpen,
    A,
    Comma,
    B,
    MulClose,
    O,
    DoOpen,
    DoClose,
    N,
    Quote,
    T,
    DontOpen,
    DontClose,
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
    let mut step = P::MorD;

    let mut disable = false;

    while i < max {
        let char = iter.next().unwrap();

        match step {
            P::MorD => {
                if char == 'm' {
                    step = P::U;
                }
                if char == 'd' {
                    step = P::O;
                }
            }
            P::U => check_letter(&char, 'u', P::L, &mut step),
            P::L => check_letter(&char, 'l', P::MulOpen, &mut step),
            P::MulOpen => check_letter(&char, '(', P::A, &mut step),
            P::A => check_num(&char, &mut a_str, &mut step, &mut iter, &mut i),
            P::Comma => check_letter(&char, ',', P::B, &mut step),
            P::B => check_num(&char, &mut b_str, &mut step, &mut iter, &mut i),
            P::MulClose => {
                if char == ')' && !disable {
                    let a = i32::from_str_radix(&a_str.as_str(), 10).unwrap();
                    let b = i32::from_str_radix(&b_str.as_str(), 10).unwrap();
                    total += a * b;
                }
                step = P::MorD;
            }

            P::O => {
                if char != 'o' {
                    step = P::MorD;
                } else {
                    let &next = iter.peek().unwrap();
                    if next == '(' {
                        step = P::DoOpen;
                    } else if next == 'n' {
                        step = P::N;
                    } else {
                        step = P::MorD;
                    }
                }
            }
            P::DoOpen => check_letter(&char, '(', P::DoClose, &mut step),
            P::DoClose => {
                if char == ')' {
                    disable = false;
                }
                step = P::MorD;
            }
            P::N => check_letter(&char, 'n', P::Quote, &mut step),
            P::Quote => check_letter(&char, '\'', P::T, &mut step),
            P::T => check_letter(&char, 't', P::DontOpen, &mut step),
            P::DontOpen => check_letter(&char, '(', P::DontClose, &mut step),
            P::DontClose => {
                if char == ')' {
                    disable = true;
                }
                step = P::MorD;
            }
        }

        i += 1;
    }

    println!("{}", total);
}

fn check_letter(to_check: &char, example: char, if_correct_value: P, to_set: &mut P) {
    if *to_check == example {
        *to_set = if_correct_value;
    } else {
        *to_set = P::MorD;
    }
}

fn check_num(
    first: &char,
    num_str: &mut String,
    current_step: &mut P,
    iter: &mut std::iter::Peekable<std::str::Chars<'_>>,
    i: &mut usize,
) {
    if !first.is_digit(10) {
        *current_step = P::MorD;
        return;
    }

    let &second = iter.peek().unwrap();
    if !second.is_digit(10) {
        if second == ',' && matches!(*current_step, P::A) {
            *num_str = String::from(*first);
            *current_step = P::Comma;
        } else if second == ')' && matches!(*current_step, P::B) {
            *num_str = String::from(*first);
            *current_step = P::MulClose;
        } else {
            *current_step = P::MorD;
        }

        return;
    }
    iter.next();
    *i += 1;

    let &third = iter.peek().unwrap();
    if !third.is_digit(10) {
        if third == ',' && matches!(*current_step, P::A) {
            let vec = Vec::from([*first, second]);
            *num_str = vec.iter().collect();
            *current_step = P::Comma;
        } else if third == ')' && matches!(*current_step, P::B) {
            let vec = Vec::from([*first, second]);
            *num_str = vec.iter().collect();
            *current_step = P::MulClose;
        } else {
            *current_step = P::MorD;
        }

        return;
    }
    iter.next();
    *i += 1;

    let vec = Vec::from([*first, second, third]);
    *num_str = vec.iter().collect();

    match current_step {
        P::A => *current_step = P::Comma,
        P::B => *current_step = P::MulClose,
        _ => (),
    }
}
