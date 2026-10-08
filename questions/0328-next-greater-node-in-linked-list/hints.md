# Hints

## Hint 1
Comparing every node with every later node is O(n^2). Since you need answers by
position, start by copying the values into an array.

## Hint 2
Scan left to right and keep the indices that are still waiting for a larger
value. Notice that their values never increase from bottom to top — why?

## Hint 3
Use a stack of waiting indices. For each new value `v`, pop every index whose
value is smaller than `v` and record `v` as its answer, then push the current
index. Whatever is left on the stack at the end keeps the answer `0`.
