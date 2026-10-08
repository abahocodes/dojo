# Approach: shrink a candidate prefix

The answer is a prefix of the first string. Take the whole first string as
the candidate, then compare it with every other string and trim it to the
longest part they agree on. After the last string, whatever is left is common
to all of them.

```python
def longest_common_prefix(strs):
    prefix = strs[0]
    for s in strs[1:]:
        i = 0
        while i < len(prefix) and i < len(s) and prefix[i] == s[i]:
            i += 1
        prefix = prefix[:i]
        if not prefix:
            break
    return prefix
```

**Alternative (vertical scan):** compare column by column: check character
`i` of every string, and stop at the first column where a string ends or
disagrees.

**Alternative (trie):** insert every string into a trie and walk down from the
root while the current node has exactly one child and no string ends there.
This is overkill for one query, but useful when many prefix queries are run
against the same set of strings.

## Complexity

- Time: O(S), where `S` is the total number of characters: each character is
  compared at most once.
- Space: O(m) for the prefix, where `m` is the length of the first string.

## Pitfalls

- An empty string anywhere in the list makes the answer `""`.
- Comparing past the end of the shorter string: guard the index with both
  lengths.
- A single string is its own longest common prefix.
