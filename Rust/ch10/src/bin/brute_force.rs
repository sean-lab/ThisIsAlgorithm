use std::env;
use std::fs::File;
use std::io::{self, BufReader, Read, Write};
use std::process::ExitCode;

use ch10::brute_force::brute_force;

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
        let position = brute_force(&text, text.len() as i32, 0, pattern, pattern_size);
        line += 1;
        if position >= 0 {
            write!(out, "line:{}, column:{} : ", line, position + 1).unwrap();
            out.write_all(&text).unwrap();
        }
    }

    ExitCode::SUCCESS
}
