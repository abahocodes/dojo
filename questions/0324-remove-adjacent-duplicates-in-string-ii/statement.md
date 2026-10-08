You are given a string `s` of lowercase letters and an integer `k`.

A **removal** picks a block of exactly `k` neighbouring characters that are all
the same letter and deletes it; the pieces on either side then join together,
which may create new blocks. Keep performing removals until no block of `k`
equal neighbouring characters is left.

Return the string that remains. It can be shown that the final string does not
depend on the order in which the removals are made. It may be empty.

## Example 1

```
s = "xyyyxxz", k = 3
output = "z"
```

Deleting `"yyy"` leaves `"xxxz"`; deleting `"xxx"` then leaves `"z"`.

## Example 2

```
s = "abcd", k = 2
output = "abcd"      # no two neighbours are equal
```

## Constraints

- `1 <= len(s) <= 10^5`
- `2 <= k <= 10^4`
- `s` contains only lowercase English letters.
