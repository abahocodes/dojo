# Approach: parity masks and XOR

A sequence of letters can be rearranged into a palindrome iff at most one
letter occurs an odd number of times. Represent the odd/even status of the 26
letters as a bitmask.

Define `mask[v]` as the parity mask of the root-to-`v` path. For the path
between `u` and `v`, the edges from their lowest common ancestor up to the
root appear in both `mask[u]` and `mask[v]` and cancel under XOR, so

```
path_mask(u, v) = mask[u] ^ mask[v]
```

The path qualifies iff that XOR is `0` or a single bit. Counting such pairs is
a two-sum style hash map count: for each node, look up its own mask and the
26 masks one bit away among the nodes seen before it.

```python
from collections import defaultdict

def count_palindrome_paths(parent, s):
    n = len(parent)
    children = [[] for _ in range(n)]
    for v in range(1, n):
        children[parent[v]].append(v)
    mask = [0] * n
    order = [0]
    for v in order:  # BFS from the root
        for c in children[v]:
            mask[c] = mask[v] ^ (1 << (ord(s[c]) - 97))
            order.append(c)
    seen = defaultdict(int)
    total = 0
    for v in range(n):
        m = mask[v]
        total += seen[m]
        for b in range(26):
            total += seen.get(m ^ (1 << b), 0)
        seen[m] += 1
    return total
```

Each unordered pair is counted once, when its later node is processed.

## Complexity

- Time: O(26 n).
- Space: O(n) for the masks, the child lists and the hash map.

## Pitfalls

- A parent may have a larger index than its child, so don't fill `mask` in
  index order; traverse from the root (BFS or an iterative DFS).
- Recursion on a 10^5-node path overflows the stack in most languages.
- The count can reach about `n^2 / 2 = 5 * 10^9`, which overflows a 32-bit
  integer; use a 64-bit accumulator.
- `s[0]` is not an edge label; the root's mask is `0`.
