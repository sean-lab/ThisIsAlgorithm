// Longest Common Subsequence (Dynamic Programming, with traceback).
// Port of Clang/13/LCSDP/LCSDP.c
//
// BUG FIX: the original `main` sizes the result buffer with
//     size_t TableSize = sizeof( Table.Data[LEN_X][LEN_Y] + 1 );
// `sizeof(int + 1)` is always 4, so only 4 bytes are allocated for a
// result string that can be much longer (here "G MORN." needs 8 bytes).
// `LCS_TraceBack` then writes past the end of the buffer (heap buffer
// overflow / undefined behavior). This port uses a growable String, so
// the result is always sized correctly. See bug-fix-report.md.

pub fn lcs(x: &[u8], y: &[u8], i: usize, j: usize, table: &mut Vec<Vec<i32>>) -> i32 {
    for m in 0..=i {
        table[m][0] = 0;
    }
    for n in 0..=j {
        table[0][n] = 0;
    }

    for m in 1..=i {
        for n in 1..=j {
            if x[m - 1] == y[n - 1] {
                table[m][n] = table[m - 1][n - 1] + 1;
            } else if table[m][n - 1] >= table[m - 1][n] {
                table[m][n] = table[m][n - 1];
            } else {
                table[m][n] = table[m - 1][n];
            }
        }
    }

    table[i][j]
}

pub fn lcs_trace_back(
    x: &[u8],
    y: &[u8],
    m: usize,
    n: usize,
    table: &[Vec<i32>],
    lcs: &mut String,
) {
    if m == 0 || n == 0 {
        return;
    }

    if table[m][n] > table[m][n - 1]
        && table[m][n] > table[m - 1][n]
        && table[m][n] > table[m - 1][n - 1]
    {
        let temp = lcs.clone();
        *lcs = format!("{}{}", x[m - 1] as char, temp);

        lcs_trace_back(x, y, m - 1, n - 1, table, lcs);
    } else if table[m][n] > table[m - 1][n] && table[m][n] == table[m][n - 1] {
        lcs_trace_back(x, y, m, n - 1, table, lcs);
    } else {
        lcs_trace_back(x, y, m - 1, n, table, lcs);
    }
}

pub fn lcs_print_table(table: &[Vec<i32>], x: &[u8], y: &[u8], len_x: usize, len_y: usize) {
    print!("{:4}", "");

    for i in 0..len_y + 1 {
        let c = if i < y.len() { y[i] } else { 0 };
        print!("{} ", c as char);
    }
    println!();

    for i in 0..len_x + 1 {
        if i == 0 {
            print!("{:2}", "");
        } else {
            print!("{:<2}", x[i - 1] as char);
        }

        for j in 0..len_y + 1 {
            print!("{} ", table[i][j]);
        }

        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(x: &[u8], y: &[u8]) -> (i32, String) {
        let mut table = vec![vec![0i32; y.len() + 1]; x.len() + 1];
        let length = lcs(x, y, x.len(), y.len(), &mut table);
        let mut result = String::new();
        lcs_trace_back(x, y, x.len(), y.len(), &table, &mut result);
        (length, result)
    }

    #[test]
    fn main_program_lcs_and_traceback() {
        let (length, result) = run(b"GOOD MORNING.", b"GUTEN MORGEN.");
        assert_eq!(length, 7);
        assert_eq!(result, "G MORN.");
    }

    #[test]
    fn identical_strings() {
        let (length, result) = run(b"ABC", b"ABC");
        assert_eq!(length, 3);
        assert_eq!(result, "ABC");
    }

    #[test]
    fn no_common_subsequence() {
        let (length, result) = run(b"ABC", b"XYZ");
        assert_eq!(length, 0);
        assert_eq!(result, "");
    }
}
