You are given a string `s` made only of the digits `0` to `9`. Call a
non-empty substring of `s` **awesome** if its characters can be reordered
(any number of swaps) so that it reads the same forwards and backwards.

Return the length of the longest awesome substring of `s`.

## Example 1

```
s      = "3242415"
output = 5    # "24241" can be rearranged into "24142"
```

## Example 2

```
s      = "1234"
output = 1    # every longer substring has two digits that appear once;
              # a single digit is always a palindrome
```

## Constraints

- `1 <= len(s) <= 10^5`
- `s` consists only of the characters `0` to `9`
