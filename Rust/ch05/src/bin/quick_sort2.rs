// qsort 표준 라이브러리 정렬 (QuickSort2)
// Clang/05/QuickSort2 포팅. C의 qsort + 비교 함수를 sort_by + Ordering 으로 옮겼다.
use std::cmp::Ordering;

// C의 ComparePoint 와 동일한 비교 함수.
fn compare_point(elem1: &i32, elem2: &i32) -> Ordering {
    if *elem1 > *elem2 {
        Ordering::Greater
    } else if *elem1 < *elem2 {
        Ordering::Less
    } else {
        Ordering::Equal
    }
}

fn main() {
    let mut data_set = [6, 4, 2, 3, 1, 5];

    data_set.sort_by(compare_point);

    for value in &data_set {
        print!("{} ", value);
    }
    println!();
}
