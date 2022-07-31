#include <stdio.h>
#include <string.h>
#include <stdlib.h>

typedef struct structLCSTable
{
    int** Data;
} LCSTable;

int LCS( char* X, char* Y, int i, int j, LCSTable* Table )
{
    int m = 0;
    int n = 0;

    for ( m=0; m<=i; m++ )
        Table->Data[m][0] = 0;

    for ( n=0; n<=j; n++ )
        Table->Data[0][n] = 0;
    
    for ( m=1; m<=i; m++ ) 
    {
        for ( n=1; n<=j; n++ )
        {
            if ( X[m-1] == Y[n-1] )
                Table->Data[m][n] = Table->Data[m-1][n-1] + 1;
            else
            {
                if ( Table->Data[m][n-1] >= Table->Data[m-1][n] )
                    Table->Data[m][n] = Table->Data[m][n-1];
                else
                    Table->Data[m][n] = Table->Data[m-1][n];
            }
        }
    }
    
    return Table->Data[i][j];
}

void LCS_TraceBack( char* X, char* Y, int m, int n, LCSTable* Table, char* LCS)
{
    if ( m == 0 || n == 0 )
        return;
    
    if ( Table->Data[m][n] > Table->Data[m][n-1] 
         && Table->Data[m][n] > Table->Data[m-1][n]
         && Table->Data[m][n] > Table->Data[m-1][n-1] )
    {
        char TempLCS[100];
        strcpy( TempLCS, LCS );
        sprintf(LCS, "%c%s", X[m-1], TempLCS );

        LCS_TraceBack( X, Y, m-1, n-1, Table, LCS );
    }
    else if ( Table->Data[m][n] > Table->Data[m-1][n] 
              && Table->Data[m][n] == Table->Data[m][n-1])
    {
        LCS_TraceBack( X, Y, m, n-1, Table, LCS );
    }
    else
    {
        LCS_TraceBack( X, Y, m-1, n, Table, LCS );
    }
}

void LCS_PrintTable( LCSTable* Table, char* X, char* Y, int LEN_X, int LEN_Y )
{
    int i = 0;
    int j = 0;

    printf("%4s", "");

    for ( i=0; i<LEN_Y+1; i++ )
       printf("%c ", Y[i]);
    printf("\n");

    for ( i=0; i<LEN_X+1; i++ )
    {   

        if ( i == 0 )
            printf( "%2s", "");
        else
            printf("%-2c", X[i-1]);

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
    char* Result;

    int   LEN_X = strlen(X);
    int   LEN_Y = strlen(Y);

    int   i = 0;
    int   Length = 0;

    LCSTable Table;
    
    Table.Data = (int**)malloc( sizeof(int*) * (LEN_X + 1) );

    for ( i=0; i<LEN_X+1; i++ ) {
        Table.Data[i] = (int*)malloc( sizeof(int) * (LEN_Y + 1) );
        
        memset( Table.Data[i], 0, sizeof( int) * (LEN_Y + 1 ) );
    }

    Length = LCS(X, Y, LEN_X, LEN_Y, &Table );
    
    LCS_PrintTable( &Table, X, Y, LEN_X, LEN_Y );

    size_t TableSize = sizeof( Table.Data[LEN_X][LEN_Y] + 1 ) ;
    Result = (char*)malloc(TableSize);
    memset( Result, 0, TableSize );

    LCS_TraceBack(X, Y, LEN_X, LEN_Y, &Table, Result);

    printf("\n");
    printf("LCS:\"%s\" (Length:%d)\n", Result, Length);

    return 0;
}
