# Hints

## Hint 1
Rebuilding the height after each removal is O(n) per query, too slow for
10^5 nodes and 10^4 queries. Precompute something so each query is O(1).

## Hint 2
For every node `v`, `depth[v] + height[v]` is the length of the longest
root-to-node path that passes through `v`. Removing `v` kills exactly the
paths through `v`. Every other deep path passes through some other node at
the same depth as `v`, or ends above it.

## Hint 3
Group nodes by depth. For each depth keep the largest and second-largest
`depth + height` values and which node gives the largest. If `q` is that
node, the answer is the second-largest (or `depth[q] - 1` if `q` is alone on
its level); otherwise it is the largest.
