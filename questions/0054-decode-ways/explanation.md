# Approach: DP over prefixes, Fibonacci-style

Let `ways(i)` be the number of decodings of the first `i` digits. Any decoding
of that prefix ends with a final piece of one or two digits:

- one digit `s[i-1]`: allowed when it is `1`-`9`, contributing `ways(i - 1)`
- two digits `s[i-2:i]`: allowed when they form `10`-`26` (which also rules out
  a leading zero), contributing `ways(i - 2)`

`ways(0) = 1` (the empty prefix has one decoding: nothing). Keep two rolling
values:

```python
def num_decodings(s):
    prev, curr = 1, 1 if s[0] != "0" else 0  # ways(0), ways(1)
    for i in range(2, len(s) + 1):
        nxt = 0
        if s[i - 1] != "0":
            nxt += curr
        if 10 <= int(s[i - 2:i]) <= 26:
            nxt += prev
        prev, curr = curr, nxt
    return curr
```

Once `curr` becomes `0` it can only recover through `prev`; when both are `0`
the answer stays `0`, which correctly handles strings like `"100"`.

## Complexity

- Time: O(n).
- Space: O(1).

## Pitfalls

- `"0"`, a leading `0`, or `"00"` anywhere makes the whole string undecodable.
- `"30"`, `"40"`, ...: a zero must pair with a preceding `1` or `2`.
- Two-digit pieces like `"06"` are invalid; check `>= 10`, not just `<= 26`.
- Plain recursion is exponential: `"111...1"` of length 70 has about 3 × 10^14
  decodings, so you can't enumerate them.
