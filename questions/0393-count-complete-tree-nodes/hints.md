# Hints

## Hint 1
A **perfect** tree of height `h` (every level full) has exactly `2^h - 1`
nodes. In a complete tree, the depth of the leftmost path tells you the height.

## Hint 2
Compare the leftmost-path depth of the root's left subtree with that of its
right subtree. If they are equal, the last level reaches into the right
subtree, so the left subtree must be perfect. If they differ, the right
subtree is perfect, one level shorter.

## Hint 3
Count the perfect subtree (plus the root) with a shift, `1 << height`, then
continue into the other subtree, which is again complete. Each step costs
O(log n) to measure depths, and there are O(log n) steps.
