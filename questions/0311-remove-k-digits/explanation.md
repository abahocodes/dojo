# Approach: greedy with a monotonic stack

The result always has `len(num) - k` digits (before stripping zeros), so we
want its leading digit as small as possible, then the next, and so on. Read the
digits left to right and keep the chosen ones on a stack. When the incoming
digit is smaller than the top of the stack and we still have deletions to
spend, the top should be deleted: replacing it by a smaller digit at that
position beats anything the later digits could achieve. The stack therefore
stays non-decreasing.

If deletions are left over after the scan, the stack is non-decreasing, so the
largest digits sit at its end: remove the last `k` of them. Then strip leading
zeros.

```python
def remove_kdigits(num, k):
    stack = []
    for d in num:
        while k > 0 and stack and stack[-1] > d:
            stack.pop()
            k -= 1
        stack.append(d)
    if k > 0:
        del stack[-k:]
    result = "".join(stack).lstrip("0")
    return result or "0"
```

## Complexity

- Time: O(n): each digit is pushed and popped at most once.
- Space: O(n) for the stack.

## Pitfalls

- Forgetting the leftover deletions when the input is already
  non-decreasing, e.g. `"12345"` with `k = 2` must give `"123"`.
- Stopping after a single pass that deletes one "peak" per scan and
  restarting: that is O(n * k), too slow for `10^5` digits. The stack handles
  every deletion in one pass.
- Not stripping leading zeros, or returning `""` instead of `"0"`.
- Converting to an integer type: `num` has up to `10^5` digits.
