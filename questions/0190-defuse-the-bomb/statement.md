A lock shows a ring of `n` numbers, given as the array `code`. The ring is
circular: after `code[n - 1]` comes `code[0]`, and before `code[0]` comes
`code[n - 1]`. To open it you must replace **every** number at the same time,
using the original values, according to an integer `k`:

- if `k > 0`, replace `code[i]` with the sum of the `k` numbers that follow it
  (`code[i + 1]`, ..., `code[i + k]`, wrapping around);
- if `k < 0`, replace `code[i]` with the sum of the `|k|` numbers that precede
  it (`code[i - 1]`, ..., `code[i - |k|]`, wrapping around);
- if `k == 0`, replace `code[i]` with `0`.

Return the new ring as an array of length `n`.

## Example 1

```
code   = [6, 1, 3, 5]
k      = 2
output = [4, 8, 11, 7]    # e.g. index 2: 5 + 6 = 11; index 3: 6 + 1 = 7
```

## Example 2

```
code   = [2, 9, 4, 7, 1]
k      = -2
output = [8, 3, 11, 13, 11]    # index 0: 1 + 7; index 1: 2 + 1
```

## Constraints

- `1 <= n == len(code) <= 100`
- `1 <= code[i] <= 100`
- `-(n - 1) <= k <= n - 1`
