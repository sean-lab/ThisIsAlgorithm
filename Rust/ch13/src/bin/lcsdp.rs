// Longest Common Subsequence (Dynamic Programming, with traceback) - 로직은 ch13::lcsdp 모듈에 있다.
// 버그 수정 내용은 ch13::lcsdp 모듈과 bug-fix-report.md 참조.
use ch13::lcsdp::{lcs, lcs_print_table, lcs_trace_back};

fn main() {
    let x = b"GOOD MORNING.";
    let y = b"GUTEN MORGEN.";

    let len_x = x.len();
    let len_y = y.len();

    let mut table = vec![vec![0i32; len_y + 1]; len_x + 1];

    let length = lcs(x, y, len_x, len_y, &mut table);

    lcs_print_table(&table, x, y, len_x, len_y);

    let mut result = String::new();
    lcs_trace_back(x, y, len_x, len_y, &table, &mut result);

    println!();
    println!("LCS:\"{}\" (Length:{})", result, length);
}
