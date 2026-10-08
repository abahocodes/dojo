Students are standing in a row for a photo, and `heights[i]` is the height of
the student at position `i`. The photographer wants them lined up in
non-decreasing order of height.

Return the number of positions `i` where the student currently standing there
is not the height that belongs at position `i` in the sorted line-up.

## Example 1

```
heights = [2, 2, 4, 1, 3, 2]
output  = 4    # sorted: [1, 2, 2, 2, 3, 4]
               # positions 0, 2, 3 and 5 differ
```

## Example 2

```
heights = [3, 5, 8, 8]
output  = 0    # already in order
```

## Constraints

- `1 <= len(heights) <= 100`
- `1 <= heights[i] <= 100`
