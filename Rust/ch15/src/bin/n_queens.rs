use std::env;
use std::process::ExitCode;

fn find_solution_for_queen(
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

fn is_threatened(columns: &[i32], new_row: i32) -> bool {
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

fn print_solution(columns: &[i32], number_of_queens: i32) {
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

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        print!("Usage: {} <Number Of Queens>", args[0]);
        return ExitCode::from(1);
    }

    let number_of_queens: i32 = args[1].parse().unwrap_or(0);
    let mut columns = vec![0i32; number_of_queens.max(0) as usize];
    let mut solution_count = 0;

    for i in 0..number_of_queens {
        columns[0] = i;
        find_solution_for_queen(&mut columns, 0, number_of_queens, &mut solution_count);
    }

    ExitCode::SUCCESS
}
