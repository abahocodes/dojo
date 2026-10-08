# Approach: binary lifting

Answering each query by walking up parent pointers is `O(n)` per query on a
path-shaped tree. Binary lifting precomputes power-of-two jumps so that any
climb takes `O(log n)` steps.

1. **Depths.** Turn `parent` into child lists and run a BFS from the root. The
   BFS assigns `depth[child] = depth[parent] + 1`.
2. **Jump table.** `up[0][v]` is `v`'s parent, with the root pointing to
   itself. `up[k][v] = up[k-1][up[k-1][v]]`: jumping `2^k` levels is two jumps
   of `2^(k-1)`. About `log2 n` levels are enough.
3. **Query.** Make `u` the deeper node and lift it by `depth[u] - depth[v]`,
   one jump for each set bit. If now `u == v`, that node is the answer.
   Otherwise go through `k` from largest to smallest and jump both nodes when
   `up[k][u] != up[k][v]`. Jumps that would land on a common ancestor are
   skipped, so the nodes end as the two children of the LCA just below it.
   The answer is `up[0][u]`.

```python
from collections import deque


def lca_queries(parent: list[int], queries: list[list[int]]) -> list[int]:
    n = len(parent)
    children = [[] for _ in range(n)]
    root = 0
    for v, p in enumerate(parent):
        if p == -1:
            root = v
        else:
            children[p].append(v)

    # BFS from the root gives every node's depth without recursion.
    depth = [0] * n
    queue = deque([root])
    while queue:
        u = queue.popleft()
        for c in children[u]:
            depth[c] = depth[u] + 1
            queue.append(c)

    # up[k][v] = the 2^k-th ancestor of v (the root points to itself).
    log = max(1, (n - 1).bit_length())
    up = [[root if p == -1 else p for p in parent]]
    for k in range(1, log):
        prev = up[k - 1]
        up.append([prev[prev[v]] for v in range(n)])

    answers = []
    for u, v in queries:
        if depth[u] < depth[v]:
            u, v = v, u
        diff = depth[u] - depth[v]
        k = 0
        while diff:
            if diff & 1:
                u = up[k][u]
            diff >>= 1
            k += 1
        if u != v:
            for k in range(log - 1, -1, -1):
                if up[k][u] != up[k][v]:
                    u = up[k][u]
                    v = up[k][v]
            u = up[0][u]
        answers.append(u)
    return answers
```

## Complexity

- Time: `O(n log n)` to build the table, `O(log n)` per query, so
  `O((n + q) log n)` in total.
- Space: `O(n log n)` for the jump table.

## Pitfalls

- Recursive DFS for depths overflows the stack on a path of 50,000 nodes. Use
  BFS or an explicit stack.
- The root must point to itself in `up[0]` (not `-1`), or later table rows
  index with `-1`.
- In the final loop, jump only when the ancestors **differ**. Jumping when
  they are equal can overshoot the LCA to a higher common ancestor.
- After equalising depths, check `u == v` before the loop. It covers the case
  where one node is an ancestor of the other.
- Labels are arbitrary: do not assume `parent[i] < i`.
