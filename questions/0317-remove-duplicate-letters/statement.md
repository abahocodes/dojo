You are given a string `s` of lowercase English letters. Delete characters
from `s` so that every letter that occurs in `s` survives **exactly once**.
The surviving characters keep their original relative order.

Many results may be possible. Return the one that is **lexicographically
smallest**.

## Example 1

```
s      = "cabcb"
output = "abc"    # candidates include "cab", "abc", "acb"; "abc" is smallest
```

## Example 2

```
s      = "dcbadc"
output = "badc"
```

## Constraints

- `1 <= len(s) <= 10^4`
- `s` consists of lowercase English letters.
