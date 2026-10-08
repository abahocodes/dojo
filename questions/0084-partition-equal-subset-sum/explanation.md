# Approach: subset-sum knapsack

Both groups must weigh `total / 2`, so an odd total is impossible. When the
total is even, it's enough to find **one** group weighing `half = total / 2`:
the remaining boxes automatically weigh the other half.

Let `can[s]` mean "some subset of the boxes processed so far weighs exactly
`s`". Initially only `can[0]` is true (the empty subset). Processing box `x`,
every reachable `s` makes `s + x` reachable too. Iterate `s` from high to low
so that `can[s - x]` still describes the subsets *without* box `x`.

```python
def can_partition(nums):
    total = sum(nums)
    if total % 2:
        return False
    half = total // 2
    can = [True] + [False] * half
    for x in nums:
        for s in range(half, x - 1, -1):
            if can[s - x]:
                can[s] = True
        if can[half]:
            return True
    return can[half]
```

**Bitset variant:** store the reachable sums as the bits of one big integer.
Adding box `x` is then a single `reachable |= reachable << x`, which updates
every sum at once. The Python reference solution does this, since Python
integers have arbitrary precision and the shift runs in native code.

## Complexity

- Time: O(n × half), at most 200 × 10,000 = 2 × 10^6 steps.
- Space: O(half).

## Pitfalls

- Sweeping `s` upwards lets the same box be counted several times (that's the
  unbounded knapsack), so `[1, 3]` would wrongly report `true` via `1 + 1`.
- Forgetting the odd-total check: integer division rounds `half` down, and you
  may "find" a group that doesn't actually balance the load.
- Trying every subset is 2^n, hopeless for 200 boxes.
- Greedy (sort descending, give each box to the lighter mover) fails, for
  example on `[3, 3, 2, 2, 2]`.
