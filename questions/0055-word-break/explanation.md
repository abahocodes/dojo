# Approach: DP over prefixes

Let `ok[i]` mean "the first `i` characters of `s` can be cut into dictionary
words". The empty prefix trivially can, so `ok[0] = true`. A non-empty prefix
can be cut exactly when its last piece is some word `w` and the part before it
can be cut:

```
ok[i] = any(ok[i - L] and s[i - L:i] in words  for each word length L <= i)
```

Store the words in a hash set and loop only over the distinct word lengths
(at most 20), so each `ok[i]` costs at most 20 set lookups.

```python
def word_break(s, words):
    vocab = set(words)
    lengths = sorted({len(w) for w in words})
    ok = [True] + [False] * len(s)
    for i in range(1, len(s) + 1):
        for length in lengths:
            if length > i:
                break
            if ok[i - length] and s[i - length:i] in vocab:
                ok[i] = True
                break
    return ok[len(s)]
```

## Complexity

- Time: O(n · k · L), where `n = len(s)`, `k` is the number of distinct word
  lengths (at most 20) and `L` the cost of hashing a piece (at most 20).
- Space: O(n + total length of the words).

## Pitfalls

- Greedy matching fails: `"sunflow"` matches first in Example 1 and leaves the
  unmatchable `"erseed"`.
- Backtracking without memoisation is exponential on inputs like
  `"aaaa...ab"` with words `"a"`, `"aa"`, `"aaa"`, ...
- Words may be reused; don't remove a word from the set after using it.
- Return `ok[len(s)]`, not whether some prefix was segmented.
