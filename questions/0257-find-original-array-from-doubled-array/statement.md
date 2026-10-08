Someone took an integer array `original`, appended **twice the value** of
each of its elements (so `[1, 3]` becomes `[1, 3, 2, 6]`), and then shuffled
the result. You are given that shuffled array as `changed`.

If `changed` could have been produced this way, return `original` sorted in
ascending order. (When it is possible, the sorted `original` is unique.)
Otherwise return an empty array.

## Example 1

```
changed = [6, 3, 2, 4, 1, 8]
output  = [1, 3, 4]   # 1 -> 2, 3 -> 6, 4 -> 8
```

## Example 2

```
changed = [5, 10, 10]
output  = []          # odd length: cannot be a doubled array
```

## Constraints

- `1 <= len(changed) <= 10^5`
- `0 <= changed[i] <= 10^5`
