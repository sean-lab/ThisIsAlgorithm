#include <stdio.h>
#include "DisjointSet.h"

int main( void )
{
	int a=1, b=2, c=3, d=4;
	DisjointSet* Set1 = DS_MakeSet(&a);
	DisjointSet* Set2 = DS_MakeSet(&b);
	DisjointSet* Set3 = DS_MakeSet(&c);
	DisjointSet* Set4 = DS_MakeSet(&d);

    printf("Set1 == Set2 : %d \n", DS_FindSet( Set1 ) == DS_FindSet( Set2 ) );

	DS_UnionSet( Set1, Set3 );
    printf("Set1 == Set3 : %d \n", DS_FindSet( Set1 ) == DS_FindSet( Set3 ) );

    DS_UnionSet( Set3, Set4 );
    printf("Set3 == Set4 : %d \n", DS_FindSet( Set3 ) == DS_FindSet( Set4 ) );

	return 0;
}
