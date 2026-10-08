A message of capital letters was encoded by replacing every letter with its
position in the alphabet: `A` becomes `1`, `B` becomes `2`, ..., `Z` becomes
`26`. The numbers were then written next to each other with no separators.

Given the digit string `s`, return **how many different letter messages**
could have produced it. Each piece of a split must be a number from `1` to
`26` written without a leading zero, so `"06"` is not a valid piece and `"0"`
on its own decodes to nothing. If `s` can't be decoded at all, return `0`.

## Example 1

```
s      = "126"
output = 3      # 1 2 6 -> ABF,  12 6 -> LF,  1 26 -> AZ
```

## Example 2

```
s      = "302"
output = 0      # "30" is above 26 and "0" alone is not a letter
```

## Constraints

- `1 <= len(s) <= 70`
- `s` contains only digits `0`-`9`
- The answer is below `2^53`, so it fits in a JavaScript number
