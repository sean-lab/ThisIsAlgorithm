#include "BinarySearchTree.h"

int main( void )
{
    /*  노드 생성 */
    BSTNode* Tree = BST_CreateNode(123);
    BSTNode* Node = NULL;

    /*  트리에 노드 추가 */
    BST_InsertNode( Tree, BST_CreateNode(22) );
    BST_InsertNode( Tree, BST_CreateNode(9918) );
    BST_InsertNode( Tree, BST_CreateNode(424) );
    BST_InsertNode( Tree, BST_CreateNode(17) );
    BST_InsertNode( Tree, BST_CreateNode(3) );

    BST_InsertNode( Tree, BST_CreateNode(98) );
    BST_InsertNode( Tree, BST_CreateNode(34) );

    BST_InsertNode( Tree, BST_CreateNode(760) );
    BST_InsertNode( Tree, BST_CreateNode(317) );
    BST_InsertNode( Tree, BST_CreateNode(1) );
    
	Node =  BST_SearchNode(Tree, 17 );
    if(Node != NULL)
        printf("%d \n", Node->Data);
    else
        puts("그런 노드 없어요");

    /*  트리 출력 */
    BST_InorderPrintTree( Tree );
    printf( "\n");

    /*  특정 노드 삭제 */
    printf( "Removing 98...\n");

    Node = BST_RemoveNode( Tree, NULL, 98 );
    BST_DestroyNode( Node );

    BST_InorderPrintTree( Tree );
    printf( "\n");

    /*  새 노드 삽입 */
    printf( "Inserting 111...\n");

    BST_InsertNode( Tree, BST_CreateNode(111) );
    BST_InorderPrintTree( Tree );
    printf( "\n");

    /*  트리 소멸시키기 */
    BST_DestroyTree( Tree );

    return 0;
}