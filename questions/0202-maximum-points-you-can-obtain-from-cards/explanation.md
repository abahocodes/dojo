# Approach: slide the split point

Any valid sequence of picks takes some prefix of length `i` and some suffix of
length `k - i`, for some `0 <= i <= k`, and every such combination is
reachable. Start with the sum of the first `k` cards (`i = k`). Moving to
`i - 1` removes `card_points[i - 1]` and adds the next card from the back. Each
move is O(1), so all `k + 1` splits are checked in O(k).

```python
def max_score_cards(card_points, k):
    n = len(card_points)
    current = sum(card_points[:k])
    best = current
    for i in range(1, k + 1):
        current += card_points[n - i] - card_points[k - i]
        best = max(best, current)
    return best
```

Another view: the cards not taken form one contiguous block of length `n - k`.
The answer is the total minus the smallest sum of such a window.

## Complexity

- Time: O(k), at most O(n).
- Space: O(1).

## Pitfalls

- Greedily taking the larger end card each turn fails: in Example 1 it would
  take 7, then 8, then 2, and end with 17.
- `k == n`: every card is taken; make sure your indices stay in range.
- The total can reach `10^9`, which still fits in a 32-bit integer.
