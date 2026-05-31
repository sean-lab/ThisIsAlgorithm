// 분리 집합 (Disjoint Set)
// Clang/04/DisjointSet 포팅. 로직은 ch04::disjoint_set 모듈에 있다.
use ch04::disjoint_set::{ds_destroy_set, ds_find_set, ds_make_set, ds_union_set};

fn main() {
    unsafe {
        let set1 = ds_make_set(1);
        let set2 = ds_make_set(2);
        let set3 = ds_make_set(3);
        let set4 = ds_make_set(4);

        println!(
            "Set1 == Set2 : {} ",
            (ds_find_set(set1) == ds_find_set(set2)) as i32
        );

        ds_union_set(set1, set3);
        println!(
            "Set1 == Set3 : {} ",
            (ds_find_set(set1) == ds_find_set(set3)) as i32
        );

        ds_union_set(set3, set4);
        println!(
            "Set3 == Set4 : {} ",
            (ds_find_set(set3) == ds_find_set(set4)) as i32
        );

        ds_destroy_set(set1);
        ds_destroy_set(set2);
        ds_destroy_set(set3);
        ds_destroy_set(set4);
    }
}
