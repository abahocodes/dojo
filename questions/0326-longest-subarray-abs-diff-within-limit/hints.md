# Hints

## Hint 1
If a window satisfies the condition, so does every window inside it. That
monotonicity makes a sliding window (two pointers) work: grow the right end,
and shrink the left end only while the window is invalid.

## Hint 2
The window is valid when `max - min <= limit`. The hard part is knowing the
current maximum and minimum as elements leave from the left. A heap or sorted
container gives O(n log n); can you do it in O(1) amortized?

## Hint 3
Keep two deques of indices: one with decreasing values (front = window max) and
one with increasing values (front = window min). Before appending index `r`,
pop from the back everything it dominates. When advancing `left`, drop a front
index once it falls out of the window.
