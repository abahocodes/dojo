A bar chart has `len(heights)` bars standing side by side, each one unit
wide; bar `i` is `heights[i]` units tall. Consider every axis-aligned rectangle
that fits entirely inside the bars (it sits on the baseline and spans a run of
adjacent bars, no taller than the shortest bar in that run).

Return the largest area such a rectangle can have.

## Example 1

```
heights = [3, 1, 4, 5, 2, 6]
output  = 8          # bars 2..3 (heights 4 and 5) give 2 * 4 = 8
```

## Example 2

```
heights = [2, 2, 2]
output  = 6
```

## Constraints

- `1 <= len(heights) <= 10^5`
- `0 <= heights[i] <= 10^4`
