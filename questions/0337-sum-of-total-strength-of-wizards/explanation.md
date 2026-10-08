# Approach: contribution of each minimum + prefix sums of prefix sums

Charge every group to its weakest wizard. To make the charge unique when
strengths repeat, let `left[i]` be the nearest index to the left with a
strictly smaller strength (or `-1`) and `right[i]` the nearest index to the
right with a smaller **or equal** strength (or `n`). Then wizard `i` is the
designated minimum of exactly the groups `[l..r]` with
`left[i] < l <= i <= r < right[i]`. Both arrays come from monotonic stacks.

Let `P[k]` be the sum of the first `k` strengths, so the group `[l..r]` sums
to `P[r+1] - P[l]`. Summing over all valid `l` and `r`:

```
sum over r, l of (P[r+1] - P[l])
  = (i - left) * (P[i+1] + ... + P[right])
  - (right - i) * (P[left+1] + ... + P[i])
```

With `PP[k] = P[0] + ... + P[k-1]`, both brackets are differences of `PP`.
Multiply by `strength[i]` and add, all modulo `10^9 + 7`.

```python
def total_strength(strength):
    MOD = 10**9 + 7
    n = len(strength)
    left, right = [-1] * n, [n] * n
    stack = []
    for i, x in enumerate(strength):
        while stack and strength[stack[-1]] >= x:
            right[stack.pop()] = i          # first smaller-or-equal to the right
        left[i] = stack[-1] if stack else -1  # first strictly smaller to the left
        stack.append(i)
    pp = [0] * (n + 2)
    p = 0
    for k in range(n + 1):
        pp[k + 1] = (pp[k] + p) % MOD
        if k < n:
            p = (p + strength[k]) % MOD
    total = 0
    for i, x in enumerate(strength):
        l, r = left[i], right[i]
        plus = (i - l) * (pp[r + 1] - pp[i + 1])
        minus = (r - i) * (pp[i + 1] - pp[l + 1])
        total = (total + x * ((plus - minus) % MOD)) % MOD
    return total
```

## Complexity

- Time: O(n): one stack pass and two linear passes.
- Space: O(n) for the boundaries and the prefix arrays.

## Pitfalls

- Using strict comparisons on both sides (or non-strict on both): groups with
  equal minima are then counted twice or not at all. Exactly one side must
  include equality.
- Overflow: strengths up to `10^9` times sums up to `10^14` overflow even
  64-bit integers. Reduce modulo `10^9 + 7` before every multiplication, and
  bring negative differences back into range before multiplying.
- Off-by-one in the `PP` ranges. Check the formula on a single wizard:
  `left = i - 1`, `right = i + 1` must give `strength[i]^2`.
