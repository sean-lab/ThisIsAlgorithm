// 병합 정렬 (Merge Sort) - 로직은 ch12::merge_sort 모듈에 있다.
use ch12::merge_sort::merge_sort;

fn main() {
    let mut data_set = [334, 6, 4, 2, 3, 1, 5, 117, 12, 34];
    let length = data_set.len() as i32;

    merge_sort(&mut data_set, 0, length - 1);

    for i in 0..length {
        print!("{} ", data_set[i as usize]);
    }

    println!();
}
