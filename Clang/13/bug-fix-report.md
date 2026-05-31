# Bug Fix Report — Chapter 13 (Dynamic Programming, C 원본)

C 원본 예제 코드에서 발견된 두 가지 실제 정확성/안전성 버그와 그 수정 내용을
정리한다. 동일한 버그는 Rust 포팅(`Rust/ch13/`)에서도 수정되어 있으며, 두 버전
모두 콘솔 출력은 기존과 동일하게 유지된다.

## 1. LCSDP — 결과 문자열 크기 계산 시 힙 버퍼 오버플로

**파일:** `Clang/13/LCSDP/LCSDP.c`, `main()`

```c
size_t TableSize = sizeof( Table.Data[LEN_X][LEN_Y] + 1 ) ;
Result = (char*)malloc(TableSize);
memset( Result, 0, TableSize );

LCS_TraceBack(X, Y, LEN_X, LEN_Y, &Table, Result);
```

`Table.Data[LEN_X][LEN_Y]` 는 `int`(LCS 길이)이므로
`Table.Data[LEN_X][LEN_Y] + 1` 역시 `int` 이고, `sizeof(int)` 는 LCS 길이와
무관하게 **항상 4** 다. 의도는 분명히 `LCS_길이 + 1` *바이트*(문자 + 종료 NUL)를
할당하려던 것이다.

예제 입력(`"GOOD MORNING."` vs `"GUTEN MORGEN."`)의 LCS 는 `"G MORN."`
(길이 7)로 **8바이트**가 필요한데 **4바이트**만 할당된다. `LCS_TraceBack` 이
`sprintf` 로 문자열을 채우며 버퍼 끝을 넘어 써서 힙 버퍼 오버플로(미정의 동작)가
발생한다. 참조 빌드에서 "동작하는 것처럼 보이는" 이유는 할당기가 4바이트 블록
뒤에 여유 공간을 남겼기 때문일 뿐이다.

**수정:**

```c
size_t TableSize = Table.Data[LEN_X][LEN_Y] + 1;   // 길이 + 1 바이트
```

## 2. LCSDC — 테이블 출력 시 배열 경계 밖 읽기 `X[-1]`

**파일:** `Clang/13/LCSDC/LCSDC.c`, `LCS_PrintTable()`

```c
for (i = 0; i < LEN_X + 1; i++)
{
    printf("%c ", X[i - 1]);   // i == 0  ->  X[-1]
    ...
}
```

첫 행(`i == 0`)에서 `X[-1]` 을 평가하여 문자열 리터럴 `X` 한 바이트 *앞*을
읽는다. 이는 경계 밖 읽기(미정의 동작)다. 참조 빌드에서는 마침 그 바이트가
`0x00` 이라 헤더에 NUL 한 바이트가 끼어 출력될 뿐이지만, 다른 빌드에서는 임의의
바이트가 나오거나 크래시할 수 있다.

같은 챕터의 `LCSDP.c` 는 의도된 동작(첫 행은 빈 칸 출력)을 이미 보여준다.

**수정:** 첫 행에서는 `X[-1]` 을 읽는 대신 빈(공백 두 칸) 칸을 출력한다.

```c
if (i == 0)
    printf("%2s", "");
else
    printf("%c ", X[i - 1]);
```

기존 출력과의 유일한 차이는 첫 행 첫 칸이 NUL 한 바이트 대신 공백이 된다는
점이다.
