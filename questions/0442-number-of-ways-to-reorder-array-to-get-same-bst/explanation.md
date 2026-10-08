# Approach: count interleavings subtree by subtree

The first inserted value becomes the root, so every valid ordering starts with
it. The remaining values split into a left group (smaller than the root) and a
right group (larger). Values from different groups never meet in the tree, so
their relative order is irrelevant; what matters is that each group, read on
its own, is an ordering that builds its own subtree.

For a node whose subtrees have `L` and `R` nodes, the number of orderings is

```
ways(node) = C(L + R, L) * ways(left) * ways(right),   ways(empty) = 1
```

where `C(L + R, L)` counts the ways to interleave the two groups. The answer
excludes `nums` itself, so it is `ways(root) - 1`.

To avoid deep recursion (a sorted `nums` builds a 1000-long chain), build the
tree with child arrays indexed by value, then evaluate it bottom-up. Every
child is inserted after its parent, so walking `nums` backwards finishes each
subtree before its root. Binomials come from precomputed factorials and
inverse factorials (Fermat's little theorem, since the modulus is prime).

```python
MOD = 10**9 + 7

def num_of_ways(nums):
    n = len(nums)
    left, right = [0] * (n + 1), [0] * (n + 1)
    root = nums[0]
    for v in nums[1:]:
        cur = root
        while True:
            if v < cur:
                if left[cur] == 0:
                    left[cur] = v
                    break
                cur = left[cur]
            else:
                if right[cur] == 0:
                    right[cur] = v
                    break
                cur = right[cur]

    fact = [1] * (n + 1)
    for i in range(1, n + 1):
        fact[i] = fact[i - 1] * i % MOD
    inv_fact = [1] * (n + 1)
    inv_fact[n] = pow(fact[n], MOD - 2, MOD)
    for i in range(n, 0, -1):
        inv_fact[i - 1] = inv_fact[i] * i % MOD

    size, ways = [0] * (n + 1), [1] * (n + 1)   # index 0 = empty subtree
    for v in reversed(nums):
        l, r = left[v], right[v]
        size[v] = size[l] + size[r] + 1
        interleave = fact[size[l] + size[r]] * inv_fact[size[l]] % MOD * inv_fact[size[r]] % MOD
        ways[v] = interleave * ways[l] % MOD * ways[r] % MOD
    return (ways[root] - 1) % MOD
```

## Complexity

- Time: O(n * h) to build the tree by insertion (O(n^2) in the worst case,
  500,000 steps for n = 1000), plus O(n) for the counting.
- Space: O(n).

## Pitfalls

- Forgetting to subtract 1: the original ordering is not counted.
- Subtracting 1 after the modulo can give -1 when `ways(root) % MOD == 0`;
  take the modulo again (or add `MOD`) after subtracting.
- Overflow: products of two residues reach about 10^18. That fits a 64-bit
  integer but not a JavaScript number; split one factor or use BigInt there.
- Recursing over the subtree lists (`[x for x in nums if x < root]`) is O(n^2)
  and recurses 1000 deep on a sorted input.
