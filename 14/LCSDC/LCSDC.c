#include <stdio.h>
#include <string.h>
#include <stdlib.h>

typedef struct structLCSTable
{
    int** Data;
} LCSTable;

int LCS( char* X, char* Y, int i, int j, LCSTable* Table )
{
    if ( i == 0 || j == 0 )
    {
        Table->Data[i][j] = 0;
        return Table->Data[i][j];
    }
    else if ( X[i-1] == Y[j-1] )
    {
        Table->Data[i][j] = LCS( X, Y, i-1, j-1, Table ) + 1;
        return Table->Data[i][j];
    }
    else 
    {
        int A = LCS( X, Y, i-1, j  , Table );
        int B = LCS( X, Y, i  , j-1, Table );

        if ( A > B )
            Table->Data[i][j] = A;
        else
            Table->Data[i][j] = B;

        return Table->Data[i][j];
    }
}

void LCS_PrintTable( LCSTable* Table, char* X, char* Y, int LEN_X, int LEN_Y )
{
    int i = 0;
    int j = 0;

    printf("%-04s", " ");

    for ( i=0; i<LEN_Y+1; i++ )
       printf("%c ", Y[i]);
    printf("\n");

    for ( i=0; i<LEN_X+1; i++ )
    {   
        printf("%c ", X[i-1]);

        for ( j=0; j<LEN_Y+1; j++ )
        {
            printf("%d ", Table->Data[i][j]);
        }

        printf("\n");
    }
}

int main( void )
{
    char* X = "GOOD MORNING.";
    char* Y = "GUTEN MORGEN.";

    int   LEN_X = strlen(X);
    int   LEN_Y = strlen(Y);

    int   i = 0;
    int   j = 0;
    int   LCSLength = 0;

    LCSTable Table;
    
    Table.Data = (int**)malloc( sizeof(int*) * (LEN_X + 1) );

    for ( i=0; i<LEN_X+1; i++ ) {
        Table.Data[i] = (int*)malloc( sizeof(int) * (LEN_Y + 1) );
        memset( Table.Data[i], 0, sizeof( int) * (LEN_Y + 1 ) );
    }

    LCSLength = LCS(X, Y, LEN_X, LEN_Y, &Table );

    LCS_PrintTable( &Table, X, Y, LEN_X, LEN_Y );

    return 0;
}