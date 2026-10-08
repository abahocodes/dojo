# Approach: discard every proper suffix

A word needs its own `word#` chunk exactly when it is not a suffix of some
other word; otherwise it can be read from the end of that longer word's chunk.
Duplicates need nothing extra. So collect the distinct words, delete every
proper suffix of every word, and add up `len + 1` over what is left.

```python
def minimum_length_encoding(words):
    keep = set(words)
    for w in set(words):
        for k in range(1, len(w)):
            keep.discard(w[k:])
    return sum(len(w) + 1 for w in keep)
```

An equivalent view: insert every word reversed into a trie. Suffixes become
prefixes, so a word that is a suffix of another lies on that word's path. The
chunk words are the trie's leaves, and the answer is the sum of `depth + 1`
over the leaves.

## Complexity

- Time: O(n * k^2) with `k <= 7` the word length (k suffixes, each hashed in
  O(k)); effectively linear.
- Space: O(n * k) for the set.

## Pitfalls

- Counting duplicate words twice. Two copies of `"abc"` share one chunk.
- Removing *prefixes* instead of suffixes. `"bel"` cannot be read from
  `"bell#"`, because reading must stop at a `#`.
- Forgetting the `#` after every chunk (the `+ 1`).
- Mutating the set you are iterating over. Loop over a separate copy.
