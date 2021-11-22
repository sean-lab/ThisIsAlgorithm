#include "DisjointSet.h"

void DS_UnionSet( DisjointSet* Set1, DisjointSet* Set2 )
{
    Set2 = DS_FindSet(Set2);
    Set2->Parent = Set1;
}

DisjointSet* DS_FindSet( DisjointSet* Set )
{
    while ( Set->Parent != NULL )
    {
        Set = Set->Parent;
    }

    return Set;
}

DisjointSet* DS_MakeSet( void* NewData )
{
    DisjointSet* NewNode = (DisjointSet*)malloc(sizeof(DisjointSet));
    NewNode->Data   = NewData;
    NewNode->Parent = NULL;

    return NewNode;
}

void DS_DestroySet( DisjointSet* Set )
{
    free( Set );
}
