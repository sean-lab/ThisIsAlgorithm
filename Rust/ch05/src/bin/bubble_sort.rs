// 버블 정렬 (Bubble Sort)
// Clang/05/BubbleSort 포팅.

fn bubble_sort(data_set: &mut [i32]) {
    let length = data_set.len();
    if length == 0 {
        return;
    }
    for i in 0..length - 1 {
        for j in 0..length - (i + 1) {
            if data_set[j] > data_set[j + 1] {
                data_set.swap(j, j + 1);
            }
        }
    }
}

fn main() {
    let mut data_set = [6, 4, 2, 3, 1, 5];

    bubble_sort(&mut data_set);

    for value in &data_set {
        print!("{} ", value);
    }
    println!();
}
