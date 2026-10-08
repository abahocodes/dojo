# Approach: two pointers

Keep `i` in `word` and `j` in `abbr`.

- If `abbr[j]` is a letter, it must match `word[i]`; advance both.
- If `abbr[j]` is a digit, it starts a number. A leading `0` is invalid. Read
  the full number `k` and advance `i` by `k` (skipping those letters).
- If `i` runs past the end of `word`, the abbreviation is too long.

The abbreviation is valid exactly when both pointers end at the ends of their
strings simultaneously.

```python
def valid_word_abbreviation(word, abbr):
    i = j = 0
    while i < len(word) and j < len(abbr):
        c = abbr[j]
        if c.isdigit():
            if c == "0":
                return False
            k = 0
            while j < len(abbr) and abbr[j].isdigit():
                k = k * 10 + int(abbr[j])
                j += 1
            i += k
        else:
            if word[i] != c:
                return False
            i += 1
            j += 1
    return i == len(word) and j == len(abbr)
```

In the fixed-width languages the number can reach about 2 * 10^9, which is
close to the 32-bit limit, so the solutions there accumulate into a 64-bit
value (or stop as soon as it exceeds the word length).

## Complexity

- Time: O(len(word) + len(abbr)).
- Space: O(1).

## Pitfalls

- Accepting `"0"` or `"01"`: a zero-length piece is not allowed and leading
  zeros are not allowed.
- Reading only one digit: `"l10n"` skips ten letters, not one then zero.
- Returning `true` when only one string is used up: `"a2"` for `"apple"`
  leaves letters unmatched, and `"a9"` overshoots.
- Integer overflow when adding a large number to the index in Java, C++ or Go.
