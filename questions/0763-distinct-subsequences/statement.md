Given two strings `s` and `t`, count the number of different ways to pick
characters out of `s`, keeping their order, so that the picked characters
spell exactly `t`.

Two ways are different when they use a different set of positions in `s`,
even if the letters are the same. In other words, count the subsequences of
`s` (as sets of indices) that are equal to `t`.

## Example 1

```
s      = "banana"
t      = "ban"
output = 3
```

The `b` must be position 0. Then pick an `a` and a later `n`: the index
triples are `(0,1,2)`, `(0,1,4)` and `(0,3,4)`. The last `a` has no `n`
after it.

## Example 2

```
s      = "aaa"
t      = "aa"
output = 3
```

Any two of the three positions work: `(0,1)`, `(0,2)`, `(1,2)`.

## Example 3

```
s      = "abc"
t      = "abcd"
output = 0
```

## Constraints

- `1 <= len(s), len(t) <= 1000`
- `s` and `t` contain only English letters.
- The answer fits in a 32-bit signed integer. Counts for shorter pieces of
  `t` computed along the way may be far larger.
