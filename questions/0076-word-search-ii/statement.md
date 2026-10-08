You are given a rectangular `board` of single lowercase letters and a list of
distinct `words`. A word is **on the board** if you can spell it by starting at
some cell and repeatedly stepping to a neighbouring cell (up, down, left or
right — not diagonally), reading one letter per cell. A single word may not
use the same cell twice.

Return every word from `words` that is on the board, in any order.

## Example 1

```
board = [
  ["c", "a", "t", "s"],
  ["o", "r", "e", "p"],
  ["d", "o", "g", "s"]
]
words  = ["cat", "core", "dog", "cats", "rope", "gods", "cog", "tops"]
output = ["cat", "cats", "core", "dog"]
```

## Example 2

```
board = [
  ["a", "b"],
  ["c", "d"]
]
words  = ["abdc", "abcd", "ba"]
output = ["abdc", "ba"]     # "abcd" would need a diagonal step from b to c
```

## Constraints

- `1 <= rows, cols <= 12`
- `1 <= len(words) <= 1000`
- `1 <= len(words[i]) <= 10`
- The board and the words contain only lowercase English letters.
- All words are distinct.
