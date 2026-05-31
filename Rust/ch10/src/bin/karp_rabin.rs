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

fn hash(string: &[u8], size: i32) -> i32 {
    let mut hash_value: i32 = 0;
    let mut i = 0;
    while i < size {
        // C `char` is signed on x86/arm.
        hash_value = (string[i as usize] as i8 as i32).wrapping_add(hash_value.wrapping_mul(2));
        i += 1;
    }
    hash_value
}

fn re_hash(string: &[u8], start: i32, size: i32, hash_prev: i32, coefficient: i32) -> i32 {
    if start == 0 {
        return hash_prev;
    }
    let last = string[(start + size - 1) as usize] as i8 as i32;
    let first = string[(start - 1) as usize] as i8 as i32;
    last.wrapping_add(hash_prev.wrapping_sub(coefficient.wrapping_mul(first)).wrapping_mul(2))
}

fn karp_rabin(text: &[u8], text_size: i32, start: i32, pattern: &[u8], pattern_size: i32) -> i32 {
    let coefficient = 2f64.powi(pattern_size - 1) as i32;
    let mut hash_text = hash(text, pattern_size);
    let hash_pattern = hash(pattern, pattern_size);

    let mut i = start;
    while i <= text_size - pattern_size {
        hash_text = re_hash(text, i, pattern_size, hash_text, coefficient);

        if hash_pattern == hash_text {
            let mut j = 0;
            while j < pattern_size {
                if text[(i + j) as usize] != pattern[j as usize] {
                    break;
                }
                j += 1;
            }
            if j >= pattern_size {
                return i;
            }
        }
        i += 1;
    }

    -1
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
        let position = karp_rabin(&text, text.len() as i32, 0, pattern, pattern_size);
        line += 1;
        if position >= 0 {
            write!(out, "line:{}, column:{} : ", line, position + 1).unwrap();
            out.write_all(&text).unwrap();
        }
    }

    ExitCode::SUCCESS
}
