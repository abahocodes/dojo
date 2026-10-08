# Approach: monotonic stack of increasing heights

For each bar, the widest rectangle using that bar as its shortest one spans
from just after the nearest shorter bar on the left to just before the nearest
shorter bar on the right.

Scan the bars with a stack of indices whose heights increase from bottom to top.
When a bar `i` is lower than the top, the top bar has found its right boundary
(`i`), and the element below it on the stack is its left boundary. Pop, compute
the area, and repeat. A trailing sentinel of height `0` forces every remaining
bar to be popped.

```python
def largest_rectangle_area(heights):
    stack = []
    best = 0
    for i, h in enumerate(heights + [0]):
        while stack and heights[stack[-1]] >= h:
            height = heights[stack.pop()]
            left = stack[-1] if stack else -1
            best = max(best, height * (i - left - 1))
        stack.append(i)
    return best
```

## Complexity

- Time: O(n) — each bar is pushed and popped once.
- Space: O(n) for the stack.

## Pitfalls

- Forgetting the final flush: bars still on the stack at the end extend to the
  right edge. The sentinel `0` handles this uniformly.
- The left boundary is the index *below* the popped one on the stack, not the
  popped index itself; with an empty stack the bar reaches the left edge (`-1`).
- Popping on `>=` versus `>` both give the right maximum: equal bars are
  measured correctly by whichever of them is popped last.
