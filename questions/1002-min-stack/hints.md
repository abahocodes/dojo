# Hints

## Hint 1
Keeping a single "current minimum" variable works for `push`, but what
happens when you pop the element that *is* the minimum? You would need the
minimum of everything below it, and scanning for it costs O(n).

## Hint 2
The minimum of the stack depends only on which elements are in it, and a
stack only ever changes at the top. So the minimum *at each height* never
changes once that height is reached. Could you remember it there?

## Hint 3
Store pairs `(value, minimum of this element and everything below it)`. On
`push(v)`, the new pair is `(v, min(v, previous top's minimum))`. `pop` drops
the top pair, `top` reads its value and `getMin` reads its minimum, all O(1).
