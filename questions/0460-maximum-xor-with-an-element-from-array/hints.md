# Hints

## Hint 1
Forget the limit `m` for a moment. To maximize `x ^ y`, you want the highest
bit of the result to be set, then the next one, and so on. Which data
structure lets you walk the binary representations of all candidates from the
most significant bit down, choosing a branch at each step?

## Hint 2
A binary trie over 30 bits answers "maximum XOR with `x`" in 30 steps: at each
bit, go to the child holding the opposite bit of `x` if it exists. Now, how
can you make sure the trie only ever contains elements `<= m`?

## Hint 3
Answer the queries offline. Sort `nums` ascending and the queries by `m`
(remembering their original positions). Before answering a query, insert every
number `<= m` that hasn't been inserted yet. If the trie is still empty, the
answer is `-1`.
