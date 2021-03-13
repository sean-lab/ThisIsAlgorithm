#include "ArrayStack.h"

void  AS_CreateStack(ArrayStack** Stack, int Capacity)
{
    /*  스택을 자유저장소에 생성 */
    (*Stack)           = (ArrayStack*)malloc(sizeof(ArrayStack));

    /*  입력된 Capacity만큼의 노드를 자유저장소에 생성 */
    (*Stack)->Nodes    = (Node*)malloc(sizeof(Node)*Capacity);

    /*  Capacity 및 Top 초기화 */
    (*Stack)->Capacity = Capacity;
    (*Stack)->Top = 0;
}

void AS_DestroyStack(ArrayStack* Stack)
{
    /*  노드를 자유 저장소에서 해제 */
    free(Stack->Nodes);

    /*  스택을 자유 저장소에서 해제 */
    free(Stack);
}

void AS_Push(ArrayStack* Stack, ElementType Data)
{
    int Position = Stack->Top;

    Stack->Nodes[Position].Data = Data;
    Stack->Top++;
}

ElementType AS_Pop(ArrayStack* Stack)
{
    int Position = --( Stack->Top );

    return Stack->Nodes[Position].Data;
}

ElementType AS_Top(ArrayStack* Stack)
{
    int Position = Stack->Top - 1;

    return Stack->Nodes[Position].Data;
}

int AS_GetSize(ArrayStack* Stack)
{
    return Stack->Top;
}

int AS_IsEmpty(ArrayStack* Stack)
{
    return (Stack->Top == 0);
}