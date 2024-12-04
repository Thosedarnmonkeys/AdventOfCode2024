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
            if lines[vert_i][horz_i] == 'A'
                && vert_i + 1 < lines.len()
                && vert_i > 0
                && horz_i + 1 < lines[vert_i].len()
                && horz_i > 0
            {
                let ul = lines[vert_i - 1][horz_i - 1];
                let ur = lines[vert_i - 1][horz_i + 1];
                let dl = lines[vert_i + 1][horz_i - 1];
                let dr = lines[vert_i + 1][horz_i + 1];

                let mut m_count = 0;
                let mut s_count = 0;

                check_and_count(&ul, &mut m_count, &mut s_count);
                check_and_count(&ur, &mut m_count, &mut s_count);
                check_and_count(&dl, &mut m_count, &mut s_count);
                check_and_count(&dr, &mut m_count, &mut s_count);

                if m_count == 2 && s_count == 2 && ul != dr {
                    total += 1;
                }
            }
        }
    }

    println!("{}", total);
}

fn check_and_count(char: &char, m_count: &mut i32, s_count: &mut i32) {
    if *char == 'M' {
        *m_count += 1;
    }

    if *char == 'S' {
        *s_count += 1;
    }
}
