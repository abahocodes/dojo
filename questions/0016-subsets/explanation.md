# Approach: backtracking

Build subsets incrementally. Every call to `backtrack(start)` first records
the current subset, then tries extending it with each element at index
`start` or later. Since the next call only looks at later indices, each subset
is built in exactly one way: with its elements in their original order.

```python
def subsets(nums):
    result, current = [], []

    def backtrack(start):
        result.append(current[:])      # copy! current keeps changing
        for i in range(start, len(nums)):
            current.append(nums[i])
            backtrack(i + 1)
            current.pop()

    backtrack(0)
    return result
```

**Alternative (bitmask):** every integer `mask` in `0 .. 2^n - 1` stands for a
subset. Bit `i` set means `nums[i]` is included.

```python
def subsets(nums):
    n = len(nums)
    return [[nums[i] for i in range(n) if mask >> i & 1] for mask in range(1 << n)]
```

**Alternative (iterative doubling):** start from `[[]]`. For each element `x`,
add a copy of every existing subset with `x` appended.

## Complexity

- Time: O(n · 2^n): there are `2^n` subsets of average length `n/2` to copy.
- Space: O(n) recursion depth, plus the O(n · 2^n) output.

## Pitfalls

- Appending `current` itself instead of a copy leaves you with `2^n`
  references to the same (finally empty) list.
- Looping from `0` instead of `start` produces permutations and duplicates
  such as `[1, 4]` and `[4, 1]`.
- Don't forget the empty subset, and `nums = []` must return `[[]]`.
