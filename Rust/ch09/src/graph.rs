// 그래프 (Graph) - 인접 리스트, raw pointer 기반
use std::ptr;

pub type VElementType = i32;

// enum VisitMode { Visited, NotVisited };
pub const VISITED: i32 = 0;
pub const NOT_VISITED: i32 = 1;

pub struct Vertex {
    pub data: VElementType,
    pub visited: i32,
    pub index: i32,
    pub next: *mut Vertex,
    pub adjacency_list: *mut Edge,
}

pub struct Edge {
    pub weight: i32,
    pub next: *mut Edge,
    pub from: *mut Vertex,
    pub target: *mut Vertex,
}

pub struct Graph {
    pub vertices: *mut Vertex,
    pub vertex_count: i32,
}

pub unsafe fn create_graph() -> *mut Graph {
    Box::into_raw(Box::new(Graph {
        vertices: ptr::null_mut(),
        vertex_count: 0,
    }))
}

pub unsafe fn destroy_graph(g: *mut Graph) {
    while !(*g).vertices.is_null() {
        let vertices = (*(*g).vertices).next;
        destroy_vertex((*g).vertices);
        (*g).vertices = vertices;
    }
    drop(Box::from_raw(g));
}

pub unsafe fn create_vertex(data: VElementType) -> *mut Vertex {
    Box::into_raw(Box::new(Vertex {
        data,
        visited: NOT_VISITED,
        index: -1,
        next: ptr::null_mut(),
        adjacency_list: ptr::null_mut(),
    }))
}

pub unsafe fn destroy_vertex(v: *mut Vertex) {
    while !(*v).adjacency_list.is_null() {
        let edge = (*(*v).adjacency_list).next;
        destroy_edge((*v).adjacency_list);
        (*v).adjacency_list = edge;
    }
    drop(Box::from_raw(v));
}

pub unsafe fn create_edge(from: *mut Vertex, target: *mut Vertex, weight: i32) -> *mut Edge {
    Box::into_raw(Box::new(Edge {
        weight,
        next: ptr::null_mut(),
        from,
        target,
    }))
}

pub unsafe fn destroy_edge(e: *mut Edge) {
    drop(Box::from_raw(e));
}

pub unsafe fn add_vertex(g: *mut Graph, v: *mut Vertex) {
    let mut vertex_list = (*g).vertices;

    if vertex_list.is_null() {
        (*g).vertices = v;
    } else {
        while !(*vertex_list).next.is_null() {
            vertex_list = (*vertex_list).next;
        }
        (*vertex_list).next = v;
    }

    (*v).index = (*g).vertex_count;
    (*g).vertex_count += 1;
}

pub unsafe fn add_edge(v: *mut Vertex, e: *mut Edge) {
    if (*v).adjacency_list.is_null() {
        (*v).adjacency_list = e;
    } else {
        let mut adjacency_list = (*v).adjacency_list;
        while !(*adjacency_list).next.is_null() {
            adjacency_list = (*adjacency_list).next;
        }
        (*adjacency_list).next = e;
    }
}

pub unsafe fn print_graph(g: *mut Graph) {
    let mut v = (*g).vertices;
    if v.is_null() {
        return;
    }

    while !v.is_null() {
        print!("{} : ", (*v).data as u8 as char);

        let mut e = (*v).adjacency_list;
        if e.is_null() {
            v = (*v).next;
            println!();
            continue;
        }

        while !e.is_null() {
            print!("{}[{}] ", (*(*e).target).data as u8 as char, (*e).weight);
            e = (*e).next;
        }

        println!();
        v = (*v).next;
    }

    println!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_vertex_assigns_index_and_counts() {
        unsafe {
            let g = create_graph();
            let a = create_vertex(b'A' as i32);
            let b = create_vertex(b'B' as i32);
            let c = create_vertex(b'C' as i32);

            add_vertex(g, a);
            add_vertex(g, b);
            add_vertex(g, c);

            assert_eq!((*g).vertex_count, 3);
            assert_eq!((*a).index, 0);
            assert_eq!((*b).index, 1);
            assert_eq!((*c).index, 2);

            destroy_graph(g);
        }
    }

    #[test]
    fn add_edge_builds_adjacency_list() {
        unsafe {
            let g = create_graph();
            let a = create_vertex(b'A' as i32);
            let b = create_vertex(b'B' as i32);
            let c = create_vertex(b'C' as i32);
            add_vertex(g, a);
            add_vertex(g, b);
            add_vertex(g, c);

            add_edge(a, create_edge(a, b, 5));
            add_edge(a, create_edge(a, c, 7));

            // A 의 인접 리스트: B[5] -> C[7]
            let e1 = (*a).adjacency_list;
            assert!(!e1.is_null());
            assert_eq!((*(*e1).target).data, b'B' as i32);
            assert_eq!((*e1).weight, 5);

            let e2 = (*e1).next;
            assert!(!e2.is_null());
            assert_eq!((*(*e2).target).data, b'C' as i32);
            assert_eq!((*e2).weight, 7);
            assert!((*e2).next.is_null());

            destroy_graph(g);
        }
    }
}
