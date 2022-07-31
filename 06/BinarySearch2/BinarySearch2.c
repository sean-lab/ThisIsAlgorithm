#include <stdlib.h> 
#include <stdio.h>
#include "Point.h"

int ComparePoint( const void *_elem1, const void *_elem2 ) 
{ 
    Point* elem1 = (Point*)_elem1; 
    Point* elem2 = (Point*)_elem2; 
 
    if ( elem1->point > elem2->point ) 
        return 1; 
    else if ( elem1->point < elem2->point ) 
        return -1; 
    else 
        return 0;     
} 


int main( void )
{
    int Length = sizeof DataSet / sizeof DataSet[0];     
    Point  target;
    Point* found  = NULL;
 
    //  점수의 오름차순으로 정렬 
    qsort( (void*)DataSet, Length, sizeof (Point), ComparePoint );

    //  671.78 점을 받은 학생 찾기 
    target.id     = 0;
    target.point  = 671.78;

    found = bsearch( 
             (void*)&target, 
             (void*)DataSet, 
             Length, 
             sizeof (Point), 
             ComparePoint );
    
    printf("found... ID: %d, Point: %f \n", found->id, found->point );
 
    return 0;
}
