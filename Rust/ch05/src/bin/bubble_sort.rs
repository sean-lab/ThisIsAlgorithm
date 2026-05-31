// 버블 정렬 (Bubble Sort)
// Clang/05/BubbleSort 포팅. 로직은 ch05::bubble_sort 모듈에 있다.
use ch05::bubble_sort::bubble_sort;

fn main() {
    let mut data_set = [6, 4, 2, 3, 1, 5];

    bubble_sort(&mut data_set);

    for value in &data_set {
        print!("{} ", value);
    }
    println!();
}
