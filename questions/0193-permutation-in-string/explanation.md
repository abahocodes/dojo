# Approach: sliding window of letter counts

Let `m = len(s1)`. A substring of `s2` is a rearrangement of `s1` exactly when
it has length `m` and the same count for each of the 26 letters. Keep `need`
(counts of `s1`) and `have` (counts of the current window of `s2`), plus
`matches`, the number of letters where `need` and `have` agree. Initially
`have` is all zero, so `matches` starts as the number of letters absent from
`s1`.

When a letter's count in `have` changes, `matches` can only change for that
letter: before the change, if it matched, it no longer does (and vice versa if
it now matches). So each slide costs O(1).

```python
def check_inclusion(s1, s2):
    m = len(s1)
    if m > len(s2):
        return False
    need = [0] * 26
    have = [0] * 26
    for c in s1:
        need[ord(c) - 97] += 1

    # have is all zero, so exactly the letters absent from s1 match so far
    matches = sum(1 for x in need if x == 0)

    def change(c, delta):
        nonlocal matches
        i = ord(c) - 97
        if have[i] == need[i]:
            matches -= 1
        have[i] += delta
        if have[i] == need[i]:
            matches += 1

    for j, c in enumerate(s2):
        change(c, 1)
        if j >= m:
            change(s2[j - m], -1)
        if j >= m - 1 and matches == 26:
            return True
    return False
```

## Complexity

- Time: O(len(s1) + len(s2)).
- Space: O(1): two arrays of 26 counts.

## Pitfalls

- Sorting every window: O(len(s2) * m log m), too slow for long inputs.
- Forgetting the case `len(s1) > len(s2)`.
- Checking the window before it has reached full length `m`.
- Starting `matches` at 26: before any letter is added, only the letters that
  do not occur in `s1` agree.
- Comparing only the set of letters, not their counts (`"aab"` vs `"abb"`).
