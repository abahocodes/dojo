# Approach: one sliding window per offset

Let `L` be the word length, `m = len(words)` and `need` the count map of
`words`. A match is `m` consecutive length-`L` pieces whose counts equal
`need`.

Split the start positions by their remainder modulo `L`. For a fixed offset
`r`, the pieces `s[r:r+L], s[r+L:r+2L], ...` form a sequence of tokens, and a
match is a run of `m` consecutive tokens with the right counts. That is a
classic sliding window over tokens:

- Take the next token `w` on the right.
- If `w` is not in `need`, no window can contain it: empty the window and
  restart after it.
- Otherwise count it; while `w` is now over-represented, remove the leftmost
  token.
- If the window holds `m` tokens, its left edge is a match. Remove the
  leftmost token so the window can keep moving.

Each token enters and leaves the window at most once per offset, so one
offset costs O(n / L) map operations on strings of length `L`.

```python
from collections import Counter

def find_substring(s, words):
    L, m = len(words[0]), len(words)
    if m * L > len(s):
        return []
    need = Counter(words)
    result = []
    for r in range(L):
        have = Counter()
        left = r
        count = 0
        for right in range(r, len(s) - L + 1, L):
            w = s[right:right + L]
            if w not in need:
                have.clear()
                count = 0
                left = right + L
                continue
            have[w] += 1
            count += 1
            while have[w] > need[w]:
                have[s[left:left + L]] -= 1
                count -= 1
                left += L
            if count == m:
                result.append(left)
                have[s[left:left + L]] -= 1
                count -= 1
                left += L
    result.sort()
    return result
```

## Complexity

- Time: O(n * L): `L` offsets, each scanning about `n / L` tokens, and each
  token costs O(L) to slice and hash.
- Space: O(m * L) for the count maps.

## Pitfalls

- Duplicate words: `["ab", "ab"]` needs two copies of `"ab"`, so use counts,
  not a set.
- Only scanning offset 0. A match can start at any index, so every offset
  `0..L-1` needs its own window.
- Results come out grouped by offset; sort them before returning.
- If `m * L > len(s)` there is no match at all; return early.
