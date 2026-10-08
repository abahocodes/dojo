A city's buildings are drawn as rectangles standing on flat ground at height
`0`. Building `i` is given as `buildings[i] = [left, right, height]`: it covers
every `x` with `left <= x < right` up to the given height. Buildings may
overlap, touch, or nest inside one another. The list is sorted by `left`.

Seen from far away, the buildings merge into a single outline, the
**skyline**: at each `x` its height is the tallest building covering `x`, or
`0` if none does. Describe the skyline by its **key points** `[x, y]`, where
each key point says "starting at `x`, the outline is at height `y`".

Return the key points sorted by `x`, following these rules:

- a key point appears exactly where the outline height changes, so two
  consecutive key points never have the same `y` (an outline that stays flat
  across touching buildings of equal height gets no extra point);
- the first key point is at the leftmost `left`, and the last key point has
  `y = 0`, marking where the final building ends.

## Example 1

```
buildings = [[1, 5, 8], [3, 7, 5], [4, 9, 10], [11, 13, 6], [12, 15, 6]]
output    = [[1, 8], [4, 10], [9, 0], [11, 6], [15, 0]]
```

The building of height `5` is hidden behind its taller neighbours, and the two
buildings of height `6` merge into one flat stretch from `11` to `15`.

## Example 2

```
buildings = [[0, 3, 4], [3, 6, 4]]
output    = [[0, 4], [6, 0]]
```

The two buildings touch at `x = 3` with the same height, so the outline does
not change there.

## Constraints

- `1 <= len(buildings) <= 10^4`
- `0 <= left < right <= 2^31 - 1`
- `1 <= height <= 2^31 - 1`
- `buildings` is sorted by `left` in non-decreasing order.
