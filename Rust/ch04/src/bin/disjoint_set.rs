// 분리 집합 (Disjoint Set)
// Clang/04/DisjointSet 포팅. 노드의 포인터 동일성(==)을 사용하므로 raw pointer로 옮겼다.
use std::ptr;

struct DisjointSet {
    parent: *mut DisjointSet,
    // C 원본의 void* Data 를 옮긴 필드. 테스트에서 직접 읽지는 않는다.
    #[allow(dead_code)]
    data: i32,
}

unsafe fn ds_make_set(new_data: i32) -> *mut DisjointSet {
    Box::into_raw(Box::new(DisjointSet {
        parent: ptr::null_mut(),
        data: new_data,
    }))
}

unsafe fn ds_find_set(mut set: *mut DisjointSet) -> *mut DisjointSet {
    while !(*set).parent.is_null() {
        set = (*set).parent;
    }
    set
}

unsafe fn ds_union_set(set1: *mut DisjointSet, set2: *mut DisjointSet) {
    let root2 = ds_find_set(set2);
    (*root2).parent = set1;
}

unsafe fn ds_destroy_set(set: *mut DisjointSet) {
    drop(Box::from_raw(set));
}

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
