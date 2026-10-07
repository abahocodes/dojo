A staircase has `n` steps. You start at the bottom, and each move takes you up
either **one** step or **two** steps.

Return the number of different move sequences that land you exactly on the top
step. Two sequences are different if they differ in at least one move.

## Example 1

```
n      = 4
output = 5      # 1+1+1+1, 1+1+2, 1+2+1, 2+1+1, 2+2
```

## Example 2

```
n      = 1
output = 1      # a single one-step move
```

## Constraints

- `1 <= n <= 45`
