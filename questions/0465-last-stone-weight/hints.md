# Hints

## Hint 1
Simulate the game exactly as described. The only expensive part is finding
the two heaviest stones every turn. Re-sorting the pile each turn works but
costs O(n log n) per turn.

## Hint 2
You need a collection that supports "remove the largest" and "insert a new
value" quickly, over and over. Which data structure does both in O(log n)?

## Hint 3
Put every stone in a max-heap (in Python, push negated weights into `heapq`).
While the heap has at least two stones, pop `y` then `x`; if they differ, push
`y - x`. At the end return the remaining top, or `0` if the heap is empty.
