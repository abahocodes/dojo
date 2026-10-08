# Hints

## Hint 1
Simulating the removals literally (scan, delete, rescan) can take O(n^2 / k)
passes over the string. A removal only ever affects the characters right next
to it, so try to process the string in a single left-to-right pass.

## Hint 2
While scanning, keep the "survivors so far" on a stack. When a new character
arrives, only the top of the stack can merge with it.

## Hint 3
Store pairs `(letter, run length)` on the stack. If the incoming letter equals
the top letter, increment its count and pop the pair when the count reaches
`k`; otherwise push `(letter, 1)`. At the end, expand each pair back into
`letter * count`.
