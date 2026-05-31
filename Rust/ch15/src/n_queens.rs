// N-Queens (백트래킹)
// Clang/15/NQueens 포팅.

pub fn find_solution_for_queen(
    columns: &mut [i32],
    row: i32,
    number_of_queens: i32,
    solution_count: &mut i32,
) {
    if is_threatened(columns, row) {
        return;
    }

    if row == number_of_queens - 1 {
        *solution_count += 1;
        println!("Solution #{} : ", *solution_count);
        print_solution(columns, number_of_queens);
    } else {
        for i in 0..number_of_queens {
            columns[(row + 1) as usize] = i;
            find_solution_for_queen(columns, row + 1, number_of_queens, solution_count);
        }
    }
}

pub fn is_threatened(columns: &[i32], new_row: i32) -> bool {
    let mut current_row = 0;

    while current_row < new_row {
        if columns[new_row as usize] == columns[current_row as usize]
            || (columns[new_row as usize] - columns[current_row as usize]).abs()
                == (new_row - current_row).abs()
        {
            return true;
        }
        current_row += 1;
    }

    false
}

pub fn print_solution(columns: &[i32], number_of_queens: i32) {
    for i in 0..number_of_queens {
        for j in 0..number_of_queens {
            if columns[i as usize] == j {
                print!("Q");
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;

    // 출력 없이 해의 개수만 세는 테스트용 헬퍼 (find_solution_for_queen 과 동일 로직).
    fn count_for_queen(columns: &mut [i32], row: i32, n: i32, count: &mut i32) {
        if is_threatened(columns, row) {
            return;
        }
        if row == n - 1 {
            *count += 1;
        } else {
            for i in 0..n {
                columns[(row + 1) as usize] = i;
                count_for_queen(columns, row + 1, n, count);
            }
        }
    }

    fn count_solutions(n: i32) -> i32 {
        let mut columns = vec![0i32; n as usize];
        let mut count = 0;
        for i in 0..n {
            columns[0] = i;
            count_for_queen(&mut columns, 0, n, &mut count);
        }
        count
    }

    #[test]
    fn is_threatened_detects_same_column() {
        // row 0 과 row 2 가 같은 열(3) -> 위협.
        let cols = [3, 1, 3];
        assert!(is_threatened(&cols, 2));
    }

    #[test]
    fn is_threatened_detects_diagonal() {
        // row 0 = col 0, row 1 = col 1 -> 대각선 위협.
        let cols = [0, 1];
        assert!(is_threatened(&cols, 1));
    }

    #[test]
    fn known_solution_counts() {
        assert_eq!(count_solutions(4), 2);
        assert_eq!(count_solutions(6), 4);
        assert_eq!(count_solutions(8), 92);
    }
}
