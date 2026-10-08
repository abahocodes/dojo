# Approach: compare letter sets and count multisets

Repositioning lets us rearrange characters freely, so a string is fully
described by its 26 letter counts. A relabel swaps the counts of two letters
that are both present. Swaps generate every permutation, so with relabels we
can assign the counts of present letters to present letters in any order,
but a letter with count 0 always stays at 0 and a present letter can never
reach 0.

So `word1` and `word2` are close exactly when:

1. the same letters occur in both (a letter has count 0 in one iff it has
   count 0 in the other), and
2. the multiset of counts is the same (sort both count arrays and compare).

Different lengths fail condition 2 automatically.

```python
def close_strings(word1, word2):
    if len(word1) != len(word2):
        return False
    a = [0] * 26
    b = [0] * 26
    for ch in word1:
        a[ord(ch) - 97] += 1
    for ch in word2:
        b[ord(ch) - 97] += 1
    for x, y in zip(a, b):
        if (x == 0) != (y == 0):
            return False
    return sorted(a) == sorted(b)
```

## Complexity

- Time: O(n + m), counting both strings; sorting 26 numbers is constant.
- Space: O(1), two arrays of 26 counters.

## Pitfalls

- Comparing only the sorted counts: `"aab"` and `"ccd"` have counts `{2, 1}`
  in both, but they use different letters, so they are not close.
- Comparing only the letter sets: `"aaab"` and `"aabb"` use the same
  letters, but their counts `{3, 1}` and `{2, 2}` differ, so they are not
  close.
- Forgetting that strings of different lengths can never be close.
