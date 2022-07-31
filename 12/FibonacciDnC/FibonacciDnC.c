#include <stdio.h>

typedef unsigned long ULONG;

typedef struct tagMatrix2x2 
{ 
    ULONG Data[2][2]; 
} Matrix2x2;

Matrix2x2 Matrix2x2_Multiply(Matrix2x2 A, Matrix2x2 B) 
{
    Matrix2x2 C;

    C.Data[0][0] = A.Data[0][0]*B.Data[0][0] + A.Data[0][1]*B.Data[1][0];
    C.Data[0][1] = A.Data[0][0]*B.Data[1][0] + A.Data[0][1]*B.Data[1][1];

    C.Data[1][0] = A.Data[1][0]*B.Data[0][0] + A.Data[1][1]*B.Data[1][0];
    C.Data[1][1] = A.Data[1][0]*B.Data[1][0] + A.Data[1][1]*B.Data[1][1];

    return C;
}

Matrix2x2 Matrix2x2_Power(Matrix2x2 A, int n) 
{
    if( n>1) 
    {
        A = Matrix2x2_Power( A, n/2 );
        A = Matrix2x2_Multiply( A, A );

        if( n & 1 )  
        {
            Matrix2x2 B;    
            B.Data[0][0] = 1; B.Data[0][1] = 1;
            B.Data[1][0] = 1; B.Data[1][1] = 0;

            A = Matrix2x2_Multiply( A, B );
        }
    }
              
    return A;
}

ULONG Fibonacci( int N ) 
{
    Matrix2x2 A;    
    A.Data[0][0] = 1; A.Data[0][1] = 1;
    A.Data[1][0] = 1; A.Data[1][1] = 0;

    A = Matrix2x2_Power(A, N);

    return A.Data[0][1];
}

int main( void )
{
    int   N      = 46;
    ULONG Result = Fibonacci( N );

    printf("Fibonacci(%d) = %lu\n", N, Result );

    return 0;
}

