#include "NQueens.h"

#include <stdio.h>
#include <stdlib.h>

void FindSolutionForQueen(int Columns[], int Row, 
                          int NumberOfQueens, int* SolutionCount) 
{
    if ( IsThreatened( Columns, Row ) )
        return;
    
    if ( Row == NumberOfQueens-1 )
    {
        printf("Solution #%d : \n", ++(*SolutionCount));
        PrintSolution( Columns, NumberOfQueens );
    }
    else
    {
        int i;

        for ( i=0; i<NumberOfQueens; i++ )
        {
            Columns[Row+1] = i;
            FindSolutionForQueen ( Columns, Row+1, 
                                   NumberOfQueens, SolutionCount );
        }
    }
}

int IsThreatened ( int Columns[], int NewRow )
{
    int CurrentRow = 0;
    int Threatened = 0;

    while ( CurrentRow < NewRow )
    { 
        if ( Columns[NewRow] == Columns[CurrentRow]
             || abs (Columns[NewRow] - Columns[CurrentRow]) 
               == abs(NewRow - CurrentRow))        
        {
            Threatened = 1;
            break;
        }

       CurrentRow++;
    }

    return Threatened;
}

void PrintSolution( int Columns[], int NumberOfQueens ) 
{
    int i=0;
    int j=0;

    for ( i=0; i<NumberOfQueens; i++ )
    {
        for ( j=0; j<NumberOfQueens; j++ )
        {
            if ( Columns[i] == j )
                printf("Q");
            else
                printf(".");
        }

        printf("\n");
    }
    printf("\n");
}
