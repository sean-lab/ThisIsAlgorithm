// 다익스트라 최단 경로 (Dijkstra)
// Clang/09/Dijkstra 포팅
use ch09::graph::*;
use ch09::pq::*;
use std::ffi::c_void;
use std::ptr;

const MAX_WEIGHT: i32 = 36267;

unsafe fn dijkstra(g: *mut Graph, start_vertex: *mut Vertex, shortest_path: *mut Graph) {
    let mut i: usize = 0;

    let start_node = PQNode {
        priority: 0,
        data: start_vertex as *mut c_void,
    };
    let mut pq = PriorityQueue::create(10);

    let vertex_count = (*g).vertex_count as usize;

    let mut weights = vec![0i32; vertex_count];
    let mut shortest_path_vertices: Vec<*mut Vertex> = vec![ptr::null_mut(); vertex_count];
    let mut fringes: Vec<*mut Vertex> = vec![ptr::null_mut(); vertex_count];
    let mut precedences: Vec<*mut Vertex> = vec![ptr::null_mut(); vertex_count];

    let mut current_vertex = (*g).vertices;
    while !current_vertex.is_null() {
        let new_vertex = create_vertex((*current_vertex).data);
        add_vertex(shortest_path, new_vertex);

        fringes[i] = ptr::null_mut();
        precedences[i] = ptr::null_mut();
        shortest_path_vertices[i] = new_vertex;
        weights[i] = MAX_WEIGHT;
        current_vertex = (*current_vertex).next;
        i += 1;
    }

    pq.enqueue(start_node);

    weights[(*start_vertex).index as usize] = 0;

    while !pq.is_empty() {
        let mut popped = PQNode::default();
        pq.dequeue(&mut popped);
        let current_vertex = popped.data as *mut Vertex;

        fringes[(*current_vertex).index as usize] = current_vertex;

        let mut current_edge = (*current_vertex).adjacency_list;
        while !current_edge.is_null() {
            let target_vertex = (*current_edge).target;

            if fringes[(*target_vertex).index as usize].is_null()
                && weights[(*current_vertex).index as usize] + (*current_edge).weight
                    < weights[(*target_vertex).index as usize]
            {
                let new_node = PQNode {
                    priority: (*current_edge).weight,
                    data: target_vertex as *mut c_void,
                };
                pq.enqueue(new_node);

                precedences[(*target_vertex).index as usize] = (*current_edge).from;
                weights[(*target_vertex).index as usize] =
                    weights[(*current_vertex).index as usize] + (*current_edge).weight;
            }

            current_edge = (*current_edge).next;
        }
    }

    for i in 0..vertex_count {
        if precedences[i].is_null() {
            continue;
        }

        let from_index = (*precedences[i]).index as usize;
        let to_index = i;

        add_edge(
            shortest_path_vertices[from_index],
            create_edge(
                shortest_path_vertices[from_index],
                shortest_path_vertices[to_index],
                weights[i],
            ),
        );
    }
}

fn main() {
    unsafe {
        let graph = create_graph();
        let prim_mst = create_graph();
        let kruskal_mst = create_graph();

        let a = create_vertex('A' as i32);
        let b = create_vertex('B' as i32);
        let c = create_vertex('C' as i32);
        let d = create_vertex('D' as i32);
        let e = create_vertex('E' as i32);
        let f = create_vertex('F' as i32);
        let gg = create_vertex('G' as i32);
        let h = create_vertex('H' as i32);
        let ii = create_vertex('I' as i32);

        add_vertex(graph, a);
        add_vertex(graph, b);
        add_vertex(graph, c);
        add_vertex(graph, d);
        add_vertex(graph, e);
        add_vertex(graph, f);
        add_vertex(graph, gg);
        add_vertex(graph, h);
        add_vertex(graph, ii);

        add_edge(a, create_edge(a, e, 247));

        add_edge(b, create_edge(b, a, 35));
        add_edge(b, create_edge(b, c, 126));
        add_edge(b, create_edge(b, f, 150));

        add_edge(c, create_edge(c, d, 117));
        add_edge(c, create_edge(c, f, 162));
        add_edge(c, create_edge(c, gg, 220));

        add_edge(e, create_edge(e, h, 98));

        add_edge(f, create_edge(f, e, 82));
        add_edge(f, create_edge(f, gg, 154));
        add_edge(f, create_edge(f, h, 120));

        add_edge(gg, create_edge(gg, ii, 106));

        dijkstra(graph, b, prim_mst);
        print_graph(prim_mst);

        destroy_graph(prim_mst);
        destroy_graph(kruskal_mst);
        destroy_graph(graph);
    }
}
