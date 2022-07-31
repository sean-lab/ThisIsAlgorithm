#include "BoyerMoore.h"
#include <stdlib.h>

int print_table(int* table, int table_size)
{
    for(int i=0; i<table_size; i++)
        if (table[i] > 0)
            printf("'%C':%d, ", (char)i, table[i]);
    printf("\n");
}

int print_GoodSuffTable_table(int* table, int* PosOfBorder, char* Pattern, int table_size)
{
    for(int i=0; i<table_size; i++)
        if (table[i] > 0)
            printf("'%C':%d(%d), ", Pattern[i], table[i], PosOfBorder[i]);
    printf("\n");
}

int  BoyerMoore(char* Text, int TextSize, int Start, 
                char* Pattern, int PatternSize )
{
    int BadCharTable[128]; 
    int* GoodSuffTable  = (int*)calloc( PatternSize + 1, sizeof( int ) );
    int* PosOfBorder = (int*)calloc( PatternSize + 1, sizeof( int ) );
    int i = Start;
    int j = 0;

    int Position = -1;

    BuildBST( Pattern, PatternSize, BadCharTable );    
    BuildGST( Pattern, PatternSize, PosOfBorder, GoodSuffTable );

    print_table(BadCharTable, 128);
    print_GoodSuffTable_table(GoodSuffTable, PosOfBorder, Pattern, PatternSize+1);
    
    while (i <= TextSize - PatternSize)
    {
        j = PatternSize - 1;

        while ( j >= 0 && Pattern[j] == Text[i+j] ) 
            j--;

        if (j<0)
        {
            Position = i;
            break;
        }
        else 
        {
            i+= Max( GoodSuffTable[j+1], j-BadCharTable[ Text[i+j] ])  ;
        }
    }

    free ( PosOfBorder );
    free ( GoodSuffTable  );

    return Position;
}

void BuildBST( char* Pattern, int PatternSize, int* BadCharTable )
{
    int i;
    int j;

    for ( i=0; i<128; i++ ) 
        BadCharTable[i]=-1;

    for ( j=0; j<PatternSize; j++ )
        BadCharTable[ Pattern[j] ]=j;
}

void BuildGST( char* Pattern, int PatternSize, int* PosOfBorder, int* GoodSuffTable )
{
    //  Case 1 
    int i = PatternSize;
    int j = PatternSize + 1;

    PosOfBorder[i]=j; 
    
    while (i>0)
    {
        printf("%s:%d) %s, i:%d, j:%d\n", __FUNCTION__ , __LINE__, Pattern, i,j );
        while (j<=PatternSize && Pattern[i-1] != Pattern[j-1])
        {
            if ( GoodSuffTable[j] == 0 ) 
                GoodSuffTable[j]=j-i;

            j=PosOfBorder[j];
        }

        i--; 
        j--;
        
        PosOfBorder[i] = j;
    }

    //  Case 2 
    j = PosOfBorder[0];

    for ( i = 0; i <= PatternSize; i++ )
    {
        if ( GoodSuffTable[i] == 0 ) 
            GoodSuffTable[i] = j;

        if ( i == j ) 
            j = PosOfBorder[j];
    }
}

int  Max( int A, int B )
{
    if ( A > B )
        return A;
    else
        return B;
}
