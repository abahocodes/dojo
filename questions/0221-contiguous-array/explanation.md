# Approach: balance + first occurrence

Replace every `0` by `-1`. A subarray has equally many `0`s and `1`s exactly
when its sum is `0`. Let `balance(i)` be the sum of the first `i + 1`
encoded elements, with `balance(-1) = 0`. The subarray from `j + 1` to `i` sums
to `balance(i) - balance(j)`, so it is balanced when the two balances are
equal.

For each `i`, the longest balanced subarray ending at `i` starts right after
the earliest `j` with the same balance. So we only need the first index at
which each balance value appeared.

```python
def find_max_length(nums):
    first = {0: -1}
    balance = 0
    best = 0
    for i, x in enumerate(nums):
        balance += 1 if x == 1 else -1
        if balance in first:
            best = max(best, i - first[balance])
        else:
            first[balance] = i
    return best
```

The balance always lies in `[-n, n]`, so the other solutions use an array of
size `2n + 1` (offset by `n`) instead of a hash map.

## Complexity

- Time: O(n), a single pass.
- Space: O(n) for the first-occurrence table.

## Pitfalls

- Forgetting the entry `balance 0 -> index -1`. Without it, a balanced prefix
  such as `[0, 1]` is never counted.
- Overwriting the stored index when a balance repeats. Keeping the **first**
  index is what makes the subarray the longest.
- A sliding window does not work here: extending a window can make it
  unbalanced and then balanced again, so there is no monotone shrink rule.
