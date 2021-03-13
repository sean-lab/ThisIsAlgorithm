#include <stdio.h> 
#include <string.h> 
 
void InsertionSort(int DataSet[], int Length) 
{ 
    int i = 0; 
    int j = 0; 
    int value = 0; 
 
    for ( i=1; i<Length; i++ ) 
    { 
        if ( DataSet[i-1] <= DataSet[i] ) 
            continue; 
 
        value = DataSet[i]; 
 
        for ( j=0; j<i; j++ )  
        {             
            if (DataSet[j] > value)  
            { 
                memmove(&DataSet[j+1], &DataSet[j], sizeof(DataSet[0]) * (i-j)); 
                DataSet[j] = value; 
                break;     
            } 
        } 
    } 
} 
 
int main( void ) 
{ 
    int DataSet[] = {6, 4, 2, 3, 1, 5}; 
    int Length = sizeof DataSet / sizeof DataSet[0];     
    int i = 0; 
 
    InsertionSort(DataSet, Length); 
 
    for ( i=0; i<Length; i++ ) 
    { 
        printf("%d ", DataSet[i]); 
    } 
 
    printf("\n"); 
 
    return 0; 
}
