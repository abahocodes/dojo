# Approach: union-find (disjoint set union)

Begin with `n` groups, one per machine. Process cables one at a time. A cable
between two machines in different groups merges those groups, so the count
drops by one. A cable inside a group changes nothing.

Union-find answers "which group is this machine in?" by following parent
pointers to a root. Two tricks keep the trees shallow:

- **Path halving:** while walking up, point each node at its grandparent.
- **Union by size:** hang the smaller tree under the larger one.

```python
def count_components(n, edges):
    parent = list(range(n))
    size = [1] * n

    def find(x):
        while parent[x] != x:
            parent[x] = parent[parent[x]]
            x = parent[x]
        return x

    components = n
    for a, b in edges:
        ra, rb = find(a), find(b)
        if ra == rb:
            continue
        if size[ra] < size[rb]:
            ra, rb = rb, ra
        parent[rb] = ra
        size[ra] += size[rb]
        components -= 1
    return components
```

**Alternative:** build an adjacency list and run a BFS or DFS from every
unvisited machine, counting how many searches you start. It's also O(V + E);
use an explicit stack or queue rather than recursion so a long chain of
cables can't overflow the call stack.

## Complexity

- Time: O(V + E · α(V)), where α is the inverse Ackermann function, which is
  at most 4 for any realistic input. Effectively linear.
- Space: O(V) for the parent and size arrays.

## Pitfalls

- Forgetting isolated machines. They never appear in `edges` but each one is
  a group. Starting the count at `n` handles them for free.
- Decrementing the count for every cable. A cable that closes a cycle joins
  machines that are already connected and must not change the count.
- Comparing `parent[a] == parent[b]` instead of the roots from `find`. Two
  machines can be in the same group with different direct parents.
- A recursive DFS on a 2000-machine chain can hit Python's recursion limit.
