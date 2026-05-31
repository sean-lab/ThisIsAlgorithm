// 삽입 정렬 (Insertion Sort)
// Clang/05/InsertionSort 포팅. memmove 부분은 Vec 슬라이스 이동으로 옮겼다.

fn insertion_sort(data_set: &mut [i32]) {
    let length = data_set.len();

    for i in 1..length {
        if data_set[i - 1] <= data_set[i] {
            continue;
        }

        let value = data_set[i];

        for j in 0..i {
            if data_set[j] > value {
                // memmove(&DataSet[j+1], &DataSet[j], sizeof * (i-j))
                data_set.copy_within(j..i, j + 1);
                data_set[j] = value;
                break;
            }
        }
    }
}

fn main() {
    let mut data_set = [6, 4, 2, 3, 1, 5];

    insertion_sort(&mut data_set);

    for value in &data_set {
        print!("{} ", value);
    }
    println!();
}
