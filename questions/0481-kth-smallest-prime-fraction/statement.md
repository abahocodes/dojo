You are given a sorted list `arr` whose first element is `1` and whose other
elements are distinct prime numbers. For every pair of positions `i < j`, form
the fraction `arr[i] / arr[j]`. All these fractions are different numbers
between 0 and 1.

Return the `k`-th smallest of them (1-based) as a two-element list
`[numerator, denominator]`.

## Example 1

```
arr    = [1, 2, 3, 5]
k      = 3
output = [2, 5]
```

The fractions in increasing order are 1/5, 1/3, 2/5, 1/2, 3/5, 2/3.

## Example 2

```
arr    = [1, 7]
k      = 1
output = [1, 7]
```

## Constraints

- `2 <= len(arr) <= 1000`
- `arr[0] == 1`, and `arr[1..]` are distinct primes in increasing order, each
  at most `3 * 10^4`
- `1 <= k <= len(arr) * (len(arr) - 1) / 2`

**Follow-up:** can you beat O(n² log n)?
