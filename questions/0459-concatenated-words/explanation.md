# Approach: word break over shorter words, shortest first

Every piece used to build a concatenated word is a shorter word, and a
decomposition into words strictly shorter than `w` automatically has at least
two pieces. So sort the words by length and process them in that order,
keeping a set `known` of words already processed. For each word, run the
classic word-break DP against `known`:

`can[0] = True`, and `can[end]` is true if some `start < end` has `can[start]`
and `w[start:end]` in `known`.

```python
def find_all_concatenated_words(words):
    order = sorted(range(len(words)), key=lambda i: len(words[i]))
    known = set()
    found = [False] * len(words)
    for i in order:
        w = words[i]
        if known:
            n = len(w)
            can = [False] * (n + 1)
            can[0] = True
            for end in range(1, n + 1):
                for start in range(end):
                    if can[start] and w[start:end] in known:
                        can[end] = True
                        break
            found[i] = can[n]
        known.add(w)
    return [w for i, w in enumerate(words) if found[i]]
```

Words of equal length may already be in `known`, but since the words are
distinct, none of them can equal the whole of `w`, so they never count as a
single piece.

A trie variant replaces the set: from each reachable `start`, walk the trie
along `w` and mark every `end` where a word finishes. It does the same work
without building substrings.

## Complexity

- Time: O(n * L^2) substring checks with `L <= 30` (each hashed in O(L)),
  plus O(n log n) for the sort.
- Space: O(total characters) for the set.

## Pitfalls

- Letting a word match itself as a single piece, which would report every
  word. Either exclude it or process shortest first as above.
- Plain recursion without memoization, which is exponential on inputs like
  `"aaaa...ab"`.
- Assuming exactly two pieces: `"moonsunflower"` may need three.
