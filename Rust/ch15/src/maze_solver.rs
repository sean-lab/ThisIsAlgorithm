// 미로 찾기 (Maze Solver, 백트래킹)
// Clang/15/MazeSolver 포팅.

use std::fs::File;
use std::io::{BufRead, BufReader};

pub const START: u8 = b'S';
pub const GOAL: u8 = b'G';
pub const WAY: u8 = b' ';
pub const WALL: u8 = b'#';
pub const MARKED: u8 = b'+';

pub const NORTH: i32 = 0;
pub const SOUTH: i32 = 1;
pub const EAST: i32 = 2;
pub const WEST: i32 = 3;

pub const FAIL: i32 = 0;
pub const SUCCEED: i32 = 1;

pub const INIT_VALUE: i32 = -1;

pub struct MazeInfo {
    pub column_size: i32,
    pub row_size: i32,
    pub data: Vec<Vec<u8>>,
}

#[derive(Clone, Copy)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

pub fn solve(maze: &mut MazeInfo) -> i32 {
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

pub fn move_to(maze: &mut MazeInfo, current: &Position, _direction: i32) -> i32 {
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

pub fn get_next_step(
    maze: &MazeInfo,
    current: &Position,
    direction: i32,
    next: &mut Position,
) -> i32 {
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

pub fn get_maze(file_path: &str, maze: &mut MazeInfo) -> i32 {
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

#[cfg(test)]
mod tests {
    use super::*;

    fn maze_from(rows: &[&str]) -> MazeInfo {
        let data: Vec<Vec<u8>> = rows.iter().map(|r| r.as_bytes().to_vec()).collect();
        MazeInfo {
            column_size: data[0].len() as i32,
            row_size: data.len() as i32,
            data,
        }
    }

    #[test]
    fn solves_simple_maze() {
        // S 에서 G 까지 길이 존재.
        let mut maze = maze_from(&["S G"]);
        assert_eq!(solve(&mut maze), SUCCEED);
        // 시작점은 다시 START 로 복원된다.
        assert_eq!(maze.data[0][0], START);
        // 통로는 지나간 표시(+)로 남는다.
        assert_eq!(maze.data[0][1], MARKED);
    }

    #[test]
    fn unsolvable_when_walled_off() {
        // S 와 G 사이가 벽으로 막혀 있음.
        let mut maze = maze_from(&["S#G"]);
        assert_eq!(solve(&mut maze), FAIL);
    }

    #[test]
    fn fails_without_start() {
        let mut maze = maze_from(&["  G"]);
        assert_eq!(solve(&mut maze), FAIL);
    }
}
