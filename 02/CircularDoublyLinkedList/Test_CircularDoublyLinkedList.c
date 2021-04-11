#include "CircularDoublyLinkedList.h"

int main( void )
{
    int   i       = 0;
    int   Count   = 0;
    Node* List    = NULL;
    Node* NewNode = NULL;
    Node* Current = NULL;

    //  노드 5개 추가 
    for ( i = 0; i<5; i++ )
    {
        NewNode = CDLL_CreateNode( i );
        CDLL_AppendNode( &List,NewNode );
    }

    //  리스트 출력 
    Count = CDLL_GetNodeCount( List );
    for ( i = 0; i<Count; i++ )
    {
        Current = CDLL_GetNodeAt( List, i );
        printf( "List[%d] : %d\n", i, Current->Data );
    }

    //  리스트의 세번째 칸 뒤에 노드 삽입 
    printf( "\nInserting 3000 After [2]...\n\n" );

    Current = CDLL_GetNodeAt( List, 2 );
    NewNode = CDLL_CreateNode( 3000 );
    CDLL_InsertAfter( Current, NewNode );

    printf( "\nRemoving Node at 2...\n" );
    Current = CDLL_GetNodeAt( List, 2 );
    CDLL_RemoveNode( &List, Current );
    CDLL_DestroyNode( Current );

    //  리스트 출력  
    //  (노드 수의 2배만큼 루프를 돌며 환형임을 확인한다.) 
    Count = CDLL_GetNodeCount( List );
    for ( i = 0; i<Count*2; i++ )
    {
        if ( i == 0 )
            Current = List;
        else
            Current = Current->NextNode;
        
        printf( "List[%d] : %d\n", i, Current->Data );
    }

    //  모든 노드를 메모리에서 제거     
    printf( "\nDestroying List...\n" );

    Count = CDLL_GetNodeCount( List );

    for ( i = 0; i<Count; i++ )
    {
        Current = CDLL_GetNodeAt( List, 0 );

        if ( Current != NULL ) 
        {
            CDLL_RemoveNode( &List, Current );
            CDLL_DestroyNode( Current );
        }
    }

    return 0;
}
