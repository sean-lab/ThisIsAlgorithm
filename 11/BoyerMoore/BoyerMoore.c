#include "BoyerMoore.h"
#include <stdlib.h>

int  BoyerMoore(char* Text, int TextSize, int Start, 
                char* Pattern, int PatternSize )
{
    int BCT[128];
    int* Suffix = (int*)calloc( PatternSize + 1, sizeof( int ) );
    int* GST  = (int*)calloc( PatternSize + 1, sizeof( int ) );
    int i = Start;
    int j = 0;

    int Position = -1;

    BuildBCT( Pattern, PatternSize, BCT );
    BuildGST( Pattern, PatternSize, Suffix, GST );
    
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
            i+= Max( GST[j+1], j-BCT[ Text[i+j] ])  ;
        }
    }

    free ( Suffix );
    free ( GST  );

    return Position;
}

void BuildBCT( char* Pattern, int PatternSize, int* BCT )
{
    int i;
    int j;

    for ( i=0; i<128; i++ ) 
        BCT[i]=-1;

    for ( j=0; j<PatternSize; j++ )
        BCT[ Pattern[j] ]=j;
}

void BuildGST( char* Pattern, int PatternSize, int* Suffix, int* GST )
{
    /*  Case 1 */
    int i = PatternSize;
    int j = PatternSize + 1;

    Suffix[i]=j; 
    
    while (i>0)
    {
        while (j<=PatternSize && Pattern[i-1] != Pattern[j-1])
        {
            if ( GST[j] == 0 ) 
                GST[j]=j-i;

            j=Suffix[j];
        }

        i--; 
        j--;
        
        Suffix[i] = j;
    }

    /*  Case 2 */
    j = Suffix[0];

    for ( i = 0; i <= PatternSize; i++ )
    {
        if ( GST[i] == 0 ) 
            GST[i] = j;

        if ( i == j ) 
            j = Suffix[j];
    }
}

int  Max( int A, int B )
{
    if ( A > B )
        return A;
    else
        return B;
}