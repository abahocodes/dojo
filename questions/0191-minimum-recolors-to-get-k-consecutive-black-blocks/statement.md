A row of blocks is described by the string `blocks`, where `blocks[i]` is
`'W'` if the `i`-th block is white and `'B'` if it is black. In one operation
you may paint a single white block black.

Return the minimum number of operations needed so that the row contains at
least one run of `k` consecutive black blocks.

## Example 1

```
blocks = "BBWBBWWB"
k      = 5
output = 1    # paint index 2: blocks 0..4 become "BBBBB"
```

## Example 2

```
blocks = "WBBBW"
k      = 3
output = 0    # "BBB" already exists
```

## Constraints

- `1 <= k <= len(blocks) <= 100`
- `blocks[i]` is `'W'` or `'B'`
