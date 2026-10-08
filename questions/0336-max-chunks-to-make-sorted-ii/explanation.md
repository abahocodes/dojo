# Approach: stack of chunk maxima

Process the array left to right while maintaining the best chunking of the
prefix seen so far. Store only each chunk's maximum on a stack; these maxima
are non-decreasing from bottom to top (otherwise two chunks would be out of
order).

For a new value `x`:

- If `x >= top`, it can be a chunk of its own: push `x`.
- Otherwise `x` must end up before some earlier elements, so it has to join
  every chunk whose maximum is greater than `x`. Remember the current top (the
  largest maximum), pop while the top is greater than `x`, and push the
  remembered maximum back as the merged chunk.

At the end the number of chunks on the stack is the answer.

```python
def max_chunks_to_sorted(arr):
    stack = []
    for x in arr:
        if not stack or x >= stack[-1]:
            stack.append(x)
        else:
            biggest = stack[-1]
            while stack and stack[-1] > x:
                stack.pop()
            stack.append(biggest)
    return len(stack)
```

Equivalently, a cut after index `i` is valid exactly when
`max(arr[0..i]) <= min(arr[i+1..])`; counting those cuts with prefix maxima and
suffix minima gives the same answer.

## Complexity

- Time: O(n). Each element is pushed once and popped at most once.
- Space: O(n) for the stack.

## Pitfalls

- Using the "chunk ends when the prefix max equals the index" trick from the
  permutation version: with duplicates and arbitrary values it does not apply.
- Popping while `top >= x` instead of `top > x`. Equal values may stay in
  separate chunks, so `[1, 1]` must give `2`.
- Pushing `x` instead of the largest popped maximum after a merge; the merged
  chunk's maximum is the old top.
