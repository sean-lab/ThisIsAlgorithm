#include "LinkedQueue.h"

void  LQ_CreateQueue( LinkedQueue** Queue )
{
    /*  큐를 자유저장소에 생성 */
    (*Queue )        = ( LinkedQueue*)malloc(sizeof( LinkedQueue ) );
    (*Queue )->Front = NULL;
    (*Queue )->Rear  = NULL;
    (*Queue )->Count = 0;
}

void LQ_DestroyQueue( LinkedQueue* Queue )
{
    while ( !LQ_IsEmpty( Queue ) )
    {
        Node* Popped = LQ_Dequeue( Queue );
        LQ_DestroyNode(Popped);    
    }

    /*  큐를 자유 저장소에서 해제 */
    free( Queue );
}

Node* LQ_CreateNode( char* NewData )
{
    Node* NewNode = (Node*)malloc( sizeof( Node ) );
    NewNode->Data = (char*)malloc( strlen( NewData) + 1);

    strcpy(NewNode->Data, NewData);  /*  데이터를 저장한다. */

    NewNode->NextNode = NULL; /*  다음 노드에 대한 포인터는 NULL로 초기화 한다. */

    return NewNode;/*  노드의 주소를 반환한다. */
}

void  LQ_DestroyNode(Node* _Node )
{
    free(_Node->Data);
    free(_Node );
}

void LQ_Enqueue( LinkedQueue* Queue, Node* NewNode )
{
    if ( Queue->Front == NULL ) 
    {        
        Queue->Front = NewNode;
        Queue->Rear  = NewNode;
        Queue->Count++;
    } 
    else
    {
        Queue->Rear->NextNode = NewNode;
        Queue->Rear = NewNode;
        Queue->Count++;
    }
}

Node* LQ_Dequeue( LinkedQueue* Queue )
{
    /*  LQ_Dequeue() 함수가 반환할 최상위 노드 */
    Node* Front = Queue->Front;

    if ( Queue->Front->NextNode == NULL )
    {
        Queue->Front = NULL;
        Queue->Rear  = NULL;
    }
    else
    {
        Queue->Front = Queue->Front->NextNode;
    }

    Queue->Count--;

    return Front;
}

int LQ_IsEmpty( LinkedQueue* Queue )
{
    return ( Queue->Front == NULL);
}