#ifndef LINKED_QUEUE_H
#define LINKED_QUEUE_H

#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include "Graph.h"

typedef struct tagNode
{
    Vertex*         Data;
    struct tagNode* NextNode;
} Node;

typedef struct tagLinkedQueue
{
    Node* Front;
    Node* Rear;
    int   Count;
} LinkedQueue;

void  LQ_CreateQueue( LinkedQueue** Queue );
void  LQ_DestroyQueue( LinkedQueue* Queue );

Node* LQ_CreateNode(Vertex* V);
void  LQ_DestroyNode(Node* _Node);

void  LQ_Enqueue( LinkedQueue* Queue, Node* NewNode );
Node* LQ_Dequeue( LinkedQueue* Queue );

int   LQ_IsEmpty( LinkedQueue* Queue );

#endif
