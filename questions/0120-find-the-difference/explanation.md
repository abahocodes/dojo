# Approach: XOR everything

XOR is its own inverse (`x ^ x == 0`) and order does not matter. Every letter
of `s` also appears in `t`, so XOR-ing the character codes of both strings
cancels all of them in pairs, leaving only the code of the extra letter. This
works even when the extra letter is a repeat, because only the number of
copies matters.

```python
def find_the_difference(s, t):
    x = 0
    for ch in s:
        x ^= ord(ch)
    for ch in t:
        x ^= ord(ch)
    return chr(x)
```

Subtracting the sum of the codes of `s` from that of `t` works the same way.

## Complexity

- Time: O(n).
- Space: O(1).

## Pitfalls

- Looking for a letter of `t` that does not occur in `s`. When the extra
  letter is a repeat (`"aab"` / `"baba"`), there isn't one.
- Comparing the strings position by position; `t` is shuffled.
- Returning a character code instead of a one-character string.
