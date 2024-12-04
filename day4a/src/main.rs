use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut lines: Vec<Vec<char>> = Vec::new();
    let mut total = 0;

    for line in reader.lines() {
        let string = line.unwrap();
        let mut line_vec: Vec<char> = Vec::new();
        for char in string.chars() {
            line_vec.push(char);
        }
        lines.push(line_vec);
    }

    for vert_i in 0..lines.len() {
        for horz_i in 0..lines[vert_i].len() {
            if lines[vert_i][horz_i] == 'X' {
                search(vert_i, horz_i, &lines, &mut total, 0, 1);
                search(vert_i, horz_i, &lines, &mut total, 0, -1);
                search(vert_i, horz_i, &lines, &mut total, 1, 0);
                search(vert_i, horz_i, &lines, &mut total, -1, 0);
                search(vert_i, horz_i, &lines, &mut total, -1, 1);
                search(vert_i, horz_i, &lines, &mut total, 1, 1);
                search(vert_i, horz_i, &lines, &mut total, 1, -1);
                search(vert_i, horz_i, &lines, &mut total, -1, -1);
            }
        }
    }

    println!("{}", total);
}

fn carry_on(char: &char, example: &mut char, total: &mut i32) -> bool {
    if *example == 'M' && *char == 'M' {
        *example = 'A';
        return true;
    } else if *example == 'A' && *char == 'A' {
        *example = 'S';
        return true;
    } else if *example == 'S' && *char == 'S' {
        *total += 1;
    }

    return false;
}

fn search(
    vert_i: usize,
    horz_i: usize,
    lines: &Vec<Vec<char>>,
    total: &mut i32,
    vert_increment: i32,
    horz_increment: i32,
) {
    let vert_limit = if vert_increment > 0 {
        lines.len() as i32
    } else {
        -1
    };

    let horz_limit = if horz_increment > 0 {
        lines[vert_i].len() as i32
    } else {
        -1
    };

    let mut letter = 'M';
    let mut v = vert_i as i32;
    let mut h = horz_i as i32;

    v += vert_increment;
    h += horz_increment;

    while v != vert_limit && h != horz_limit {
        let char = lines[v as usize][h as usize];

        if !carry_on(&char, &mut letter, total) {
            break;
        }

        v += vert_increment;
        h += horz_increment;
    }
}
