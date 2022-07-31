#include "ShortestPath.h"

void Dijkstra(Graph* G, Vertex* StartVertex, Graph* ShortestPath )
{
    int i = 0;

    PQNode         StartNode = { 0, StartVertex };
    PriorityQueue* PQ        = PQ_Create(10);

    Vertex*  CurrentVertex = NULL;
    Edge*    CurrentEdge   = NULL; 

    int*     Weights       = (int*) malloc( sizeof(int) * G->VertexCount );
    Vertex** ShortestPathVertices = 
                             (Vertex**) malloc( sizeof(Vertex*) * G->VertexCount );
    Vertex** Fringes       = (Vertex**) malloc( sizeof(Vertex*) * G->VertexCount );
    Vertex** Precedences   = (Vertex**) malloc( sizeof(Vertex*) * G->VertexCount );

    CurrentVertex = G->Vertices;
    while ( CurrentVertex != NULL )
    {
        Vertex* NewVertex = CreateVertex( CurrentVertex->Data );
        AddVertex( ShortestPath, NewVertex);

        Fringes[i]     = NULL;
        Precedences[i] = NULL;
        ShortestPathVertices[i] = NewVertex;        
        Weights[i]     = MAX_WEIGHT;        
        CurrentVertex  = CurrentVertex->Next;
        i++;
    }

    PQ_Enqueue ( PQ, StartNode );

    Weights[StartVertex->Index] = 0;
    
    while( ! PQ_IsEmpty( PQ ) )
    {
        PQNode  Popped;
        
        PQ_Dequeue(PQ, &Popped);
        CurrentVertex = (Vertex*)Popped.Data;
        
        Fringes[CurrentVertex->Index] = CurrentVertex;

        CurrentEdge = CurrentVertex->AdjacencyList;
        while ( CurrentEdge != NULL )
        {            
            Vertex* TargetVertex = CurrentEdge->Target;

            if ( Fringes[TargetVertex->Index] == NULL &&
                 Weights[CurrentVertex->Index] + CurrentEdge->Weight < 
                               Weights[TargetVertex->Index] )
            {
                PQNode NewNode =  { CurrentEdge->Weight, TargetVertex };
                PQ_Enqueue ( PQ, NewNode );

                Precedences[TargetVertex->Index] =  CurrentEdge->From;
                Weights[TargetVertex->Index]     =  
                               Weights[CurrentVertex->Index] + CurrentEdge->Weight; 
            }
            
            CurrentEdge = CurrentEdge->Next;
        }
    }

    for ( i=0; i<G->VertexCount; i++ )
    {
        int FromIndex, ToIndex;

        if ( Precedences[i] == NULL )
            continue;

        FromIndex = Precedences[i]->Index;
        ToIndex   = i;

        AddEdge( ShortestPathVertices[FromIndex], 
            CreateEdge( 
                ShortestPathVertices[FromIndex], 
                ShortestPathVertices[ToIndex],   
                Weights[i] ) );
    }

    free( Fringes );
    free( Precedences );
    free( ShortestPathVertices );
    free( Weights );

    PQ_Destroy( PQ );
}
