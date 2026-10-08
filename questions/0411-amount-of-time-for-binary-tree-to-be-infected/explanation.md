# Approach: add parent links, then BFS from the start

Infection moves one edge per minute along every edge, so the answer is the
largest distance from the start node to any node. In other words, how many BFS
layers there are around the start, minus one.

A BFS needs all neighbors of a node, and in a binary tree a node only points
to its children. So first traverse the tree once (with an explicit stack) to
record every node's parent and to find the start node. Then run a layer-by-layer
BFS over `left`, `right` and `parent`.

```python
def amount_of_time(root, start):
    parent = {root: None}
    source = None
    stack = [root]
    while stack:
        node = stack.pop()
        if node.val == start:
            source = node
        for child in (node.left, node.right):
            if child is not None:
                parent[child] = node
                stack.append(child)

    seen = {source}
    frontier = [source]
    minutes = -1
    while frontier:
        minutes += 1
        nxt = []
        for node in frontier:
            for neighbor in (node.left, node.right, parent[node]):
                if neighbor is not None and neighbor not in seen:
                    seen.add(neighbor)
                    nxt.append(neighbor)
        frontier = nxt
    return minutes
```

**Alternative (one DFS):** a post-order DFS can return each subtree's height
and, once the start node is found below, its distance from the current node.
At every ancestor `a` of the start, the farthest node through `a` is
`dist(start, a) + 1 + height(other child of a)`. Taking the maximum of those
values and the start's own subtree height gives the answer in a single pass,
but it's trickier to get right than the BFS.

## Complexity

- Time: O(n). Every node is visited once by the traversal and once by the BFS.
- Space: O(n) for the parent map, the visited set and the frontier.

## Pitfalls

- Counting only the start node's subtree height. Infection also travels up
  through the parent and down into other branches.
- Starting the minute counter at 0 and incrementing it for the first layer too:
  the start node is infected at minute 0, so a single-node tree takes 0 minutes.
- Forgetting the visited set: without it, the parent links make the BFS bounce
  back and forth forever.
- Recursion depth: with up to `10^5` nodes in a path-shaped tree, a recursive
  parent-mapping pass overflows the stack in several languages.
