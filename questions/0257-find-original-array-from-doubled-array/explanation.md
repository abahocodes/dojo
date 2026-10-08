# Approach: pair from the smallest value upward

The smallest positive value `x` still unpaired cannot be the double of a
smaller unpaired value, so it must belong to `original`, and one copy of `2x`
must be removed to pair with it. Processing values in increasing order makes
every decision forced, which also proves the answer is unique.

Because values are bounded by `10^5`, a count array replaces sorting. Walk
`x = 1, 2, ...`: all `count[x]` remaining copies of `x` are originals and need
`count[x]` copies of `2x`; if there are fewer, fail. Zero is its own double,
so zeros pair among themselves and their count must be even. Emitting values
in this walk order produces `original` already sorted.

```python
def find_original_array(changed):
    if len(changed) % 2:
        return []
    top = max(changed)
    count = [0] * (top + 1)
    for v in changed:
        count[v] += 1
    if count[0] % 2:
        return []
    original = [0] * (count[0] // 2)
    for x in range(1, top + 1):
        c = count[x]
        if c == 0:
            continue
        if 2 * x > top or count[2 * x] < c:
            return []
        count[2 * x] -= c
        original.extend([x] * c)
    return original
```

## Complexity

- Time: O(n + V), where `V = max(changed)`.
- Space: O(V) for the counts (plus the output).

## Pitfalls

- Walking values from largest to smallest, or in arbitrary hash-map order:
  `x` might then be paired with `x / 2` when it should have been paired with
  `2x`, and a valid array is rejected.
- Treating zero like other values: `0` pairs with another `0`, so an odd
  number of zeros means no answer.
- Forgetting the odd-length shortcut, or indexing `count[2 * x]` past the end
  of the array.
