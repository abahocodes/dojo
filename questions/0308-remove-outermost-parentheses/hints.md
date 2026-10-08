# Hints

## Hint 1
Scan left to right and track the current nesting depth: `(` raises it, `)`
lowers it. A block ends exactly when the depth falls back to zero.

## Hint 2
The characters you must drop are the `(` that lifts the depth from 0 to 1 and
the `)` that brings it from 1 back to 0.

## Hint 3
Keep `(` only if the depth was already at least 1 before it; keep `)` only if
the depth is still at least 1 after it. One pass, one counter, no explicit
stack needed.
