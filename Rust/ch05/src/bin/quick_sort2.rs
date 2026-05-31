// qsort 표준 라이브러리 정렬 (QuickSort2)
// Clang/05/QuickSort2 포팅. 로직은 ch05::quick_sort2 모듈에 있다.
use ch05::quick_sort2::compare_point;

fn main() {
    let mut data_set = [6, 4, 2, 3, 1, 5];

    data_set.sort_by(compare_point);

    for value in &data_set {
        print!("{} ", value);
    }
    println!();
}
