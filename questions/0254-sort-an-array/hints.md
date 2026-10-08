# Hints

## Hint 1
Simple sorts (bubble, insertion, selection) are O(n^2): around 2.5 * 10^9
steps for the largest input. You need divide and conquer or a heap.

## Hint 2
Merge sort: two sorted halves can be merged into one sorted run in linear
time by repeatedly taking the smaller front element.

## Hint 3
To avoid deep recursion, merge bottom-up: first merge runs of width 1 into
runs of width 2, then 2 into 4, and so on, alternating between two buffers.
