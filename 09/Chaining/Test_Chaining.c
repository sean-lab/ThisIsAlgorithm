#include "Chaining.h"

int main( void )
{
    HashTable* HT = CHT_CreateHashTable( 12289 );

    CHT_Set( HT, "MSFT",   "Microsoft Corporation");
    CHT_Set( HT, "JAVA",   "Sun Microsystems");
    CHT_Set( HT, "REDH",   "Red Hat Linux");
    CHT_Set( HT, "APAC",   "Apache Org");
    CHT_Set( HT, "ZYMZZ",  "Unisys Ops Check"); //  APAC와 충돌 
    CHT_Set( HT, "IBM",    "IBM Ltd.");
    CHT_Set( HT, "ORCL",   "Oracle Corporation");
    CHT_Set( HT, "CSCO",   "Cisco Systems, Inc.");
    CHT_Set( HT, "GOOG",   "Google Inc.");
    CHT_Set( HT, "YHOO",   "Yahoo! Inc.");
    CHT_Set( HT, "NOVL",   "Novell, Inc.");

    printf("\n");
    printf("Key:%s, Value:%s\n", "MSFT",   CHT_Get( HT, "MSFT" ) );
    printf("Key:%s, Value:%s\n", "REDH",   CHT_Get( HT, "REDH" ) );
    printf("Key:%s, Value:%s\n", "APAC",   CHT_Get( HT, "APAC" ) );
    printf("Key:%s, Value:%s\n", "ZYMZZ",  CHT_Get( HT, "ZYMZZ" ) );
    printf("Key:%s, Value:%s\n", "JAVA",   CHT_Get( HT, "JAVA" ) );
    printf("Key:%s, Value:%s\n", "IBM",    CHT_Get( HT, "IBM" ) );
    printf("Key:%s, Value:%s\n", "ORCL",   CHT_Get( HT, "ORCL" ) );
    printf("Key:%s, Value:%s\n", "CSCO",   CHT_Get( HT, "CSCO" ) );
    printf("Key:%s, Value:%s\n", "GOOG",   CHT_Get( HT, "GOOG" ) );
    printf("Key:%s, Value:%s\n", "YHOO",   CHT_Get( HT, "YHOO" ) );
    printf("Key:%s, Value:%s\n", "NOVL",   CHT_Get( HT, "NOVL" ) );

    CHT_DestroyHashTable( HT );

    return 0;
}
