#include "SimpleHashTable.h"

HashTable* SHT_CreateHashTable( int TableSize )
{
    HashTable* HT = (HashTable*)malloc( sizeof(HashTable) );
    HT->Table     = (Node*)malloc( sizeof(Node) * TableSize);
    HT->TableSize = TableSize;

    return HT;
}

void SHT_Set( HashTable* HT, KeyType Key, ValueType Value )
{
    int Address = SHT_Hash( Key, HT->TableSize );

    HT->Table[Address].Key   = Key;
    HT->Table[Address].Value = Value;
}

ValueType SHT_Get( HashTable* HT, KeyType Key )
{
    int Address = SHT_Hash( Key, HT->TableSize );

    return HT->Table[Address].Value;
}

void SHT_DestroyHashTable( HashTable* HT)
{
    free ( HT->Table );
    free ( HT );
}

int SHT_Hash( KeyType Key, int TableSize )
{
    return Key % TableSize;
}
