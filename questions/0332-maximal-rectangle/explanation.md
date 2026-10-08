# Approach: one histogram per row + monotonic stack

Any all-ones rectangle has a bottom row. For each row `r`, let `heights[c]`
be the number of consecutive `"1"` cells in column `c` ending at row `r`.
A rectangle with bottom row `r` covering columns `l..h` can be at most
`min(heights[l..h])` tall, so the best rectangle for this row is the largest
rectangle in the histogram `heights`.

The histogram problem is solved in linear time with a stack of indices whose
heights increase from bottom to top. When bar `i` is shorter than the bar on
top, the top bar can extend no further right; its left limit is the bar now
beneath it on the stack (everything between was taller and already popped).
So its widest rectangle has width `i - stack[-1] - 1` (or `i` if the stack is
empty). An extra bar of height 0 after the last column pops everything left.

```python
def maximal_rectangle(matrix):
    cols = len(matrix[0])
    heights = [0] * (cols + 1)          # last entry is a permanent 0 sentinel
    best = 0
    for row in matrix:
        for c in range(cols):
            heights[c] = heights[c] + 1 if row[c] == "1" else 0
        stack = []
        for i in range(cols + 1):
            while stack and heights[stack[-1]] >= heights[i]:
                h = heights[stack.pop()]
                left = stack[-1] if stack else -1
                best = max(best, h * (i - left - 1))
            stack.append(i)
    return best
```

## Complexity

- Time: O(rows * cols). Each row costs O(cols) to update the heights and
  O(cols) for the stack pass (every index is pushed and popped once).
- Space: O(cols) for the heights and the stack.

## Pitfalls

- Looking only for squares, or growing rectangles greedily from each cell:
  the widest run and the tallest run rarely give the best area.
- Forgetting to flush the stack at the end of each row; the sentinel bar of
  height 0 does it for free.
- Using the popped index instead of the new stack top as the left limit. Bars
  popped earlier were taller and still belong to the rectangle.
- Not resetting a column's height to 0 when the current cell is `"0"`.
