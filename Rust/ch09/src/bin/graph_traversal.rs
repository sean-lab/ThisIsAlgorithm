// 그래프 순회 (DFS / BFS)
// Clang/09/GraphTraversal 포팅
use ch09::graph::*;
use std::collections::VecDeque;
use std::io::{self, Read, Write};

unsafe fn dfs(v: *mut Vertex) {
    print!("{} ", (*v).data);
    (*v).visited = VISITED;

    let mut e = (*v).adjacency_list;
    while !e.is_null() {
        if !(*e).target.is_null() && (*(*e).target).visited == NOT_VISITED {
            dfs((*e).target);
        }
        e = (*e).next;
    }
}

unsafe fn bfs(mut v: *mut Vertex, queue: &mut VecDeque<*mut Vertex>) {
    print!("{} ", (*v).data);
    (*v).visited = VISITED;

    queue.push_back(v);

    while let Some(popped) = queue.pop_front() {
        v = popped;
        let mut e = (*v).adjacency_list;

        while !e.is_null() {
            let t = (*e).target;
            if !t.is_null() && (*t).visited == NOT_VISITED {
                print!("{} ", (*t).data);
                (*t).visited = VISITED;
                queue.push_back(t);
            }
            e = (*e).next;
        }
    }
}

fn main() {
    unsafe {
        let graph = create_graph();

        let v1 = create_vertex(1);
        let v2 = create_vertex(2);
        let v3 = create_vertex(3);
        let v4 = create_vertex(4);
        let v5 = create_vertex(5);
        let v6 = create_vertex(6);
        let v7 = create_vertex(7);

        add_vertex(graph, v1);
        add_vertex(graph, v2);
        add_vertex(graph, v3);
        add_vertex(graph, v4);
        add_vertex(graph, v5);
        add_vertex(graph, v6);
        add_vertex(graph, v7);

        add_edge(v1, create_edge(v1, v2, 0));
        add_edge(v1, create_edge(v1, v3, 0));

        add_edge(v2, create_edge(v2, v4, 0));
        add_edge(v2, create_edge(v2, v5, 0));

        add_edge(v3, create_edge(v3, v4, 0));
        add_edge(v3, create_edge(v3, v6, 0));

        add_edge(v4, create_edge(v4, v5, 0));
        add_edge(v4, create_edge(v4, v7, 0));

        add_edge(v5, create_edge(v5, v7, 0));

        add_edge(v6, create_edge(v6, v7, 0));

        print!("Enter Traversal Mode (0:DFS, 1:BFS) : ");
        io::stdout().flush().ok();

        let mut input = String::new();
        io::stdin().read_to_string(&mut input).ok();
        let mode: i32 = input.split_whitespace().next().and_then(|s| s.parse().ok()).unwrap_or(0);

        if mode == 0 {
            dfs((*graph).vertices);
        } else {
            let mut queue: VecDeque<*mut Vertex> = VecDeque::new();
            bfs(v1, &mut queue);
        }

        destroy_graph(graph);
    }
}
