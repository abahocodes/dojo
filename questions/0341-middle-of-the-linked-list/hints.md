# Hints

## Hint 1
A two-pass solution works: count the nodes, then walk `n / 2` steps from the
head.

## Hint 2
For a single pass, use two pointers that move at different speeds. Where is
the slower pointer when the faster one reaches the end?

## Hint 3
Start `slow` and `fast` at `head`. While `fast` and `fast.next` both exist,
move `slow` one step and `fast` two steps. When the loop stops, `slow` is the
middle node (the second middle for even lengths).
