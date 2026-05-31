// 미로 찾기 (Maze Solver, 백트래킹) - 로직은 ch15::maze_solver 모듈에 있다.
use ch15::maze_solver::{get_maze, solve, MazeInfo, FAIL};
use std::env;
use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: MazeSolver <MazeFile>");
        return ExitCode::SUCCESS;
    }

    let mut maze = MazeInfo {
        column_size: 0,
        row_size: 0,
        data: Vec::new(),
    };

    if get_maze(&args[1], &mut maze) == FAIL {
        return ExitCode::SUCCESS;
    }

    if solve(&mut maze) == FAIL {
        return ExitCode::SUCCESS;
    }

    for i in 0..maze.row_size {
        for j in 0..maze.column_size {
            print!("{}", maze.data[i as usize][j as usize] as char);
        }
        println!();
    }

    ExitCode::SUCCESS
}
