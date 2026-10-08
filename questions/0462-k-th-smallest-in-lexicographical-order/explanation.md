# Approach: walk the implicit prefix tree, skipping whole subtrees

Arrange `1..n` in a 10-ary tree where the children of `x` are `10x .. 10x+9`
(kept only if `<= n`), and the top level is `1..9`. A preorder traversal of
this tree lists the numbers in dictionary order. So the answer is the `k`-th
node in preorder, and we can find it without visiting every node.

**Subtree size.** The numbers that start with prefix `p` are, level by level,
`[p, p]`, `[10p, 10p+9]`, `[100p, 100p+99]`, ... each clipped to `n`. Summing
`min(n, last) - first + 1` while `first <= n` gives the size in O(log n).

**Walk.** Stand on `current = 1` with `k - 1` steps still to take:

- if the subtree under `current` has `size <= k`, every node in it comes
  before the answer: skip it (`k -= size`) and move to the sibling
  `current + 1`;
- otherwise the answer is inside: move to the first child (`current *= 10`),
  which costs one step.

```python
def find_kth_number(n, k):
    def size(p):
        count, first, last = 0, p, p
        while first <= n:
            count += min(n, last) - first + 1
            first, last = first * 10, last * 10 + 9
        return count

    current, k = 1, k - 1
    while k > 0:
        s = size(current)
        if s <= k:
            k -= s
            current += 1
        else:
            k -= 1
            current *= 10
    return current
```

## Complexity

- Time: O(log² n). Each level makes at most 10 sibling moves before
  descending, there are at most 10 levels, and each size computation is
  O(log n).
- Space: O(1).

## Pitfalls

- `first * 10` and `last * 10 + 9` exceed 32 bits when `n` is near `10^9`;
  use 64-bit integers in Java, C++ and similar languages.
- Count `k - 1` steps, not `k`: standing on `1` is already the first number.
- Generating and sorting the strings, or stepping through the order one number
  at a time, is O(n) or worse: far too slow for `n = 10^9`.
