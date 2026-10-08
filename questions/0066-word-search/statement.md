You are given a rectangular `board` of letters (each cell holds a one-letter
string) and a string `word`. Decide whether `word` can be spelled by a path on
the board: start on any cell, and step each time to a cell directly above,
below, left or right of the current one. Each cell may be used **at most
once** in the path.

Return `true` if such a path exists and `false` otherwise. Letters are
case-sensitive.

## Example 1

```
board = [
  ["S", "E", "A"],
  ["N", "O", "T"],
  ["A", "R", "E"]
]
word   = "SNORE"
output = true       # S(0,0) -> N(1,0) -> O(1,1) -> R(2,1) -> E(2,2)
```

## Example 2

```
board = [
  ["S", "E", "A"],
  ["N", "O", "T"],
  ["A", "R", "E"]
]
word   = "TOOT"
output = false      # the single O cannot be used twice
```

## Constraints

- `1 <= rows, cols <= 6`
- `1 <= len(word) <= 15`
- The board and `word` contain only English letters (upper or lower case).
