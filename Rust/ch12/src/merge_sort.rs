// 병합 정렬 (Merge Sort)
// Clang/12/MergeSort 포팅.

pub fn merge_sort(data_set: &mut [i32], start_index: i32, end_index: i32) {
    if end_index - start_index < 1 {
        return;
    }

    let middle_index = (start_index + end_index) / 2;

    merge_sort(data_set, start_index, middle_index);
    merge_sort(data_set, middle_index + 1, end_index);

    merge(data_set, start_index, middle_index, end_index);
}

pub fn merge(data_set: &mut [i32], start_index: i32, middle_index: i32, end_index: i32) {
    let mut left_index = start_index;
    let mut right_index = middle_index + 1;
    let mut dest_index = 0usize;

    let mut destination = vec![0i32; (end_index - start_index + 1) as usize];

    while left_index <= middle_index && right_index <= end_index {
        if data_set[left_index as usize] < data_set[right_index as usize] {
            destination[dest_index] = data_set[left_index as usize];
            left_index += 1;
        } else {
            destination[dest_index] = data_set[right_index as usize];
            right_index += 1;
        }
        dest_index += 1;
    }

    while left_index <= middle_index {
        destination[dest_index] = data_set[left_index as usize];
        dest_index += 1;
        left_index += 1;
    }

    while right_index <= end_index {
        destination[dest_index] = data_set[right_index as usize];
        dest_index += 1;
        right_index += 1;
    }

    dest_index = 0;
    for i in start_index..=end_index {
        data_set[i as usize] = destination[dest_index];
        dest_index += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sorts_main_program_data() {
        let mut data = [334, 6, 4, 2, 3, 1, 5, 117, 12, 34];
        let len = data.len() as i32;
        merge_sort(&mut data, 0, len - 1);
        assert_eq!(data, [1, 2, 3, 4, 5, 6, 12, 34, 117, 334]);
    }

    #[test]
    fn already_sorted_stays_sorted() {
        let mut data = [1, 2, 3, 4, 5];
        merge_sort(&mut data, 0, 4);
        assert_eq!(data, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn single_element_unchanged() {
        let mut data = [42];
        merge_sort(&mut data, 0, 0);
        assert_eq!(data, [42]);
    }
}
