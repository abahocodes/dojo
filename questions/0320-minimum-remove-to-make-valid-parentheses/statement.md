You are given a string `s` of lowercase English letters and the characters
`(` and `)`. Delete as few parentheses as possible so that the remaining
parentheses are balanced (letters never need to be removed), and return the
resulting string.

Several minimal deletions can exist, so use this rule to decide which ones to
delete:

- scan `s` from left to right; each `)` is paired with the **nearest**
  unpaired `(` to its left;
- delete every `)` that finds no partner, and every `(` that is still
  unpaired when the scan ends.

Return the string that remains (it may be empty).

## Example 1

```
s      = "a)b(c)d)"
output = "ab(c)d"    # the ")" at index 1 and the final ")" have no partner
```

## Example 2

```
s      = "(a(b)"
output = "a(b)"      # ")" pairs with the nearer "(" at index 2, so index 0 goes
```

## Constraints

- `1 <= len(s) <= 10^5`
- `s` consists of lowercase English letters, `(` and `)`.
