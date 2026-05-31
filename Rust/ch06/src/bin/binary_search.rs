// 이진 탐색 (Binary Search)
// Clang/06/BinarySearch 포팅. 로직은 ch06::search 모듈에 있다.
use ch06::points_data::data_set;
use ch06::search::{binary_search, compare_point};

fn main() {
    let mut data = data_set();

    // 구매포인트에 대해 오름차순으로 정렬
    data.sort_by(compare_point);

    // 671.78 포인트 고객 찾기
    let found = binary_search(&data, 671.78).unwrap();

    println!("found... ID: {}, Point: {:.6} ", found.id, found.point);
}
