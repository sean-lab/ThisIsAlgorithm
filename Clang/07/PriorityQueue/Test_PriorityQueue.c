#include "PriorityQueue.h"

void PrintNode( PQNode* Node )
{
    printf("작업명 : %s (우선순위:%d)\n", Node->Data, Node->Priority);
}

int main( void )
{
    PriorityQueue* PQ = PQ_Create(  3 );
    PQNode Popped;

    PQNode Nodes[7] = 
    {
        {34, (void*)"코딩"},
        {12, (void*)"고객미팅"},
        {87, (void*)"커피타기"},        
        {45, (void*)"문서작성"},
        {35, (void*)"디버깅"},
        {66, (void*)"이닦기"}
    };
    
    PQ_Enqueue(PQ, Nodes[0] );
    PQ_Enqueue(PQ, Nodes[1] );
    PQ_Enqueue(PQ, Nodes[2] );
    PQ_Enqueue(PQ, Nodes[3] );
    PQ_Enqueue(PQ, Nodes[4] );
    PQ_Enqueue(PQ, Nodes[5]);

    printf( "큐에 남아 있는 작업의 수 : %d\n", PQ->UsedSize );
    
    while ( !PQ_IsEmpty ( PQ ) )
    {
        PQ_Dequeue( PQ, &Popped );
        PrintNode( &Popped );
    }
    
    return 0;
}
