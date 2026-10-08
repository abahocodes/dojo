# Hints

## Hint 1
Every path has one highest node, where it turns from going up to going down.
If you fix that turning node, how long can the path be?

## Hint 2
Through a given node, the longest path goes down as far as possible on the left
and as far as possible on the right. So it has length
`height(left) + height(right)`, where height counts the nodes on the longest
downward path (an empty subtree has height `0`).

## Hint 3
Compute heights bottom-up (post-order) and, at each node, update a running
maximum with `left + right`. Visiting children before parents with an explicit
stack avoids deep recursion.
