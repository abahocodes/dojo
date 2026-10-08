# Hints

## Hint 1
The pairing rule is exactly what a stack does: push each `(`, and let each
`)` pop the most recent one.

## Hint 2
Push *indices* rather than characters, so that when the scan ends you know
precisely which `(` characters were never paired.

## Hint 3
Mark a `)` for deletion when the stack is empty at the moment you meet it.
After the scan, mark every index left on the stack. Build the answer from the
unmarked characters in one more pass.
