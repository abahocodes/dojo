# Approach: check every ordered pair

There are at most `50 · 49 / 2 = 1225` pairs, and each check compares at most
20 characters, so testing every pair directly is the clearest solution. For
each `j`, look at every earlier index `i` and test whether `words[j]` both
starts and ends with `words[i]`.

```python
def count_prefix_suffix_pairs(words):
    count = 0
    for j in range(len(words)):
        for i in range(j):
            if words[j].startswith(words[i]) and words[j].endswith(words[i]):
                count += 1
    return count
```

**Scaling up:** for many words, insert each word into a trie keyed by the pair
`(word[k], word[len - 1 - k])`. A word `w` has `p` as both prefix and suffix
exactly when `p`'s pair sequence is a prefix of `w`'s pair sequence. Walking
`w`'s path and adding up the end-of-word counters of the earlier words on that
path gives the answer in O(total length).

## Complexity

- Time: O(n² · L), where `n = len(words)` and `L` is the maximum word length.
- Space: O(1) extra.

## Pitfalls

- Only `i < j` counts. Testing both orders or all `i != j` overcounts.
- Equal words do form a pair: a string is its own prefix and suffix.
- Checking only the prefix, or only the suffix, is not enough (`"ab"` and
  `"aba"`).
- The prefix and the suffix may overlap (`"aba"` in `"ababa"`), so do not
  require `2 · len(words[i]) <= len(words[j])`.
