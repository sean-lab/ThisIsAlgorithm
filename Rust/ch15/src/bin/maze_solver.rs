use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process::ExitCode;

const START: u8 = b'S';
const GOAL: u8 = b'G';
const WAY: u8 = b' ';
const WALL: u8 = b'#';
const MARKED: u8 = b'+';

const NORTH: i32 = 0;
const SOUTH: i32 = 1;
const EAST: i32 = 2;
const WEST: i32 = 3;

const FAIL: i32 = 0;
const SUCCEED: i32 = 1;

struct MazeInfo {
    column_size: i32,
    row_size: i32,
    data: Vec<Vec<u8>>,
}

#[derive(Clone, Copy)]
struct Position {
    x: i32,
    y: i32,
}

fn solve(maze: &mut MazeInfo) -> i32 {
    let mut start_found = FAIL;
    let mut result = FAIL;
    let mut start = Position { x: 0, y: 0 };

    for i in 0..maze.row_size {
        for j in 0..maze.column_size {
            if maze.data[i as usize][j as usize] == START {
                start.x = j;
                start.y = i;
                start_found = SUCCEED;
                break;
            }
        }
    }

    if start_found == FAIL {
        return FAIL;
    }

    if move_to(maze, &start, NORTH) == SUCCEED {
        result = SUCCEED;
    } else if move_to(maze, &start, SOUTH) == SUCCEED {
        result = SUCCEED;
    } else if move_to(maze, &start, EAST) == SUCCEED {
        result = SUCCEED;
    } else if move_to(maze, &start, WEST) == SUCCEED {
        result = SUCCEED;
    }

    maze.data[start.y as usize][start.x as usize] = START;

    result
}

fn move_to(maze: &mut MazeInfo, current: &Position, _direction: i32) -> i32 {
    let dirs = [NORTH, SOUTH, EAST, WEST];

    if maze.data[current.y as usize][current.x as usize] == GOAL {
        return SUCCEED;
    }

    maze.data[current.y as usize][current.x as usize] = MARKED;

    for i in 0..4 {
        let mut next = Position { x: 0, y: 0 };
        if get_next_step(maze, current, dirs[i], &mut next) == FAIL {
            continue;
        }

        if move_to(maze, &next, NORTH) == SUCCEED {
            return SUCCEED;
        }
    }

    maze.data[current.y as usize][current.x as usize] = WAY;

    FAIL
}

fn get_next_step(maze: &MazeInfo, current: &Position, direction: i32, next: &mut Position) -> i32 {
    match direction {
        NORTH => {
            next.x = current.x;
            next.y = current.y - 1;
            if next.y == -1 {
                return FAIL;
            }
        }
        SOUTH => {
            next.x = current.x;
            next.y = current.y + 1;
            if next.y == maze.row_size {
                return FAIL;
            }
        }
        EAST => {
            next.x = current.x + 1;
            next.y = current.y;
            if next.x == maze.column_size {
                return FAIL;
            }
        }
        WEST => {
            next.x = current.x - 1;
            next.y = current.y;
            if next.x == -1 {
                return FAIL;
            }
        }
        _ => {}
    }

    if maze.data[next.y as usize][next.x as usize] == WALL {
        return FAIL;
    }
    if maze.data[next.y as usize][next.x as usize] == MARKED {
        return FAIL;
    }

    SUCCEED
}

fn get_maze(file_path: &str, maze: &mut MazeInfo) -> i32 {
    let fp = match File::open(file_path) {
        Ok(f) => f,
        Err(_) => {
            println!("Cannot open file:{}", file_path);
            return FAIL;
        }
    };

    let reader = BufReader::new(fp);
    let mut lines: Vec<Vec<u8>> = Vec::new();
    let mut column_size = INIT_VALUE;

    for line in reader.split(b'\n') {
        // `split(b'\n')` strips the newline, matching C's `strlen(buffer) - 1`.
        let buffer = line.unwrap();
        let len = buffer.len() as i32;

        if column_size == INIT_VALUE {
            column_size = len;
        } else if column_size != len {
            println!("Maze data in file:{} is not valid.", file_path);
            return FAIL;
        }

        lines.push(buffer);
    }

    let row_size = lines.len() as i32;
    maze.row_size = row_size;
    maze.column_size = column_size;
    maze.data = Vec::with_capacity(row_size as usize);

    for line in &lines {
        let mut row = vec![0u8; column_size as usize];
        for j in 0..column_size as usize {
            row[j] = line[j];
        }
        maze.data.push(row);
    }

    SUCCEED
}

const INIT_VALUE: i32 = -1;

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
