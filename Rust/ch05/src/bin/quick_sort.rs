// 퀵 정렬 (Quick Sort)
// Clang/05/QuickSort 포팅. 로직은 ch05::quick_sort 모듈에 있다.
use ch05::quick_sort::quick_sort;

fn main() {
    let mut data_set = [6, 4, 2, 3, 1, 5];
    let length = data_set.len() as i32;

    quick_sort(&mut data_set, 0, length - 1);

    for value in &data_set {
        print!("{} ", value);
    }
    println!();
}
