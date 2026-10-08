# Approach: backtracking with a prefix index

Fill the square top to bottom. After rows `0..k-1` are fixed, the first `k`
letters of column `k` are fixed too: `square[0][k], ..., square[k-1][k]`.
Since row `k` must read the same as column `k`, row `k` must **start with**
that prefix. Only those words are worth trying, and if none exist the branch
dies immediately.

The later columns are checked automatically: when row `j > k` is chosen, its
first `j` letters are forced to match `square[0..j-1][j]`, i.e. everything above
the diagonal is mirrored below it.

To list words by prefix quickly, index every prefix of every word — a trie
whose nodes keep the list of words passing through them, or simply a hash map
from prefix string to word list (with `L <= 4`, each word has only 5 prefixes).

```python
def word_squares(words):
    L = len(words[0])
    by_prefix = {}
    for w in words:
        for i in range(L + 1):
            by_prefix.setdefault(w[:i], []).append(w)

    result, square = [], []

    def backtrack():
        k = len(square)
        if k == L:
            result.append(square[:])
            return
        prefix = ''.join(row[k] for row in square)
        for w in by_prefix.get(prefix, []):
            square.append(w)
            backtrack()
            square.pop()

    backtrack()
    return result
```

## Complexity

- Time: O(n · L) to build the index, plus the search. In the worst case the
  search visits every partial square, which is bounded by the number of
  answers times `L` plus dead ends; the prefix filter prunes the vast majority
  of the `n^L` raw sequences.
- Space: O(n · L²) for the index (each word stored under `L + 1` prefixes),
  plus the output.

## Pitfalls

- Words may repeat inside a square (the list `["aa"]` alone forms the square
  `["aa", "aa"]`), so don't mark words as "used".
- The empty prefix must map to all words, or the first row has no candidates.
- Append a copy of the current square, not the list you keep mutating.
- With `L = 1`, every word is a square on its own.
