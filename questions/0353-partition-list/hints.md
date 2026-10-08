# Hints

## Hint 1
Sorting would break the "keep the original order within each group" rule. Think
about building the answer from two separate pieces.

## Hint 2
Walk the list once and append each node to one of two new lists — "small" or
"large" — depending on how it compares to `x`. Appending keeps the order.

## Hint 3
Use two dummy heads, `small` and `large`, with tail pointers. After the walk,
set `large_tail.next = None` (the last large node may still point into the old
list), link `small_tail.next = large.next`, and return `small.next`.
