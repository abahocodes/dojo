You are given a string `s` made of the characters `'0'` and `'1'`. Cut it into a
non-empty left part and a non-empty right part. The **score** of the cut is

```
(number of '0's in the left part) + (number of '1's in the right part)
```

Return the highest score over all possible cuts.

## Example 1

```
s      = "010011"
output = 5    # "0100" | "11": 3 zeros + 2 ones
```

## Example 2

```
s      = "1111"
output = 3    # "1" | "111": 0 zeros + 3 ones
```

## Constraints

- `2 <= len(s) <= 500`
- every character of `s` is `'0'` or `'1'`
