// 삽입 정렬 (Insertion Sort)
// Clang/05/InsertionSort 포팅. memmove 부분은 Vec 슬라이스 이동으로 옮겼다.

pub fn insertion_sort(data_set: &mut [i32]) {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sorts_sample() {
        let mut d = [6, 4, 2, 3, 1, 5];
        insertion_sort(&mut d);
        assert_eq!(d, [1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn handles_edge_cases() {
        let mut empty: [i32; 0] = [];
        insertion_sort(&mut empty);
        assert_eq!(empty, [] as [i32; 0]);

        let mut single = [42];
        insertion_sort(&mut single);
        assert_eq!(single, [42]);

        let mut rev = [5, 4, 3, 2, 1];
        insertion_sort(&mut rev);
        assert_eq!(rev, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn handles_duplicates_and_negatives() {
        let mut d = [3, -1, 3, 0, -1, 2];
        insertion_sort(&mut d);
        assert_eq!(d, [-1, -1, 0, 2, 3, 3]);
    }
}
