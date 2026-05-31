// 위상 정렬 (Topological Sort)
// Clang/09/TopologicalSort 포팅
use ch09::graph::*;
use ch09::linked_list::*;
use std::ptr;

unsafe fn topological_sort(mut v: *mut Vertex, list: &mut *mut Node) {
    while !v.is_null() && (*v).visited == NOT_VISITED {
        ts_dfs(v, list);
        v = (*v).next;
    }
}

unsafe fn ts_dfs(v: *mut Vertex, list: &mut *mut Node) {
    (*v).visited = VISITED;

    let mut e = (*v).adjacency_list;
    while !e.is_null() {
        if !(*e).target.is_null() && (*(*e).target).visited == NOT_VISITED {
            ts_dfs((*e).target, list);
        }
        e = (*e).next;
    }

    println!("{}", (*v).data as u8 as char);

    let new_head = sll_create_node(v);
    sll_insert_new_head(list, new_head);
}

fn main() {
    unsafe {
        let mut sorted_list: *mut Node = ptr::null_mut();

        let graph = create_graph();

        let a = create_vertex('A' as i32);
        let b = create_vertex('B' as i32);
        let c = create_vertex('C' as i32);
        let d = create_vertex('D' as i32);
        let e = create_vertex('E' as i32);
        let f = create_vertex('F' as i32);
        let gg = create_vertex('G' as i32);
        let h = create_vertex('H' as i32);

        add_vertex(graph, a);
        add_vertex(graph, b);
        add_vertex(graph, c);
        add_vertex(graph, d);
        add_vertex(graph, e);
        add_vertex(graph, f);
        add_vertex(graph, gg);
        add_vertex(graph, h);

        add_edge(a, create_edge(a, c, 0));
        add_edge(a, create_edge(a, d, 0));

        add_edge(b, create_edge(b, c, 0));
        add_edge(b, create_edge(b, e, 0));

        add_edge(c, create_edge(c, f, 0));

        add_edge(d, create_edge(d, f, 0));
        add_edge(d, create_edge(d, gg, 0));

        add_edge(e, create_edge(e, gg, 0));

        add_edge(f, create_edge(f, h, 0));

        add_edge(gg, create_edge(gg, h, 0));

        topological_sort((*graph).vertices, &mut sorted_list);

        print!("Topological Sort Result : ");

        let mut current_node = sorted_list;
        while !current_node.is_null() {
            print!("{} ", (*(*current_node).data).data as u8 as char);
            current_node = (*current_node).next_node;
        }
        println!();

        destroy_graph(graph);
    }
}
