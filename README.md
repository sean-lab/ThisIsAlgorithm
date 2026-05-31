# 개요 (Abstract)

『이것이 자료구조+알고리즘이다 with C언어』 예제 코드 저장소입니다.
Sample code for *This is the Data Structure + Algorithm with C*.

이 저장소는 두 가지 구현을 함께 제공합니다.

- **[`Clang/`](Clang/)** — 책의 원본 C 예제 코드 (장별로 정리)
- **[`Rust/`](Rust/)** — 동일한 예제를 Rust로 포팅한 Cargo 워크스페이스

두 버전은 콘솔 출력이 **바이트 단위로 동일**하도록 맞추어져 있습니다. 원본 C
코드에서 발견된 일부 버그는 양쪽 모두에서 수정되었으며, 그 내용은 각 디렉터리의
`bug-fix-report.md`에 문서화되어 있습니다.

# 서적 구매처

- [Yes24](https://www.yes24.com/Product/Goods/111362116)
- [교보문고](https://www.yes24.com/Product/Goods/111362116)

# 디렉터리 구성

장 번호는 두 버전이 동일하게 대응됩니다 (`Clang/01` ↔ `Rust/ch01`).

| 장 | 주제 | C | Rust |
|----|------|---|------|
| 01 | 리스트 (Linked List) | [Clang/01](Clang/01) | [Rust/ch01](Rust/ch01) |
| 02 | 스택 (Stack) | [Clang/02](Clang/02) | [Rust/ch02](Rust/ch02) |
| 03 | 큐 (Queue) | [Clang/03](Clang/03) | [Rust/ch03](Rust/ch03) |
| 04 | 트리 (Tree) | [Clang/04](Clang/04) | [Rust/ch04](Rust/ch04) |
| 05 | 정렬 (Sorting) | [Clang/05](Clang/05) | [Rust/ch05](Rust/ch05) |
| 06 | 탐색 (Searching) | [Clang/06](Clang/06) | [Rust/ch06](Rust/ch06) |
| 07 | 우선순위 큐와 힙 | [Clang/07](Clang/07) | [Rust/ch07](Rust/ch07) |
| 08 | 해시 테이블 (Hash Table) | [Clang/08](Clang/08) | [Rust/ch08](Rust/ch08) |
| 09 | 그래프 (Graph) | [Clang/09](Clang/09) | [Rust/ch09](Rust/ch09) |
| 10 | 문자열 탐색 (String Search) | [Clang/10](Clang/10) | [Rust/ch10](Rust/ch10) |
| 11 | 점화식과 재귀 | [Clang/11](Clang/11) | [Rust/ch11](Rust/ch11) |
| 12 | 분할 정복 (Divide & Conquer) | [Clang/12](Clang/12) | [Rust/ch12](Rust/ch12) |
| 13 | 동적 계획법 (Dynamic Programming) | [Clang/13](Clang/13) | [Rust/ch13](Rust/ch13) |
| 14 | 탐욕 알고리즘 (Greedy) | [Clang/14](Clang/14) | [Rust/ch14](Rust/ch14) |
| 15 | 백트래킹 (Backtracking) | [Clang/15](Clang/15) | [Rust/ch15](Rust/ch15) |

# C 버전 (`Clang/`)

각 장은 `Makefile`을 가진 하위 예제 디렉터리들로 구성됩니다. GCC(또는 Clang)와
`make`가 필요합니다.

```sh
# 한 예제 빌드 (예: 01장 LinkedList)
cd Clang/01/LinkedList
make
./LinkedList

# 한 장의 모든 예제 빌드
cd Clang/01
make

# 정리
make clean      # 오브젝트 파일 삭제
make distclean  # 오브젝트 파일 + 실행 파일 삭제
```

`Makefile` 없이 직접 컴파일할 수도 있습니다.

```sh
cd Clang/01/LinkedList
cc -std=c99 -Wall -o LinkedList LinkedList.c Test_LinkedList.c
./LinkedList
```

# Rust 버전 (`Rust/`)

장별 크레이트(`ch01`–`ch15`)로 이루어진 단일 Cargo 워크스페이스입니다.
[Rust 툴체인](https://www.rust-lang.org/tools/install) (`cargo`)이 필요합니다.

각 크레이트는 알고리즘 로직을 라이브러리 모듈(`src/*.rs`)로 분리하고, 책의 각
예제에 대응하는 실행 바이너리(`src/bin/*.rs`)를 제공합니다. 단위 테스트는 각
모듈 파일 안의 `#[cfg(test)]` 블록에 포함되어 있습니다.

```sh
cd Rust

# 워크스페이스 전체 빌드
cargo build

# 특정 예제 실행 (cargo run -p <크레이트> --bin <바이너리>)
cargo run -p ch01 --bin linked_list

# 인자가 필요한 예제는 `--` 뒤에 전달
cargo run -p ch15 --bin n_queens -- 8
cargo run -p ch10 --bin brute_force -- ../Clang/10/kjv.txt God

# 한 크레이트의 바이너리 목록 보기
cargo run -p ch01

# 테스트 실행
cargo test            # 전체
cargo test -p ch01    # 특정 장
```

# 버그 수정 (Bug Fixes)

원본 C 예제에서 발견된 정확성/안전성 버그는 C와 Rust 양쪽에서 수정되었습니다.
자세한 내용은 다음 문서를 참고하세요.

- [Clang/06/BinarySearchTree/bug-fix-report.md](Clang/06/BinarySearchTree/bug-fix-report.md) — `BST_RemoveNode` 잘못된 노드 반환 (힙 손상/누수)
- [Clang/13/bug-fix-report.md](Clang/13/bug-fix-report.md) — LCSDP 힙 버퍼 오버플로, LCSDC 배열 경계 밖 읽기
