// 이진 탐색 (Binary Search)
// Clang/06/BinarySearch 포팅.
use ch06::points_data::{data_set, Point};
use std::cmp::Ordering;

fn compare_point(a: &Point, b: &Point) -> Ordering {
    if a.point > b.point {
        Ordering::Greater
    } else if a.point < b.point {
        Ordering::Less
    } else {
        Ordering::Equal
    }
}

fn binary_search(list: &[Point], target: f64) -> Option<&Point> {
    let mut left: i64 = 0;
    let mut right: i64 = list.len() as i64 - 1;

    while left <= right {
        let mid = ((left + right) / 2) as usize;

        if target == list[mid].point {
            return Some(&list[mid]);
        } else if target > list[mid].point {
            left = mid as i64 + 1;
        } else {
            right = mid as i64 - 1;
        }
    }

    None
}

fn main() {
    let mut data = data_set();

    // 구매포인트에 대해 오름차순으로 정렬
    data.sort_by(compare_point);

    // 671.78 포인트 고객 찾기
    let found = binary_search(&data, 671.78).unwrap();

    println!("found... ID: {}, Point: {:.6} ", found.id, found.point);
}
