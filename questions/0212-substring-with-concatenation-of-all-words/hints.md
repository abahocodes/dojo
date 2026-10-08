# Hints

## Hint 1
Every word has the same length `L`, so a candidate window is just
`len(words) * L` characters, cut at fixed positions. Compare the multiset of
its pieces with the multiset of `words` (a count map).

## Hint 2
Checking every start from scratch costs O(n * len(words)). Notice that starts
that differ by `L` share almost all of their pieces. Group the starts by
`start % L`: within one group, the pieces line up on the same grid.

## Hint 3
For each offset `r` in `0..L-1`, slide a window over the pieces
`s[r:r+L], s[r+L:r+2L], ...`. Add the piece on the right; if it is not a word,
clear the window; if it now appears too often, drop pieces from the left until
it doesn't. Whenever the window holds exactly `len(words)` pieces, record its
left edge. Sort the recorded starts at the end.
