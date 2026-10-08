You are given a list of integers `nums`. Find every **distinct triple of
values** `[a, b, c]` such that `a`, `b` and `c` sit at three different
positions of `nums` and `a + b + c == 0`.

Two triples are the same if they contain the same values (for example
`[-1, 0, 1]` and `[1, -1, 0]`), so each such combination must be reported
only once. The triples may be listed in any order, and the values inside each
triple may be listed in any order. If no triple exists, return an empty list.

## Example 1

```
nums   = [2, -3, 1, -1, 0, 1]
output = [[-3, 1, 2], [-1, 0, 1]]
# -1 + 0 + 1 can be formed with either 1, but it is reported once
```

## Example 2

```
nums   = [0, 0, 0, 0]
output = [[0, 0, 0]]
```

## Constraints

- `0 <= len(nums) <= 3000`
- `-10^5 <= nums[i] <= 10^5`
