# Hints

## Hint 1
Checking every substring with a counter works but is O(n^2) or worse. Think
about what a single left-to-right pass with a stack of indices can tell you
when a `')'` is matched.

## Hint 2
When a `')'` at index `i` matches an `'('`, the balanced piece ending at `i`
extends left until the most recent position that could **not** be matched.
Keep that position on the stack as a "wall".

## Hint 3
Start the stack with `-1` as the wall. Push the index of every `'('`. On a
`')'`, pop. If the stack is now empty, this `')'` is unmatched: push `i` as the
new wall. Otherwise the current balanced length is `i - stack[-1]`.
