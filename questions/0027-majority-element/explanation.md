# Approach: Boyer-Moore majority vote

Walk the list holding one `candidate` and a `count`. A matching value
increments the count, a different value decrements it (cancelling one
occurrence of each), and a count of zero lets the next value become the
candidate. Because the majority value occurs more often than all other values
combined, it cannot be fully cancelled, so it is the candidate left standing.

```python
def majority_element(nums):
    candidate, count = None, 0
    for x in nums:
        if count == 0:
            candidate = x
        count += 1 if x == candidate else -1
    return candidate
```

**Alternatives:** a hash map of counts (O(n) time, O(n) space), or
`sorted(nums)[len(nums) // 2]` (O(n log n)), which works because a value
filling more than half the list must cover the middle index.

## Complexity

- Time: O(n): a single pass.
- Space: O(1).

## Pitfalls

- The vote only works because a majority is **guaranteed**. Without that
  guarantee you'd need a second pass to confirm the candidate's count.
- Check `count == 0` **before** comparing, so the value that resets the
  candidate also counts toward it.
- Don't confuse "more than `n / 2`" with "most frequent": in
  `[1, 1, 2, 3]` there is no majority at all (this input is not allowed here).
