# Approach: test each word's prefix

A word starts with `pref` exactly when it is at least as long as `pref` and its
first `len(pref)` characters equal `pref`. Each word is tested on its own, and
the test stops at the first mismatching character, so the whole scan reads at
most `len(pref)` characters per word.

```python
def prefix_count(words, pref):
    return sum(1 for word in words if word.startswith(pref))
```

A trie of all the words would answer many different prefixes quickly (store a
pass-through count on each node), but for a single query building it costs
more than the direct scan.

## Complexity

- Time: O(n · p), where `n = len(words)` and `p = len(pref)`.
- Space: O(1) extra.

## Pitfalls

- Checking `pref in word` counts words that merely contain `pref`, such as
  `"apart"` for `"par"`.
- Slicing `word[:len(pref)]` copies a string per word. That is fine here but
  wasteful where a built-in prefix test exists.
- In C++, `word.compare(0, p, pref)` handles words shorter than `pref`
  correctly. Hand-rolled loops must check the length first.
