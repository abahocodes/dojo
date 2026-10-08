# Approach: balance of greater vs. smaller, split at k

Every qualifying subarray contains `k`. Map each other element to `+1` if it
is greater than `k` and `-1` if smaller, and call the subarray's sum its
*balance*. With `g` greater and `s` smaller elements:

- odd length: `k` is the middle exactly when `g == s` (balance 0);
- even length: `k` is the left middle exactly when `g == s + 1` (balance 1).

So we need subarrays containing `k` with balance 0 or 1.

Let `p` be the index of `k`. A subarray `[l, r]` with `l <= p <= r` splits
into a left part `nums[l..p-1]` and a right part `nums[p+1..r]`, and its
balance is `L + R`. Walk right from `p`, counting how many times each right
balance `R` occurs (the empty right part gives `R = 0`). Then walk left from
`p`, and for each left balance `L` (again starting with the empty part) add
`count[-L] + count[1 - L]`.

```python
def count_subarrays_median_k(nums, k):
    p = nums.index(k)
    right = {0: 1}
    bal = 0
    for i in range(p + 1, len(nums)):
        bal += 1 if nums[i] > k else -1
        right[bal] = right.get(bal, 0) + 1
    total = 0
    bal = 0
    for i in range(p, -1, -1):
        if i < p:
            bal += 1 if nums[i] > k else -1
        total += right.get(-bal, 0) + right.get(1 - bal, 0)
    return total
```

Balances lie in `[-n, n]`, so the Java, C++ and Go solutions use an array
offset by `n` instead of a hash map.

## Complexity

- Time: O(n), two linear walks.
- Space: O(n) for the balance counts.

## Pitfalls

- Forgetting the empty left or right part (balance 0), which drops every
  subarray that starts or ends at `k`.
- Accepting balance `-1` for even length: that makes `k` the *right*
  middle, but the median is defined as the left one.
- Trying to maintain sorted windows or heaps: O(n^2 log n) is far too slow
  for `n = 10^5`.
