use std::env;
use std::fs::File;
use std::io::{self, BufReader, Read, Write};
use std::process::ExitCode;

/// Emulates C `fgets(buf, MAX_BUFFER, fp)` with MAX_BUFFER = 512:
/// reads at most 511 bytes, stopping early at a newline. Returns an empty
/// vector at end of file.
fn fgets(reader: &mut impl Read, buf: &mut Vec<u8>) {
    buf.clear();
    let mut byte = [0u8; 1];
    while buf.len() < 511 {
        match reader.read(&mut byte) {
            Ok(0) => break,
            Ok(_) => {
                buf.push(byte[0]);
                if byte[0] == b'\n' {
                    break;
                }
            }
            Err(_) => break,
        }
    }
}

fn print_table(out: &mut impl Write, table: &[i32], table_size: i32) {
    for i in 0..table_size {
        if table[i as usize] > 0 {
            write!(out, "'{}':{}, ", i as u8 as char, table[i as usize]).unwrap();
        }
    }
    writeln!(out).unwrap();
}

fn print_good_suff_table(
    out: &mut impl Write,
    table: &[i32],
    pos_of_border: &[i32],
    pattern: &[u8],
    table_size: i32,
) {
    for i in 0..table_size {
        if table[i as usize] > 0 {
            let ch = pattern.get(i as usize).copied().unwrap_or(0) as char;
            write!(
                out,
                "'{}':{}({}), ",
                ch, table[i as usize], pos_of_border[i as usize]
            )
            .unwrap();
        }
    }
    writeln!(out).unwrap();
}

fn build_bst(pattern: &[u8], pattern_size: i32, bad_char_table: &mut [i32]) {
    for i in 0..128 {
        bad_char_table[i] = -1;
    }
    for j in 0..pattern_size {
        bad_char_table[pattern[j as usize] as usize] = j;
    }
}

fn build_gst(
    out: &mut impl Write,
    pattern: &[u8],
    pattern_size: i32,
    pos_of_border: &mut [i32],
    good_suff_table: &mut [i32],
) {
    // Case 1
    let mut i = pattern_size;
    let mut j = pattern_size + 1;

    pos_of_border[i as usize] = j;

    while i > 0 {
        writeln!(
            out,
            "{}:{}) {}, i:{}, j:{}",
            "BuildGST",
            83,
            std::str::from_utf8(pattern).unwrap_or(""),
            i,
            j
        )
        .unwrap();
        while j <= pattern_size && pattern[(i - 1) as usize] != pattern[(j - 1) as usize] {
            if good_suff_table[j as usize] == 0 {
                good_suff_table[j as usize] = j - i;
            }
            j = pos_of_border[j as usize];
        }
        i -= 1;
        j -= 1;
        pos_of_border[i as usize] = j;
    }

    // Case 2
    j = pos_of_border[0];

    for i in 0..=pattern_size {
        if good_suff_table[i as usize] == 0 {
            good_suff_table[i as usize] = j;
        }
        if i == j {
            j = pos_of_border[j as usize];
        }
    }
}

fn max(a: i32, b: i32) -> i32 {
    if a > b {
        a
    } else {
        b
    }
}

fn boyer_moore(
    out: &mut impl Write,
    text: &[u8],
    text_size: i32,
    start: i32,
    pattern: &[u8],
    pattern_size: i32,
) -> i32 {
    let mut bad_char_table = [0i32; 128];
    let mut good_suff_table = vec![0i32; (pattern_size + 1) as usize];
    let mut pos_of_border = vec![0i32; (pattern_size + 1) as usize];
    let mut i = start;
    let mut position = -1;

    build_bst(pattern, pattern_size, &mut bad_char_table);
    build_gst(out, pattern, pattern_size, &mut pos_of_border, &mut good_suff_table);

    print_table(out, &bad_char_table, 128);
    print_good_suff_table(out, &good_suff_table, &pos_of_border, pattern, pattern_size + 1);

    while i <= text_size - pattern_size {
        let mut j = pattern_size - 1;

        while j >= 0 && pattern[j as usize] == text[(i + j) as usize] {
            j -= 1;
        }

        if j < 0 {
            position = i;
            break;
        } else {
            i += max(
                good_suff_table[(j + 1) as usize],
                j - bad_char_table[text[(i + j) as usize] as usize],
            );
        }
    }

    position
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Usage: {} <FilePath> <Pattern>", args[0]);
        return ExitCode::from(1);
    }

    let file_path = &args[1];
    let pattern = args[2].as_bytes();
    let pattern_size = pattern.len() as i32;

    let fp = match File::open(file_path) {
        Ok(f) => f,
        Err(_) => {
            println!("Cannot open file:{}", file_path);
            return ExitCode::from(1);
        }
    };

    let mut reader = BufReader::new(fp);
    let stdout = io::stdout();
    let mut out = stdout.lock();
    let mut line = 0;
    let mut text: Vec<u8> = Vec::new();

    loop {
        fgets(&mut reader, &mut text);
        if text.is_empty() {
            break;
        }
        let position = boyer_moore(&mut out, &text, text.len() as i32, 0, pattern, pattern_size);
        line += 1;
        if position >= 0 {
            write!(out, "line:{}, column:{} : ", line, position + 1).unwrap();
            out.write_all(&text).unwrap();
        }
    }

    ExitCode::SUCCESS
}
