# Hints

## Hint 1
Trying every pair is O(n²), too slow for `2 * 10^4` numbers. Think about the
answer bit by bit: a 1 in a high bit is worth more than all lower bits
together.

## Hint 2
For a fixed number `x`, the best partner differs from `x` in the highest bit
possible, then in the next highest bit among those candidates, and so on. A
structure that branches on bits, from bit 30 down to bit 0, makes that greedy
choice easy.

## Hint 3
Build a binary trie of the numbers (one level per bit, most significant
first). For each `x`, walk down the trie, taking the child with the opposite
bit whenever it exists (and adding that bit to the XOR), otherwise the same
bit. Inserting each number before querying it handles everything in one pass.
