#include "Huffman.h"

HuffmanNode* Huffman_CreateNode( SymbolInfo NewData )
{
    HuffmanNode* NewNode = (HuffmanNode*)malloc( sizeof(HuffmanNode) );
    NewNode->Left    = NULL;
    NewNode->Right   = NULL;
    NewNode->Data    = NewData;

    return NewNode;
}

void Huffman_DestroyNode( HuffmanNode* Node )
{
    free(Node);
}

void Huffman_DestroyTree( HuffmanNode* Node )
{
    if ( Node == NULL )
        return;

    Huffman_DestroyTree( Node->Left );
    Huffman_DestroyTree( Node->Right );
    Huffman_DestroyNode( Node );
}

void Huffman_Huffman_AddBit( BitBuffer* Buffer, char Bit )
{
    UCHAR Mask = 0x80;
    
    if ( Buffer->Size % 8  == 0 ) 
    {
        Buffer->Buffer = 
            realloc(Buffer->Buffer, sizeof(UCHAR) * ((Buffer->Size / 8) + 1) );
        Buffer->Buffer[Buffer->Size / 8 ] = 0x00;
    }

    Mask >>= Buffer->Size % 8;

    if ( Bit == 1 )
       Buffer->Buffer[Buffer->Size / 8] |= Mask;
    else
      Buffer->Buffer[Buffer->Size / 8] &= ~Mask;

    Buffer->Size++;
}

void Huffman_BuildCodeTable( HuffmanNode* Tree, HuffmanCode CodeTable[MAX_CHAR], 
                            UCHAR Code[MAX_BIT], int Size )
{
    if ( Tree == NULL )
        return;
     
    if ( Tree->Left != NULL )
    {
        Code[Size] = 0;
        Huffman_BuildCodeTable( Tree->Left, CodeTable, Code, Size + 1 );
    }

    if ( Tree->Right != NULL )
    {
        Code[Size] = 1;
        Huffman_BuildCodeTable( Tree->Right, CodeTable, Code, Size + 1 );
    }

    if ( Tree->Left == NULL && Tree->Right == NULL )
    {
        int i;

        for ( i=0; i<Size; i++ ) 
            CodeTable[Tree->Data.Symbol].Code[i] = Code[i];

        CodeTable[Tree->Data.Symbol].Size = Size;
    }
}


void Huffman_BuildPrefixTree( HuffmanNode** Tree, 
                              SymbolInfo SymbolInfoTable[MAX_CHAR])
{
    int       i = 0;
    PQNode    Result;
    PriorityQueue* PQ = PQ_Create(0);
    

    for ( i=0; i<MAX_CHAR; i++ )
    {   
        if ( SymbolInfoTable[i].Frequency > 0 )
        {
            HuffmanNode* BitNode = Huffman_CreateNode( SymbolInfoTable[i] );
            PQNode NewNode;
			NewNode.Priority = SymbolInfoTable[i].Frequency;
			NewNode.Data     = BitNode;
            PQ_Enqueue( PQ, NewNode );
        }
    }

    while ( PQ->UsedSize > 1 )
    {
        SymbolInfo     NewData = { 0, 0 };
        HuffmanNode* BitNode = Huffman_CreateNode( NewData );
        HuffmanNode* Left;
        HuffmanNode* Right;

        PQNode QLeft;    
        PQNode QRight;
        PQNode NewNode;

        PQ_Dequeue( PQ, &QLeft );
        PQ_Dequeue( PQ, &QRight );

        Left  = (HuffmanNode*)QLeft.Data;
        Right = (HuffmanNode*)QRight.Data;

        BitNode->Data.Symbol = 0;
        BitNode->Data.Frequency =             
            Left->Data.Frequency + Right->Data.Frequency;

        BitNode->Left  = Left;
        BitNode->Right = Right;

        NewNode.Priority = BitNode->Data.Frequency;
        NewNode.Data     = BitNode;

        PQ_Enqueue( PQ, NewNode );
    }

    PQ_Dequeue( PQ, &Result );
    *Tree = (HuffmanNode*)Result.Data;    
}

void Huffman_Encode( HuffmanNode** Tree, UCHAR* Source, BitBuffer* Encoded, 
                    HuffmanCode CodeTable[MAX_CHAR] )
{
    int          i = 0,
                 j = 0;
    SymbolInfo     SymbolInfoTable[MAX_CHAR];
    UCHAR        Temporary[MAX_BIT];

    for ( i=0; i<MAX_CHAR; i++ ) 
    {
        SymbolInfoTable[i].Symbol    = i;
        SymbolInfoTable[i].Frequency = 0;
    }

    i=0;
    while ( Source[i] != '\0' )
    {
        SymbolInfoTable[Source[i++]].Frequency++;
    }

    Huffman_BuildPrefixTree( Tree, SymbolInfoTable );

    Huffman_BuildCodeTable( *Tree, CodeTable, Temporary, 0 );

    i=0;
    while ( Source[i] != '\0' )
    {
        int   BitCount = CodeTable[Source[i]].Size;

        for ( j=0; j<BitCount; j++ )
            Huffman_Huffman_AddBit( Encoded, CodeTable[Source[i]].Code[j] );
        
        i++;
    }
}

void Huffman_Decode( HuffmanNode* Tree, BitBuffer* Encoded, UCHAR* Decoded )
{
    int i;
    int Index = 0;
    HuffmanNode* Current = Tree;

    for ( i=0; i<=Encoded->Size; i++ )
    {
        UCHAR Mask = 0x80; //  1000 0000 
        
        if ( Current->Left == NULL && Current->Right == NULL )
        {
            Decoded[Index++] = Current->Data.Symbol;
            Current = Tree;
        }

        Mask >>= i % 8;

        if (  (Encoded->Buffer[i/8] & Mask) != Mask )
            Current = Current->Left;
        else
            Current = Current->Right;
    }

    Decoded[Index] = '\0';
}

void Huffman_PrintBinary( BitBuffer* Buffer )
{
    int i;

    for ( i=0; i<Buffer->Size; i++ )
    {
        UCHAR Mask = 0x80; //  1000 0000 
        Mask >>= i % 8;

        printf("%d", (Buffer->Buffer[i/8] & Mask) == Mask );
    }
}
