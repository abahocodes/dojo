A chess queen attacks every square in its row, its column and both of its
diagonals. Given `n`, find every way to place `n` queens on an `n × n` board
so that no queen attacks another.

Describe each placement as a board: a list of `n` strings, one per row from
top to bottom, where row strings have length `n`, `'Q'` marks a queen and
`'.'` an empty square. Return all boards, in any order. The rows inside a
board must stay in top-to-bottom order. Rotations and reflections of a
placement are different boards and are all listed.

## Example 1

```
n      = 4
output = [[".Q..", "...Q", "Q...", "..Q."],
          ["..Q.", "Q...", "...Q", ".Q.."]]
```

## Example 2

```
n      = 1
output = [["Q"]]
```

## Example 3

```
n      = 3
output = []
```

## Constraints

- `1 <= n <= 9`
