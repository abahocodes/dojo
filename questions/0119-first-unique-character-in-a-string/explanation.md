# Approach: count, then scan again

Uniqueness needs the full count, so use two passes. The first counts each of
the 26 letters; the second walks `s` from the left and returns the first
position whose letter was counted once. Scanning `s` itself (not the 26
letters) in the second pass is what makes the answer the *leftmost* one.

```python
def first_uniq_char(s):
    counts = [0] * 26
    for ch in s:
        counts[ord(ch) - 97] += 1
    for i, ch in enumerate(s):
        if counts[ord(ch) - 97] == 1:
            return i
    return -1
```

## Complexity

- Time: O(n), two passes.
- Space: O(1): 26 counters.

## Pitfalls

- Returning the first letter of the alphabet that is unique instead of the
  first one in the string.
- Returning the character instead of its index.
- Calling `s.count(c)` for each position, which is O(n^2) on long inputs.
