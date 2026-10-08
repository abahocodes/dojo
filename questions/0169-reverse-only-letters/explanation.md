# Approach: two pointers that skip non-letters

Reversing the letter sequence pairs the k-th letter from the left with the
k-th letter from the right. Move `lo` right past non-letters, `hi` left past
non-letters, swap when both rest on letters, and continue until they meet.

```python
def reverse_only_letters(s: str) -> str:
    def is_letter(c):
        return "a" <= c <= "z" or "A" <= c <= "Z"

    chars = list(s)
    lo, hi = 0, len(chars) - 1
    while lo < hi:
        if not is_letter(chars[lo]):
            lo += 1
        elif not is_letter(chars[hi]):
            hi -= 1
        else:
            chars[lo], chars[hi] = chars[hi], chars[lo]
            lo += 1
            hi -= 1
    return "".join(chars)
```

## Complexity

- Time: O(n).
- Space: O(n) for the mutable copy of the string.

## Pitfalls

- Treating `[`, `\`, `]`, `^`, `_` and `` ` `` (codes 91 to 96) as letters.
  They sit between `Z` and `a` in ASCII, so a range check like `'A' <= c <= 'z'`
  is wrong.
- Reversing the whole string and then trying to put the punctuation back.
