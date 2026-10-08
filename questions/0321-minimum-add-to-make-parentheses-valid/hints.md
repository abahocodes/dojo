# Hints

## Hint 1
Imagine matching parentheses with a stack. Which characters are left
unmatched at the end, and does each of them need its own insertion?

## Hint 2
You never need the stack's contents, only its size: the number of `(`
currently waiting for a partner.

## Hint 3
Scan once with a counter `open`. On `(` increment it. On `)` decrement it if
it is positive; otherwise this `)` needs an inserted `(`, so count one
insertion. At the end, each of the `open` leftovers needs a `)`: the answer is
`insertions + open`.
