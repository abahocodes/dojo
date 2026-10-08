# Hints

## Hint 1
Picture the towns as nodes and the `1`s as edges. What is a province in graph
terms?

## Hint 2
A province is a connected component. You can count components by starting a
DFS/BFS from every town you haven't visited yet, or by merging towns into
groups as you discover roads between them.

## Hint 3
Union-find: start with `n` groups. For every road `i < j` with
`is_connected[i][j] == 1`, find the roots of `i` and `j`; if they differ,
join them and subtract one from the group count. The remaining count is the
answer.
