# Approach: read pointer and write pointer

Scan the array once. The read pointer visits every element; the write
pointer marks the end of the kept prefix. Every element different from
`val` is copied to the write position. Since `write <= read` at all times,
nothing is overwritten before it has been read, and the kept elements stay
in their original order.

```python
def remove_element(nums, val):
    write = 0
    for x in nums:
        if x != val:
            nums[write] = x
            write += 1
    return nums[:write]
```

## Complexity

- Time: O(n), one pass.
- Space: O(1) extra for the compaction (the returned copy aside).

## Pitfalls

- The "swap with the last element" trick removes elements in fewer writes
  but scrambles the order; this problem requires the original order.
- Deleting from a list while iterating over it skips elements.
- Empty input: return an empty array, not `null`/`None`.
