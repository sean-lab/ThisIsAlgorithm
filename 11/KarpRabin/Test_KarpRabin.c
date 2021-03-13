#include <stdio.h>
#include <string.h>
#include "KarpRabin.h"
#include <math.h>

#define MAX_BUFFER 512

int main(int argc, char** argv)
{
    char  Text[MAX_BUFFER] = "My name is pattern";
    char* Pattern = "pattern";
    int   PatternSize = 4;
    int h0, h1, rh1;

    int result = KarpRabin( Text, strlen(Text), 0, Pattern, strlen(Pattern));
    printf("%d\n", result);

    h0 = Hash(Pattern, PatternSize);
    h1 = Hash(Pattern+1, PatternSize);
    rh1 = ReHash(Pattern, 1, PatternSize, h0, (int)pow(2, PatternSize-1));

    printf("h0:%d\n", h0);
    printf("h1:%d\n", h1);
    printf("rh1:%d\n", rh1);


 /*   char* FilePath;
    FILE* fp;

    char  Text[MAX_BUFFER];
    char* Pattern;
    int   PatternSize = 0;
    int   Line        = 0;

    if ( argc < 3 )
    {
        printf("Usage: %s <FilePath> <Pattern>\n", argv[0] );
        return 1;
    }

    FilePath = argv[1];
    Pattern  = argv[2];

    PatternSize = strlen( Pattern );

    if ( (fp = fopen( FilePath, "r"))  == NULL )
    {
        printf("Cannot open file:%s\n", FilePath);
        return 1;
    } 

    while ( fgets( Text, MAX_BUFFER, fp ) != NULL )
    {
        int Position = 
            KarpRabin( Text, strlen(Text), 0, Pattern, PatternSize );
        
        Line++;

        if ( Position >= 0 ) 
        {
            printf("line:%d, column:%d : %s", Line, Position+1, Text);
        }
    }

    fclose( fp );*/

    return 0;
}