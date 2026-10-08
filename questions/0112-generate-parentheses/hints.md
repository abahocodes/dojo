# Hints

## Hint 1
Generating all `2^(2n)` strings and filtering the balanced ones works but
wastes most of its effort. Can you avoid ever building an invalid prefix?

## Hint 2
Build the string one character at a time. Track how many `(` and `)` you have
placed so far. When is adding a `(` allowed? When is adding a `)` allowed?

## Hint 3
Add `(` while `open < n`; add `)` while `close < open`. When the string reaches
length `2n`, record it. Backtracking on these two rules produces every
balanced string exactly once.
