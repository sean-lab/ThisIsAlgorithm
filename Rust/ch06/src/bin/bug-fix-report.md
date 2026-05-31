# Bug Fix Report — BinarySearchTree (06)

## 대상
- 원본 C: `Clang/06/BinarySearchTree/BinarySearchTree.c` 의 `BST_RemoveNode`
- 포팅: `Rust/ch06/src/bin/binary_search_tree.rs` 의 `bst_remove_node`

## 버그 내용
원본 `BST_RemoveNode`는 삭제 대상 노드가 **양쪽 자식을 모두 가진 경우**,
오른쪽 서브트리의 최소 노드를 물리적으로 제거한 뒤 그 값을 현재 노드에 복사한다.

```c
Removed = Tree;                       // 반환값을 현재 노드로 고정
...
if (Tree->Left != NULL && Tree->Right != NULL)
{
    BSTNode* MinNode = BST_SearchMinNode(Tree->Right);
    MinNode = BST_RemoveNode(Tree, NULL, MinNode->Data);  // 실제로 분리된 노드는 MinNode
    Tree->Data = MinNode->Data;
}
...
return Removed;                       // 그러나 MinNode가 아니라 Tree를 반환
```

이때 함수는 실제로 트리에서 분리된 `MinNode`가 아니라 **여전히 트리에 연결되어 있는
`Tree`(= Removed)** 를 반환한다. 호출부(`Test_BinarySearchTree.c`)는 반환된 노드를
`BST_DestroyNode(Node)`로 해제하므로:

1. **트리에 아직 연결되어 있는 노드가 free 되어** 댕글링 포인터/힙 손상이 발생하고,
2. 실제로 분리된 최소 노드는 **해제되지 않고 누수(memory leak)** 된다.

> 참고: 본 예제의 테스트 시나리오(`Removing 98...`)는 자식이 하나뿐인 경우만 호출하므로
> 이 버그 경로는 실행되지 않아 콘솔 출력에는 영향이 없다. 그러나 양쪽 자식을 가진 노드를
> 삭제하면 즉시 문제가 드러난다.

## 수정 내용
Rust 포팅에서는 양쪽 자식 케이스에서 실제로 분리된 최소 노드를 반환하도록 수정했다.

```rust
let min_node = bst_search_min_node((*tree).right);
let min_node = bst_remove_node(tree, ptr::null_mut(), (*min_node).data);
(*tree).data = (*min_node).data;
removed = min_node;   // 분리된 노드를 반환하여 호출부가 올바른 노드를 해제
```

이로써 호출부에서 `bst_destroy_node(node)` 호출 시 트리에 연결된 노드가 아닌
실제로 분리된 노드가 해제되어, 힙 손상과 메모리 누수가 모두 해소된다.
테스트 시나리오의 출력 결과는 동일하게 유지된다.
