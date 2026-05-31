// 퀵 정렬 (Quick Sort)
// Clang/05/QuickSort 포팅. 분할(Partition) 로직을 그대로 옮겼다.

fn swap(data_set: &mut [i32], a: usize, b: usize) {
    data_set.swap(a, b);
}

fn partition(data_set: &mut [i32], left: i32, right: i32) -> i32 {
    let first = left;
    let pivot = data_set[first as usize];

    let mut left = left + 1;
    let mut right = right;

    while left <= right {
        while data_set[left as usize] <= pivot && left < right {
            left += 1;
        }

        while data_set[right as usize] >= pivot && left <= right {
            right -= 1;
        }

        if left < right {
            swap(data_set, left as usize, right as usize);
        } else {
            break;
        }
    }

    swap(data_set, first as usize, right as usize);

    right
}

fn quick_sort(data_set: &mut [i32], left: i32, right: i32) {
    if left < right {
        let index = partition(data_set, left, right);

        quick_sort(data_set, left, index - 1);
        quick_sort(data_set, index + 1, right);
    }
}

fn main() {
    let mut data_set = [6, 4, 2, 3, 1, 5];
    let length = data_set.len() as i32;

    quick_sort(&mut data_set, 0, length - 1);

    for value in &data_set {
        print!("{} ", value);
    }
    println!();
}
