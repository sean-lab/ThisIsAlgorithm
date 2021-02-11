#include "Graph.h"
#include "MST.h"

int main( void )
{
    
    /*  그래프 생성     */
    Graph* graph      = CreateGraph();
    Graph* PrimMST    = CreateGraph();
    Graph* KruskalMST = CreateGraph();
   
    /*  정점 생성 */
    Vertex* A = CreateVertex( 'A' );
    Vertex* B = CreateVertex( 'B' );
    Vertex* C = CreateVertex( 'C' );
    Vertex* D = CreateVertex( 'D' );
    Vertex* E = CreateVertex( 'E' );
    Vertex* F = CreateVertex( 'F' );
    Vertex* G = CreateVertex( 'G' );
    Vertex* H = CreateVertex( 'H' );
    Vertex* I = CreateVertex( 'I' );

    /*  그래프에 정점을 추가 */
    AddVertex( graph, A );
    AddVertex( graph, B );
    AddVertex( graph, C );
    AddVertex( graph, D );
    AddVertex( graph, E );
    AddVertex( graph, F );
    AddVertex( graph, G );
    AddVertex( graph, H );
    AddVertex( graph, I );

    /*  정점과 정점을 간선으로 잇기 */
    AddEdge( A, CreateEdge(A, B, 35) );
    AddEdge( A, CreateEdge(A, E, 247) );
    
    AddEdge( B, CreateEdge(B, A, 35  ) );
    AddEdge( B, CreateEdge(B, C, 126 ) );
    AddEdge( B, CreateEdge(B, F, 150 ) );

    AddEdge( C, CreateEdge(C, B, 126 ) );
    AddEdge( C, CreateEdge(C, D, 117 ) );
    AddEdge( C, CreateEdge(C, F, 162 ) );
    AddEdge( C, CreateEdge(C, G, 220 ) );
    
    AddEdge( D, CreateEdge(D, C, 117 ) );

    AddEdge( E, CreateEdge(E, A, 247 ) );
    AddEdge( E, CreateEdge(E, F, 82 ) );
    AddEdge( E, CreateEdge(E, H, 98 ) );

    AddEdge( F, CreateEdge(F, B, 150 ) );
    AddEdge( F, CreateEdge(F, C, 162 ) );
    AddEdge( F, CreateEdge(F, E, 82  ) );
    AddEdge( F, CreateEdge(F, G, 154 ) );
    AddEdge( F, CreateEdge(F, H, 120 ) );

    AddEdge( G, CreateEdge(G, C, 220 ) );
    AddEdge( G, CreateEdge(G, F, 154 ) );
    AddEdge( G, CreateEdge(G, I, 106 ) );

    AddEdge( H, CreateEdge(H, E, 98  ) );
    AddEdge( H, CreateEdge(H, F, 120 ) );

    AddEdge( I, CreateEdge(I, G, 106 ) );
    
    /*  정점 B를 시작 정점으로 하는 최소 신장 트리. */
    printf("Prim's Algorithm\n");
    Prim(graph, B, PrimMST);
    PrintGraph ( PrimMST );

    printf("Kruskal's Algorithm...\n");
    Kruskal(graph, KruskalMST);
    PrintGraph ( KruskalMST );
    
    /*  그래프 소멸 */
    DestroyGraph( PrimMST );
    DestroyGraph( KruskalMST );
    DestroyGraph( graph );

    return 0;
}
