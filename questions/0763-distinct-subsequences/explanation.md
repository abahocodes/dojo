# Approach: DP over prefixes, one row

Let `dp[i][j]` be the number of ways to form `t[:j]` as a subsequence of
`s[:i]`. Look at the last character of `s[:i]`, `s[i-1]`:

- **Skip it.** Every way to form `t[:j]` from `s[:i-1]` still works:
  `dp[i-1][j]` ways.
- **Use it** as the last character of `t[:j]`, which requires
  `s[i-1] == t[j-1]`. The rest of `t` must come from earlier positions:
  `dp[i-1][j-1]` ways.

The two groups don't overlap (they differ in whether position `i-1` is used),
so `dp[i][j]` is their sum. Base cases: `dp[i][0] = 1` (pick nothing) and
`dp[0][j] = 0` for `j > 0`.

Row `i` only reads row `i-1`, so keep one array `ways` of length
`len(t) + 1` and process `s` one character at a time. Updating `j` from
**right to left** means `ways[j-1]` is still the previous row's value when
`ways[j]` reads it.

```python
def num_distinct(s, t):
    m = len(t)
    ways = [0] * (m + 1)
    ways[0] = 1
    for ch in s:
        for j in range(m, 0, -1):
            if t[j - 1] == ch:
                ways[j] += ways[j - 1]
    return ways[m]
```

## Big intermediate values

The answer fits in a 32-bit int, but the counts for short prefixes of `t`
need not. With `s = "aaa...a"` (1000 letters) and `t = "aaa...ab"` the
answer is `0`, yet `ways[500]` reaches about `C(1000, 500) ~ 10^299`. Those
huge counts can never feed into `ways[m]`: all coefficients are
non-negative, so anything that contributes to the answer is at most the
answer. The final value is therefore correct as long as the arithmetic
doesn't crash:

- Python: integers are unbounded, nothing to do.
- Java `long`, Go `int`/`uint64`: overflow wraps silently, which is
  arithmetic modulo 2^64. Sums modulo 2^64 give the true answer modulo 2^64,
  and that is the answer itself because it fits.
- C++: signed overflow is undefined behaviour (and sanitizers abort on it),
  so use `unsigned long long` (or `unsigned int`), which wraps by definition.
- JavaScript/TypeScript: numbers lose precision above 2^53 and stay finite up
  to about 10^308, which is more than `C(1000, 500)`. The imprecise values,
  like the overflowed ones above, never reach `ways[m]`.

## Complexity

- Time: O(len(s) * len(t)).
- Space: O(len(t)) with one row (O(len(s) * len(t)) for the full table).

## Pitfalls

- Updating the row left to right: `ways[j-1]` would already include the
  current character, so one position of `s` could be used twice.
- Base case `ways[0] = 1`, not 0: forming the empty prefix is one way, and
  every count is built on it.
- Using signed 64-bit in C++ and assuming it is "big enough": the
  intermediates exceed 2^64 too.
- `len(t) > len(s)` simply gives 0; no special case needed, but a quick early
  return is fine.
