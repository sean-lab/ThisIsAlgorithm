fn merge_sort(data_set: &mut [i32], start_index: i32, end_index: i32) {
    if end_index - start_index < 1 {
        return;
    }

    let middle_index = (start_index + end_index) / 2;

    merge_sort(data_set, start_index, middle_index);
    merge_sort(data_set, middle_index + 1, end_index);

    merge(data_set, start_index, middle_index, end_index);
}

fn merge(data_set: &mut [i32], start_index: i32, middle_index: i32, end_index: i32) {
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

fn main() {
    let mut data_set = [334, 6, 4, 2, 3, 1, 5, 117, 12, 34];
    let length = data_set.len() as i32;

    merge_sort(&mut data_set, 0, length - 1);

    for i in 0..length {
        print!("{} ", data_set[i as usize]);
    }

    println!();
}
