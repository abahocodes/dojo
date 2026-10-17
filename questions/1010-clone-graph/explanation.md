# Approach: graph traversal with an original-to-copy map

A plain recursive copy loops forever on a cycle and duplicates shared nodes.
The fix is a hash map `copies` from each original node to its copy, which
serves two purposes: it marks nodes as visited, and it tells you which copy to
link to when an edge leads to an already-copied node.

Traverse from the start node (DFS with an explicit stack here; BFS works the
same way):

1. Create the copy of the start node and push the start node.
2. Pop a node `cur`. For each neighbor `nb`: if `nb` has no copy yet, create
   one and push `nb`. Then append `copies[nb]` to `copies[cur].neighbors`.
3. When the stack is empty, every node has been copied and every edge wired
   (each undirected edge is added once from each side, as in the original).

```python
def clone_graph(node):
    if node is None:
        return None
    copies = {node: Node(node.val)}
    stack = [node]
    while stack:
        cur = stack.pop()
        for nb in cur.neighbors:
            if nb not in copies:
                copies[nb] = Node(nb.val)
                stack.append(nb)
            copies[cur].neighbors.append(copies[nb])
    return copies[node]
```

The recursive form is shorter: `if node in copies: return copies[node]`,
otherwise create the copy, store it **before** recursing, and set its
neighbors to `[clone(nb) for nb in node.neighbors]`.

## Complexity

- Time: O(V + E): every node is copied once and every adjacency entry is
  processed once.
- Space: O(V) for the map and the stack.

## Pitfalls

- Storing the copy in the map only after copying its neighbors (in the
  recursive version) recurses forever on the first cycle.
- Using values as map keys works here because values are unique, but keying
  by node is the general solution.
- Appending the original neighbor (`copies[cur].neighbors.append(nb)`)
  silently links the copy back into the original graph.
- Handle the empty graph (`None`) and a lone node with no neighbors.
