# Approach: test whether `s` starts with each word

A word is a prefix of `s` exactly when `s` starts with it. Test every entry
and count the hits. Because every entry is counted on its own, duplicates are
handled automatically.

```python
def count_prefixes(words, s):
    return sum(1 for word in words if s.startswith(word))
```

**Alternative:** `s` has at most 10 prefixes. Put them in a set
(`{s[:k] for k in 1..len(s)}`) and count the words found in it. A trie built
from `s` works the same way.

## Complexity

- Time: O(n · m), where `n = len(words)` and `m = len(s)` (at most 10).
- Space: O(1) extra.

## Pitfalls

- Reversing the roles: `word.startswith(s)` checks whether `s` is a prefix of
  the word, the opposite question.
- Turning `words` into a set loses duplicates, which must be counted.
- Words longer than `s` must not match. Manual loops must check lengths before
  indexing.
