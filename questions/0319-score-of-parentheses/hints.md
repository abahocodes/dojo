# Hints

## Hint 1
A stack can hold the score accumulated so far at each open nesting level. On
`(` start a fresh level at 0; on `)` close the level and fold its value into
the level below.

## Hint 2
When a level closes with value `v`, it contributes `max(2 * v, 1)` to its
parent: an empty pair is worth 1, anything else is doubled.

## Hint 3
Expand the doubling: every innermost `"()"` at depth `d` (number of pairs
enclosing it) ends up multiplied by `2^d`, and nothing else scores. So add
`1 << depth` whenever a `)` immediately follows a `(`.
