# Hints

## Hint 1
Checking every word against the end of the stream after every character costs
O(len(stream) · total length of words). Only the most recent characters can
matter: how many of them, at most?

## Hint 2
A suffix check reads the stream backwards, starting from the newest character.
If you stored the words reversed, a suffix match becomes a prefix match. Which
structure checks all prefixes against many words at once?

## Hint 3
Build a trie of the reversed words and mark word ends. After each new character
`stream[i]`, walk the trie from the root using `stream[i], stream[i-1], ...`.
Stop with `true` at the first marked node, with `false` when a child is missing
or after `max(len(word))` steps.
