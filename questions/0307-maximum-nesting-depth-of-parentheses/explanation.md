# Approach: a depth counter

Scanning left to right, the number of currently open parentheses is the
nesting depth at that point. That is what a stack of `(` would hold, but only
its size matters, so a counter replaces the stack:

- `(`: increase the depth and update the best seen;
- `)`: decrease the depth;
- digits and operators: ignore.

The input is guaranteed valid, so the depth never goes negative and ends at
zero.

```python
def max_depth_parens(s):
    depth = best = 0
    for ch in s:
        if ch == "(":
            depth += 1
            best = max(best, depth)
        elif ch == ")":
            depth -= 1
    return best
```

## Complexity

- Time: O(n), one pass.
- Space: O(1).

## Pitfalls

- Update the maximum right after incrementing, not after decrementing, or
  you report one less than the true depth.
- Don't count the total number of `(`: `"(1)+(2)"` has two pairs but depth 1.
- Expressions with no parentheses have depth 0.
