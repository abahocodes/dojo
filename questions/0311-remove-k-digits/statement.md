You are given a non-negative integer `num`, written as a string of decimal
digits with no leading zeros, and an integer `k`. Delete exactly `k` of its
digits; the remaining digits keep their relative order and form a new number.

Return the smallest number that can be obtained, written without leading
zeros. If no digits remain, or only zeros remain, return `"0"`.

## Example 1

```
num    = "4325043"
k      = 3
output = "2043"   # delete 4, 3 and 5
```

## Example 2

```
num    = "30200"
k      = 1
output = "200"    # deleting the 3 leaves "0200", which is written "200"
```

## Constraints

- `1 <= k <= len(num) <= 10^5`
- `num` consists of digits only and has no leading zeros (`"0"` itself is
  allowed).
