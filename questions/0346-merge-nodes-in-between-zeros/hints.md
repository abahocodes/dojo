# Hints

## Hint 1
Walk the list once, keeping a running sum of the values seen since the last
`0`.

## Hint 2
Every `0` after the first one closes a group. What should happen to the running
sum at that moment?

## Hint 3
Skip the leading `0`. Accumulate values into `total`; whenever you reach a `0`,
append a node holding `total` to the output (you can even reuse the `0` node
itself by overwriting its value) and reset `total` to `0`.
