#include "TopologicalSort.h"

void TopologicalSort( Vertex* V, Node** List )
{
    while ( V != NULL && V->Visited == NotVisited )
    {
        TS_DFS( V, List );

        V = V->Next;
    }
}

void TS_DFS( Vertex* V, Node** List )
{
    Node* NewHead = NULL;
    Edge* E = NULL;

    V->Visited = Visited;

    E = V->AdjacencyList;

    while ( E != NULL )
    {
        if ( E->Target != NULL && E->Target->Visited == NotVisited )
            TS_DFS( E->Target, List );

        E = E->Next;
    }

    printf("%c\n", V->Data );

    NewHead = SLL_CreateNode( V );
    SLL_InsertNewHead( List, NewHead );
}
