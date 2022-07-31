#ifndef OPEN_ADDRESSING_H
#define OPEN_ADDRESSING_H

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef char* KeyType;
typedef char* ValueType;

enum ElementStatus
{
    EMPTY    = 0,
    OCCUPIED = 1
};

typedef struct tagElementType
{
    KeyType    Key;
    ValueType  Value;

    enum ElementStatus Status;
} ElementType;

typedef struct tagHashTable 
{
    int           OccupiedCount;
    int           TableSize;

    ElementType*  Table;
} HashTable;

HashTable* OAHT_CreateHashTable( int TableSize );
void       OAHT_DestroyHashTable( HashTable* HT );
void       OAHT_ClearElement( ElementType* Element );

void       OAHT_Set( HashTable** HT, KeyType Key, ValueType Value );
ValueType  OAHT_Get( HashTable* HT, KeyType Key );
int        OAHT_Hash ( KeyType Key, int KeyLength, int TableSize );
int        OAHT_Hash2( KeyType Key, int KeyLength, int TableSize );

void       OAHT_Rehash( HashTable** HT );

#endif
