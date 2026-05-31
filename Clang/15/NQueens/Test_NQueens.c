#include <stdio.h>
#include <stdlib.h>
#include "NQueens.h"

int main(int argc, char* argv[]) 
{
    int i              = 0;
    int NumberOfQueens = 0; 
    int SolutionCount  = 0;
    int* Columns;

    if ( argc < 2 )
    {
        printf("Usage: %s <Number Of Queens>", argv[0]);
        return 1;
    }
    
    NumberOfQueens = atoi(argv[1]); 
    Columns = (int*)calloc( NumberOfQueens, sizeof(int) );
        
    for ( i=0; i<NumberOfQueens; i++ )
    {
        Columns[0] = i;
        FindSolutionForQueen(Columns, 0, NumberOfQueens, &SolutionCount);
    }    

    free ( Columns );

    return 0;
}
