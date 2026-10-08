# Approach: sliding window of letter counts

Let `m = len(p)`. Keep `need` (letter counts of `p`), `have` (letter counts of
the window `s[j - m + 1 .. j]`) and `matches`, the number of the 26 letters
whose two counts agree (initially, the letters absent from `p`). Adding or removing one letter changes only that
letter's agreement, so `matches` is updated in O(1). Whenever the window is
full and all 26 letters agree, its start index is an answer. Scanning left to
right produces the indices already sorted.

```python
def find_anagrams(s, p):
    m = len(p)
    result = []
    if m > len(s):
        return result
    need = [0] * 26
    have = [0] * 26
    for c in p:
        need[ord(c) - 97] += 1
    matches = sum(1 for x in need if x == 0)

    def change(i, delta):
        nonlocal matches
        if have[i] == need[i]:
            matches -= 1
        have[i] += delta
        if have[i] == need[i]:
            matches += 1

    for j, c in enumerate(s):
        change(ord(c) - 97, 1)
        if j >= m:
            change(ord(s[j - m]) - 97, -1)
        if j >= m - 1 and matches == 26:
            result.append(j - m + 1)
    return result
```

## Complexity

- Time: O(len(s) + len(p)).
- Space: O(1) besides the output: two 26-entry arrays.

## Pitfalls

- Sorting each window (O(n * m log m)), far too slow at `3 * 10^4`.
- Starting `matches` at 26 instead of at the number of letters missing from
  `p`.
- Reporting the window's end index instead of its start (`j - m + 1`).
- Overlapping matches all count: in `"aaaa"` with `p = "aa"` the answer is
  `[0, 1, 2]`.
- Returning `null` / `nil` instead of an empty array when nothing matches.
