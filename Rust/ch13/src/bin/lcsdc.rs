// Longest Common Subsequence (Divide and Conquer / recursive table fill).
// Port of Clang/13/LCSDC/LCSDC.c
//
// BUG FIX: the original `LCS_PrintTable` prints the row header with
// `printf("%c ", X[i - 1])`. For the first row (i == 0) this evaluates
// `X[-1]`, an out-of-bounds read one byte before the string literal
// (undefined behavior; on the reference build it happened to emit a NUL
// byte). This port prints a blank cell for row 0, matching the corrected
// behavior found in LCSDP. See bug-fix-report.md.

fn lcs(x: &[u8], y: &[u8], i: usize, j: usize, table: &mut Vec<Vec<i32>>) -> i32 {
    if i == 0 || j == 0 {
        table[i][j] = 0;
        table[i][j]
    } else if x[i - 1] == y[j - 1] {
        table[i][j] = lcs(x, y, i - 1, j - 1, table) + 1;
        table[i][j]
    } else {
        let a = lcs(x, y, i - 1, j, table);
        let b = lcs(x, y, i, j - 1, table);

        table[i][j] = if a > b { a } else { b };
        table[i][j]
    }
}

fn lcs_print_table(table: &[Vec<i32>], x: &[u8], y: &[u8], len_x: usize, len_y: usize) {
    print!("{:<4}", " ");

    for i in 0..len_y + 1 {
        let c = if i < y.len() { y[i] } else { 0 };
        print!("{} ", c as char);
    }
    println!();

    for i in 0..len_x + 1 {
        // FIX: original read X[i - 1] (== X[-1] when i == 0). Print blank.
        if i == 0 {
            print!("{:2}", "");
        } else {
            print!("{} ", x[i - 1] as char);
        }

        for j in 0..len_y + 1 {
            print!("{} ", table[i][j]);
        }

        println!();
    }
}

fn main() {
    let x = b"GOOD MORNING.";
    let y = b"GUTEN MORGEN.";

    let len_x = x.len();
    let len_y = y.len();

    let mut table = vec![vec![0i32; len_y + 1]; len_x + 1];

    let _lcs_length = lcs(x, y, len_x, len_y, &mut table);

    lcs_print_table(&table, x, y, len_x, len_y);
}
