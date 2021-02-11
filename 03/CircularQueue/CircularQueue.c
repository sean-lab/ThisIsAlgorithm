#include "CircularQueue.h"

void  CQ_CreateQueue( CircularQueue** Queue, int Capacity)
{
    /*  큐를 자유저장소에 생성 */
    (*Queue )           = ( CircularQueue*)malloc(sizeof( CircularQueue ));

    /*  입력된 Capacity+1 만큼의 노드를 자유저장소에 생성 */
    (*Queue )->Nodes    = (Node*)malloc(sizeof(Node )* ( Capacity+1) );

    (*Queue )->Capacity = Capacity;
    (*Queue )->Front = 0;
    (*Queue )->Rear  = 0;
}

void CQ_DestroyQueue( CircularQueue* Queue )
{
    free(Queue->Nodes);
    free(Queue );
}

void CQ_Enqueue( CircularQueue* Queue, ElementType Data)
{
    int Position=0;
  
    if(Queue->Rear==Queue->Capacity)
    {
        Position=Queue->Rear;
        Queue->Rear=0;
    }
    else
        Position=Queue->Rear++;
  
    Queue->Nodes[Position].Data=Data;
}

ElementType CQ_Dequeue( CircularQueue* Queue )
{
    int Position = Queue->Front;

    if ( Queue->Front == Queue->Capacity )
        Queue->Front = 0;
    else
        Queue->Front++;

    return Queue->Nodes[Position].Data;
}

int CQ_GetSize( CircularQueue* Queue )
{
    if ( Queue->Front <= Queue->Rear )
        return Queue->Rear - Queue->Front;
    else
        return Queue->Rear + (Queue->Capacity - Queue->Front) + 1;
}

int CQ_IsEmpty( CircularQueue* Queue )
{
    return (Queue->Front == Queue->Rear);
}

int CQ_IsFull( CircularQueue* Queue )
{
    if ( Queue->Front < Queue->Rear )
        return ( Queue->Rear - Queue->Front) == Queue->Capacity;
    else
        return ( Queue->Rear + 1 ) == Queue->Front;
}
