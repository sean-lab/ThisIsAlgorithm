#include "Heap.h"

int main( void )
{
    Heap* H = HEAP_Create( 3 );
    HeapNode MinNode;
    
    HEAP_Insert( H, 12 );
    HEAP_Insert( H, 87 );
    HEAP_Insert( H, 111 );
    HEAP_Insert( H, 34 );
    HEAP_Insert( H, 16 );
    HEAP_Insert( H, 75 );
    HEAP_PrintNodes( H );
    
    HEAP_DeleteMin( H, &MinNode );
    HEAP_PrintNodes( H );

    HEAP_DeleteMin( H, &MinNode );
    HEAP_PrintNodes( H );

    HEAP_DeleteMin( H, &MinNode );
    HEAP_PrintNodes( H );

    HEAP_DeleteMin( H, &MinNode );
    HEAP_PrintNodes( H );

    HEAP_DeleteMin( H, &MinNode );
    HEAP_PrintNodes( H );

    HEAP_DeleteMin( H, &MinNode );
    HEAP_PrintNodes( H );

    return 0;
}
