# Approach: hash map keyed by a canonical form

Every word maps to a **canonical key** that is identical for all of its
rearrangements: the sorted letters, or the tuple of 26 letter counts. Words
sharing a key form one group.

```python
def group_anagrams(words):
    groups = {}
    for w in words:
        counts = [0] * 26
        for ch in w:
            counts[ord(ch) - ord('a')] += 1
        groups.setdefault(tuple(counts), []).append(w)
    return list(groups.values())
```

Using `"".join(sorted(w))` as the key is simpler and just as accepted in an
interview; it costs O(k log k) per word instead of O(k).

## Complexity

- Time: O(n · k) with counted keys, where `k` is the maximum word length
  (O(n · k log k) with sorted keys).
- Space: O(n · k) for the keys and the groups.

## Pitfalls

- Lists aren't hashable in Python: convert the counts to a `tuple`. In
  JavaScript, join the counts with a separator (`"1#0#2..."`); without one,
  counts like `1,12` and `11,2` collide.
- The empty string is a valid word and forms its own group.
- Duplicate words stay duplicated: `["ab", "ab"]` is one group of two.
