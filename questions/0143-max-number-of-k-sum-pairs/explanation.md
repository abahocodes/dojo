# Approach: greedy pairing with a counter

Values only pair up within the classes `{x, k - x}`, and inside one class
the best you can do is `min(count(x), count(k - x))` pairs (or
`count(x) // 2` when `x == k - x`). A single scan achieves exactly that: keep
the counts of unpaired values seen so far, and whenever the current value's
partner is waiting, pair them immediately.

```python
def max_operations(nums, k):
    waiting = {}
    ops = 0
    for x in nums:
        partner = k - x
        if waiting.get(partner, 0) > 0:
            waiting[partner] -= 1
            ops += 1
        else:
            waiting[x] = waiting.get(x, 0) + 1
    return ops
```

The sort + two pointers alternative: sort `nums`, start at both ends, and
move the left pointer when the sum is too small, the right pointer when it
is too large, and both (counting an operation) when it equals `k`.

## Complexity

- Time: O(n) expected with the hash map (O(n log n) for the sorting variant).
- Space: O(n) for the map (O(1) extra beyond the sort otherwise).

## Pitfalls

- Pairing an element with itself: when `x == k - x`, it needs another copy.
  The scan handles this because `x` is only added to the map after the check.
- Counting pairs as `count(x) * count(k - x)`: each element can be removed
  only once.
- Values can exceed `k`; then `k - x` is negative and simply never matches.
