# Approach: sort, fix one value, two pointers for the rest

After sorting, fix the smallest value `nums[i]` and look for a pair in
`nums[i + 1:]` with sum `-nums[i]` using two pointers that move toward each
other. Duplicates are skipped at both levels so each value triple is emitted
once.

```python
def three_sum(nums):
    nums = sorted(nums)
    n = len(nums)
    result = []
    for i in range(n - 2):
        if nums[i] > 0:
            break                       # smallest value positive: no more zeros
        if i > 0 and nums[i] == nums[i - 1]:
            continue                    # same first value as before
        lo, hi = i + 1, n - 1
        while lo < hi:
            s = nums[i] + nums[lo] + nums[hi]
            if s < 0:
                lo += 1
            elif s > 0:
                hi -= 1
            else:
                result.append([nums[i], nums[lo], nums[hi]])
                lo += 1
                hi -= 1
                while lo < hi and nums[lo] == nums[lo - 1]:
                    lo += 1
    return result
```

## Complexity

- Time: O(n²): sorting is O(n log n), and each of the `n` fixed values runs a
  linear two-pointer scan.
- Space: O(n) for the sorted copy (O(1) extra if sorting in place is allowed),
  not counting the output.

## Pitfalls

- Duplicates: without the two skip rules, `[-1, -1, 0, 1, 1]` yields
  `[-1, 0, 1]` several times. Deduplicating with a set of tuples works but
  hides an O(n³)-style approach behind it if you're not careful.
- Skip the fixed value by comparing with the **previous** index
  (`nums[i - 1]`), not the next, or you'll miss triples like `[-1, -1, 2]`.
- In JavaScript, `nums.sort()` sorts numbers as strings. Use
  `sort((a, b) => a - b)`.
- Empty or short input (`len(nums) < 3`) must return `[]`.
