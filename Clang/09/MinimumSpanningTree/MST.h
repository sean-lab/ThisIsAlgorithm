#ifndef MST_H
#define MST_H

#include "Graph.h"
#include "PriorityQueue.h"
#include "DisjointSet.h"

#define MAX_WEIGHT 36267

void Prim(Graph* G, Vertex* StartVertex, Graph* MST );
void Kruskal(Graph* G, Graph* MST );

#endif
