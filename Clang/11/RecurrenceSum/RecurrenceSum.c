#include <stdio.h>

int RecurrenceSum( int Data[], int SizeOfData )
{
    if ( SizeOfData == 1 )
        return Data[0];
    else
        return RecurrenceSum( &Data[1], SizeOfData - 1 ) + Data[0];
}

int main( void )
{
    int i=0;
    int Data[150];

    for ( i=0; i<150; i++ )
        Data[i] = i+1;

    printf("Sum:%d\n", RecurrenceSum( Data, 55 ));

    return 0;
}
