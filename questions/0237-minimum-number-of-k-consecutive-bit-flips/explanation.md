# Approach: greedy scan with a running flip parity

Two facts make this a greedy problem. Flips commute and flipping the same
block twice cancels out, so a solution is just a set of starting positions,
each used at most once. And the leftmost element is touched only by the block
starting at index 0, so if it is 0 that block must be flipped, otherwise it
must not. Once that is decided, index 1 is touched only by blocks starting at
0 or 1, and the first is already decided, so the block at 1 is forced too. By
induction the whole set of flips is forced, which makes the greedy both
correct and minimal.

Simulating each flip costs `O(k)`. Instead keep `active`, the parity of flips
that cover the current index, and a marker array `ends` where `ends[j]` says
that a flip started at `j - k` stops covering at `j`. At index `i`, first undo
the flips ending there, then the effective bit is `nums[i] ^ active`. If it is
0, start a flip at `i`, which needs `i + k <= n`; otherwise it is impossible.

```python
def min_k_bit_flips(nums, k):
    n = len(nums)
    ends = [0] * (n + 1)
    active = 0
    flips = 0
    for i in range(n):
        active ^= ends[i]
        if nums[i] ^ active == 0:
            if i + k > n:
                return -1
            flips += 1
            active ^= 1
            ends[i + k] ^= 1
    return flips
```

## Complexity

- Time: O(n), one pass with constant work per index.
- Space: O(n) for the marker array (it can be reduced to O(1) by marking
  flips in `nums` itself, at the cost of modifying the input).

## Pitfalls

- Inverting all `k` elements on each flip: correct but `O(n * k)`, about
  `10^10` operations in the worst case.
- Forgetting the bounds check: a 0 found in the last `k - 1` positions cannot
  start a full block, so the answer is `-1`.
- Using a counter of active flips without reducing it mod 2. Only the parity
  decides whether a bit is inverted.
- `k = 1` flips each 0 individually; `k = n` allows only the all-0 or
  already-all-1 arrays.
