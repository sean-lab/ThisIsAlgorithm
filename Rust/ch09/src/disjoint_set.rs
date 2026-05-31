// 서로소 집합 (Disjoint Set) - raw pointer 기반
use std::ffi::c_void;
use std::ptr;

pub struct DisjointSet {
    pub parent: *mut DisjointSet,
    pub data: *mut c_void,
}

pub unsafe fn ds_union_set(set1: *mut DisjointSet, set2: *mut DisjointSet) {
    let set2 = ds_find_set(set2);
    (*set2).parent = set1;
}

pub unsafe fn ds_find_set(mut set: *mut DisjointSet) -> *mut DisjointSet {
    while !(*set).parent.is_null() {
        set = (*set).parent;
    }
    set
}

pub unsafe fn ds_make_set(new_data: *mut c_void) -> *mut DisjointSet {
    Box::into_raw(Box::new(DisjointSet {
        parent: ptr::null_mut(),
        data: new_data,
    }))
}

pub unsafe fn ds_destroy_set(set: *mut DisjointSet) {
    drop(Box::from_raw(set));
}
