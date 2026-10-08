# Hints

## Hint 1
The part sizes depend only on `n` and `k`, so first count the nodes.

## Hint 2
With `base = n // k` and `extra = n % k`, the first `extra` parts get
`base + 1` nodes and the rest get `base`.

## Hint 3
Walk the list once more: for part `i`, remember the current node as the part's
head, step `size - 1` nodes forward, then cut by saving `next` and setting it
to null. A size-0 part is simply an empty list.
