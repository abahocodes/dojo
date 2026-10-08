# Approach: prefix remainders in a hash map

Let `need = sum(nums) % p`. Deleting a subarray whose sum is congruent to
`need` modulo `p` leaves a multiple of `p`. With running prefix remainders
`cur`, the subarray ending at index `j` and starting after an earlier prefix
with remainder `r` qualifies when `r == (cur - need) mod p`.

To make it as short as possible, pair `j` with the most recent prefix that has
the required remainder, so the map keeps the latest index of each remainder.
The empty prefix has remainder `0` at index `-1`.

```python
def min_subarray_remove(nums, p):
    need = sum(nums) % p
    if need == 0:
        return 0
    latest = {0: -1}
    cur = 0
    best = len(nums)
    for j, x in enumerate(nums):
        cur = (cur + x) % p
        want = (cur - need) % p
        if want in latest:
            best = min(best, j - latest[want])
        latest[cur] = j
    return best if best < len(nums) else -1
```

## Complexity

- Time: O(n) expected, one pass with hash-map lookups.
- Space: O(min(n, p)) for the map.

## Pitfalls

- Overflow: the total can reach 10^14, beyond 32 bits. Reduce modulo `p` as
  you go, and use 64-bit arithmetic for `cur + x` in Java, C++ and Go.
- Negative remainders: `cur - need` can be negative, so add `p` before taking
  `% p` in languages where `%` keeps the sign.
- Returning the full length when the only option is deleting every element.
  That is not allowed; the answer is `-1`.
- Storing the *first* index of each remainder, which finds longer subarrays.
