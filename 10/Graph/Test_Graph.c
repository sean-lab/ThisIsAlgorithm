#include "Graph.h"

int main( void )
{
    //  그래프 생성     
    Graph* G = CreateGraph();
    
    //  정점 생성 
    Vertex* V1 = CreateVertex( '1' );
    Vertex* V2 = CreateVertex( '2' );
    Vertex* V3 = CreateVertex( '3' );
    Vertex* V4 = CreateVertex( '4' );
    Vertex* V5 = CreateVertex( '5' );

    //  그래프에 정점을 추가 
    AddVertex( G, V1 );
    AddVertex( G, V2 );
    AddVertex( G, V3 );
    AddVertex( G, V4 );
    AddVertex( G, V5 );

    //  정점과 정점을 간선으로 잇기 
    AddEdge( V1, CreateEdge(V1, V2, 0) );
    AddEdge( V1, CreateEdge(V1, V3, 0) );
    AddEdge( V1, CreateEdge(V1, V4, 0) );
    AddEdge( V1, CreateEdge(V1, V5, 0) );

    AddEdge( V2, CreateEdge(V2, V1, 0) );
    AddEdge( V2, CreateEdge(V2, V3, 0) );
    AddEdge( V2, CreateEdge(V2, V5, 0) );

    AddEdge( V3, CreateEdge(V3, V1, 0) );
    AddEdge( V3, CreateEdge(V3, V2, 0) );

    AddEdge( V4, CreateEdge(V4, V1, 0) );
    AddEdge( V4, CreateEdge(V4, V5, 0) );

    AddEdge( V5, CreateEdge(V5, V1, 0) );
    AddEdge( V5, CreateEdge(V5, V2, 0) );
    AddEdge( V5, CreateEdge(V5, V4, 0) );

    PrintGraph( G );

    //  그래프 소멸 
    DestroyGraph( G );

    return 0;
}
