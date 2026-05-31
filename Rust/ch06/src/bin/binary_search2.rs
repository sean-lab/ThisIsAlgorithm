// 표준 라이브러리 이진 탐색 (BinarySearch2)
// Clang/06/BinarySearch2 포팅. C의 qsort + bsearch 를 sort_by + binary_search_by 로 옮겼다.
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

fn main() {
    let mut data = data_set();

    // 점수의 오름차순으로 정렬
    data.sort_by(compare_point);

    // 671.78 점을 받은 학생 찾기
    let target = Point { id: 0, point: 671.78 };

    let found = data
        .binary_search_by(|probe| compare_point(probe, &target))
        .ok()
        .map(|idx| &data[idx])
        .unwrap();

    println!("found... ID: {}, Point: {:.6} ", found.id, found.point);
}
