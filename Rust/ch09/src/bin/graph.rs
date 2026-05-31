// 그래프 출력 예제
// Clang/09/Graph/Test_Graph.c 포팅
use ch09::graph::*;

fn main() {
    unsafe {
        let g = create_graph();

        let v1 = create_vertex('1' as i32);
        let v2 = create_vertex('2' as i32);
        let v3 = create_vertex('3' as i32);
        let v4 = create_vertex('4' as i32);
        let v5 = create_vertex('5' as i32);

        add_vertex(g, v1);
        add_vertex(g, v2);
        add_vertex(g, v3);
        add_vertex(g, v4);
        add_vertex(g, v5);

        add_edge(v1, create_edge(v1, v2, 0));
        add_edge(v1, create_edge(v1, v3, 0));
        add_edge(v1, create_edge(v1, v4, 0));
        add_edge(v1, create_edge(v1, v5, 0));

        add_edge(v2, create_edge(v2, v1, 0));
        add_edge(v2, create_edge(v2, v3, 0));
        add_edge(v2, create_edge(v2, v5, 0));

        add_edge(v3, create_edge(v3, v1, 0));
        add_edge(v3, create_edge(v3, v2, 0));

        add_edge(v4, create_edge(v4, v1, 0));
        add_edge(v4, create_edge(v4, v5, 0));

        add_edge(v5, create_edge(v5, v1, 0));
        add_edge(v5, create_edge(v5, v2, 0));
        add_edge(v5, create_edge(v5, v4, 0));

        print_graph(g);

        destroy_graph(g);
    }
}
