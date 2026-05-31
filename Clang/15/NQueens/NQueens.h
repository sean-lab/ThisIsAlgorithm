#ifndef NQUEENS_H
#define NQUEENS_H

void PrintSolution( int Columns[], int NumberOfQueens );
int  IsThreatened ( int Columns[], int NewRow );
void FindSolutionForQueen( int Columns[], int Row, 
                           int NumberOfQueens, int* SolutionCount );

#endif
