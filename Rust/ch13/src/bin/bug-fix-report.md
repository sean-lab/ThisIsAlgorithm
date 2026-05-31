# Bug Fix Report — Chapter 13 (Dynamic Programming)

Two real correctness/safety bugs were found in the original C sources while
porting them to Rust. Both are documented here with the fix applied in the
Rust port.

## 1. LCSDP — heap buffer overflow when sizing the result string

**File:** `Clang/13/LCSDP/LCSDP.c`, `main()`

```c
size_t TableSize = sizeof( Table.Data[LEN_X][LEN_Y] + 1 );
Result = (char*)malloc(TableSize);
memset( Result, 0, TableSize );

LCS_TraceBack(X, Y, LEN_X, LEN_Y, &Table, Result);
```

`Table.Data[LEN_X][LEN_Y]` is an `int` (the LCS length). The expression
`Table.Data[LEN_X][LEN_Y] + 1` is therefore also an `int`, and
`sizeof(int)` is **always 4** — independent of the LCS length. The intent
was clearly to allocate `LCS_length + 1` *bytes* (room for the characters
plus the terminating NUL).

For the sample input (`"GOOD MORNING."` vs `"GUTEN MORGEN."`) the LCS is
`"G MORN."` (length 7), which needs **8 bytes**, but only **4** are
allocated. `LCS_TraceBack` builds the string with `sprintf` and writes
past the end of the buffer — a heap buffer overflow (undefined behavior).
It "works" on the reference build purely because the allocator left slack
after the 4-byte block.

**Fix (Rust port `lcsdp.rs`):** the result is built into a growable
`String`, which always has the correct capacity, so the overflow cannot
occur. The visible program output is unchanged: `LCS:"G MORN." (Length:7)`.

The equivalent C fix would be:

```c
size_t TableSize = Table.Data[LEN_X][LEN_Y] + 1;   // length + 1 bytes
```

## 2. LCSDC — out-of-bounds read `X[-1]` in the table printer

**File:** `Clang/13/LCSDC/LCSDC.c`, `LCS_PrintTable()`

```c
for (i = 0; i < LEN_X + 1; i++)
{
    printf("%c ", X[i - 1]);   // i == 0  ->  X[-1]
    ...
}
```

On the first row (`i == 0`) this evaluates `X[-1]`, reading one byte
*before* the string literal `X`. This is an out-of-bounds read (undefined
behavior). On the reference build the byte happened to be `0x00`, so a
stray NUL was emitted into the table header; on another build it could be
any byte or crash.

The polished printer in `LCSDP.c` shows the intended behavior — it
special-cases the first row and prints a blank cell:

```c
if ( i == 0 )
    printf( "%2s", "");
else
    printf("%-2c", X[i-1]);
```

**Fix (Rust port `lcsdc.rs`):** print a blank (two-space) cell for row 0
instead of reading `X[-1]`. This makes the LCSDC table header consistent
with LCSDP and removes the out-of-bounds access. The only difference from
the original C output is that the first cell of row 0 is now a space
instead of a stray NUL byte.
