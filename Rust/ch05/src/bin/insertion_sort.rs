// 삽입 정렬 (Insertion Sort)
// Clang/05/InsertionSort 포팅. 로직은 ch05::insertion_sort 모듈에 있다.
use ch05::insertion_sort::insertion_sort;

fn main() {
    let mut data_set = [6, 4, 2, 3, 1, 5];

    insertion_sort(&mut data_set);

    for value in &data_set {
        print!("{} ", value);
    }
    println!();
}
