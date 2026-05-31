// N-Queens (백트래킹) - 로직은 ch15::n_queens 모듈에 있다.
use ch15::n_queens::find_solution_for_queen;
use std::env;
use std::process::ExitCode;

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
