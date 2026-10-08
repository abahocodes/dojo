# Hints

## Hint 1
Sorting all scores after every insertion costs O(n log n) per query. Notice
that values smaller than the current `k`-th largest can never become the
`k`-th largest again — new values only push the threshold up.

## Hint 2
So you only ever need the `k` largest values seen so far, and among them you
need quick access to the **smallest** one (that's the `k`-th largest overall).
Which data structure keeps a bounded collection and exposes its minimum?

## Hint 3
Keep a min-heap of size at most `k`. To insert `v`: if the heap has fewer than
`k` items, push `v`; otherwise, if `v` is larger than the top, replace the top
with `v`. After each value from `adds`, the heap's top is the answer.
