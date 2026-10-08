# Hints

## Hint 1
Trying every sequence of `L` words costs `n^L` — up to `10^12` checks. Build
the square one row at a time instead. Once rows `0..k-1` are placed, what do
you already know about row `k`?

## Hint 2
Row `k` must equal column `k`, and the first `k` characters of column `k` are
`square[0][k], square[1][k], ..., square[k-1][k]`. So row `k` has to start with
that fixed prefix. How do you quickly list all words with a given prefix?

## Hint 3
Precompute, for every prefix of every word (the empty prefix included), the
list of words that start with it — a trie or a hash map works. Backtrack: at
depth `k`, build the prefix from column `k`, try each matching word, recurse,
and record a copy of the square when `k == L`.
