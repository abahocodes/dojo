You are given an integer array `nums` of **even** length `n` and an integer
`limit`. Every element of `nums` lies between `1` and `limit`.

Pair the array from the outside in: element `i` is paired with element
`n - 1 - i`. The array is called **complementary** when every such pair has
the same sum, that is, `nums[i] + nums[n - 1 - i]` is one common value for all
`i`.

In one move you may overwrite any single element with any integer from `1` to
`limit` (inclusive). Return the smallest number of moves that makes `nums`
complementary.

## Example 1

```
nums   = [1, 2, 4, 3]
limit  = 4
output = 1    # change the 4 to a 2: [1, 2, 2, 3], both pairs sum to 4
```

## Example 2

```
nums   = [1, 2, 2, 1]
limit  = 2
output = 2    # the pair sums are 2 and 4; make both 3 by
              # rewriting the last two elements: [1, 2, 1, 2]
```

## Constraints

- `2 <= n <= 10^5`, and `n` is even
- `1 <= nums[i] <= limit <= 10^5`
