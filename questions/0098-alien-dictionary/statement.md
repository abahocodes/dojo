An alien language uses some of the lowercase letters `a`–`z`, but in an
unknown alphabetical order. You are given `words`, a list of words from its
dictionary, already sorted by the alien order. Words are compared the usual
way: at the first position where they differ, the word with the earlier
letter comes first; if one word is a prefix of the other, the shorter word
comes first.

Return an ordering of **all the distinct letters that appear in `words`**,
as a single string, under which `words` is correctly sorted.

Usually several orderings fit (letters the words don't constrain could go
anywhere). Return the one that comes **first in normal English alphabetical
order**: compare candidate strings by their first letter in ordinary `a`–`z`
order, then the second, and so on. If no ordering fits, return `""`.

## Example 1

```
words  = ["tk", "tr", "kt", "rtt", "rf"]
output = "tfkr"
# the words tell us: k before r, t before k, t before f.
# "tkrf", "tkfr" and "tfkr" all fit; "tfkr" is first in normal a-z order.
```

## Example 2

```
words  = ["dc", "ad", "ac"]
output = "dac"     # d before a, d before c; then "dac" beats "dca"
```

## Example 3

```
words  = ["abcd", "ab"]
output = ""        # a word can never come before its own prefix
```

## Constraints

- `1 <= len(words) <= 100`
- `1 <= len(words[i]) <= 100`
- Every word contains only lowercase letters `a`–`z`.
