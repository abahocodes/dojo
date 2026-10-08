# Hints

## Hint 1
Re-sorting every window costs O(k log k) per step. What do you actually need
to read off a window to get its median? Only the one or two values in the
middle.

## Hint 2
Split the window into a smaller half kept in a max-heap and a larger half kept
in a min-heap, with the smaller half holding the extra element when `k` is
odd. The median is then read from the heap tops. Adding a value is easy; the
hard part is removing the value that slides out.

## Hint 3
Heaps can't delete from the middle cheaply, so delete lazily: count the
removed value in a map, adjust the "live size" of the half it belongs to, and
only actually pop it when it reaches the top of its heap. Rebalance using the
live sizes, and prune a heap's top after every change to it.
