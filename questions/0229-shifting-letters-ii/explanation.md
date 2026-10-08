# Approach: difference array of net shifts

A letter's final value depends only on its net shift, the count of forward
operations covering it minus the backward ones. Each operation adds a constant
`+1` or `-1` over a contiguous range, which a difference array records in O(1):
add `delta` at `start` and `-delta` at `end + 1`. Sweeping the array with a
running sum produces every index's net shift.

```python
def shifting_letters(s, shifts):
    n = len(s)
    diff = [0] * (n + 1)
    for start, end, direction in shifts:
        delta = 1 if direction == 1 else -1
        diff[start] += delta
        diff[end + 1] -= delta
    out = []
    net = 0
    for i, ch in enumerate(s):
        net += diff[i]
        out.append(chr((ord(ch) - ord("a") + net) % 26 + ord("a")))
    return "".join(out)
```

Python's `%` is always non-negative for a positive modulus. In Java, C++,
JavaScript and Go the remainder of a negative number is negative, so those
solutions use `((x % 26) + 26) % 26`.

## Complexity

- Time: O(n + m) for a string of length `n` and `m` operations.
- Space: O(n) for the difference array and the output.

## Pitfalls

- Treating `direction == 0` as "no shift" instead of a backward shift.
- Negative remainders: `-3 % 26` is `-3` in most languages, which produces a
  character before `'a'`.
- Forgetting the extra slot at `end + 1 == n`.
- Building the result with repeated string concatenation in a loop, which can
  be quadratic in some languages.
