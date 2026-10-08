# Approach: contribution of each element via monotonic stacks

Every subarray's minimum sits at some position; attribute the subarray to that
position (the leftmost one if the minimum repeats). For position `i`, let

- `prev[i]` be the index of the nearest element to the left that is strictly
  smaller than `arr[i]` (or `-1`),
- `next[i]` be the index of the nearest element to the right that is
  `<= arr[i]` (or `n`).

The subarrays attributed to `i` start anywhere in `(prev[i], i]` and end
anywhere in `[i, next[i])`, so `i` contributes
`arr[i] * (i - prev[i]) * (next[i] - i)`. Because the right side stops at an
equal value but the left side does not, a subarray whose minimum appears
several times is attributed only to the rightmost copy, so it is counted
exactly once.

Both boundaries come out of a single pass with a stack of indices whose values
strictly increase from bottom to top. When `arr[j] <= arr[i]` pops index `i`,
we learn `next[i] = j`, and the index just below `i` on the stack is
`prev[i]` (everything between them was popped earlier because it was `>=`
`arr[i]`).

```python
def sum_subarray_mins(arr):
    MOD = 10**9 + 7
    n = len(arr)
    stack = []
    total = 0
    for j in range(n + 1):
        cur = arr[j] if j < n else 0
        while stack and arr[stack[-1]] >= cur:
            i = stack.pop()
            left = stack[-1] if stack else -1
            total += arr[i] * (i - left) * (j - i)
        stack.append(j)
    return total % MOD
```

The sentinel value `0` at `j = n` (smaller than every element) flushes the
stack at the end.

## Complexity

- Time: O(n): each index is pushed and popped once.
- Space: O(n) for the stack.

## Pitfalls

- Using strict comparisons on both sides (or non-strict on both). With
  `[2, 2]` the subarray `[2, 2]` is then counted twice (or never).
- Overflow: a single term can reach `3*10^4 * 3*10^4 * 3*10^4 / 4`, far beyond
  32 bits. Use 64-bit arithmetic and reduce modulo `10^9 + 7` as you go.
- Enumerating all subarrays is O(n^2), about `4.5 * 10^8` for `n = 3*10^4`.
