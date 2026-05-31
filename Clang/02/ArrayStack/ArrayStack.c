#include "ArrayStack.h"

void AS_CreateStack(ArrayStack** Stack, int Capacity)
{
    //  스택을 자유저장소에 생성
    (*Stack) = (ArrayStack*)malloc(sizeof(ArrayStack));

    if (*Stack == NULL)
    {
        printf("스택 메모리 할당 실패\n");
        return;
    }

    //  입력된 Capacity만큼의 노드를 자유저장소에 생성
    (*Stack)->Nodes = (Node*)malloc(sizeof(Node) * Capacity);

    if ((*Stack)->Nodes == NULL)
    {
        printf("노드 배열 메모리 할당 실패\n");
        free(*Stack);
        *Stack = NULL;
        return;
    }

    //  Capacity 및 Top 초기화
    (*Stack)->Capacity = Capacity;
    (*Stack)->Top = -1;
}

void AS_DestroyStack(ArrayStack *Stack)
{
    //  노드를 자유 저장소에서 해제
    free(Stack->Nodes);

    //  스택을 자유 저장소에서 해제
    free(Stack);
}

void AS_Push(ArrayStack *Stack, ElementType Data)
{
    if (Stack->Top >= Stack->Capacity - 1)
    {
        // 스택 오버플로우 방지
        printf("Stack Overflow!\n");
        return;
    }
    Stack->Top++;
    Stack->Nodes[Stack->Top].Data = Data;
}

ElementType AS_Pop(ArrayStack *Stack)
{
    if (Stack->Top < 0)
    {
        // 스택 언더플로우 방지
        printf("Stack Underflow!\n");
        return 0; // 기본값 반환
    }
    int Position = Stack->Top--;
    return Stack->Nodes[Position].Data;
}

ElementType AS_Top(ArrayStack *Stack)
{
    if (Stack->Top < 0)
    {
        // 빈 스택에서 Top 접근 방지
        printf("Stack is empty!\n");
        return 0; // 기본값 반환
    }
    return Stack->Nodes[Stack->Top].Data;
}

int AS_GetSize(ArrayStack *Stack)
{
    return Stack->Top + 1;
}

int AS_IsEmpty(ArrayStack *Stack)
{
    return (Stack->Top == -1);
}
