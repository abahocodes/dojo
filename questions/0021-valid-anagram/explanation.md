# Approach: count letters

Anagrams have identical letter counts. With a fixed alphabet of 26 lowercase
letters, a small array of counters is all we need: increment for `s`, decrement
for `t`, then check that nothing is left over.

```python
def valid_anagram(s, t):
    if len(s) != len(t):
        return False
    counts = [0] * 26
    for a, b in zip(s, t):
        counts[ord(a) - ord('a')] += 1
        counts[ord(b) - ord('a')] -= 1
    return all(c == 0 for c in counts)
```

**Alternatives:** `sorted(s) == sorted(t)` is short and correct but
O(n log n). `collections.Counter(s) == Counter(t)` is O(n) and also handles
arbitrary characters (Unicode) where a 26-slot array would not.

## Complexity

- Time: O(n) for a single pass over both strings, plus O(26) for the final check.
- Space: O(1): the counter array has a fixed size.

## Pitfalls

- Forgetting the length check: with it, a zero balance means equal counts;
  without it, zipping silently ignores the tail of the longer string.
- A string is an anagram of itself: `s == t` must return `true`.
- Comparing only the **set** of letters misses multiplicity (`"aab"` vs `"abb"`).
