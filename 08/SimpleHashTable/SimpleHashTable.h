#ifndef SIMPLE_HASHTABLE_H
#define SIMPLE_HASHTABLE_H

#include <stdio.h>
#include <stdlib.h>

typedef int KeyType;
typedef int ValueType;

typedef struct tagNode
{
    KeyType   Key;
    ValueType Value;
} Node;

typedef struct tagHashTable 
{
    int   TableSize;
    Node* Table;
} HashTable;

HashTable* SHT_CreateHashTable( int TableSize );
void       SHT_Set( HashTable* HT, KeyType Key, ValueType Value );
ValueType  SHT_Get( HashTable* HT, KeyType Key );
void       SHT_DestroyHashTable( HashTable* HT);
int        SHT_Hash( KeyType Key, int TableSize );

#endif
