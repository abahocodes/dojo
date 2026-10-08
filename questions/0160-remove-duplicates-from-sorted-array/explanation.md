# Approach: read pointer and write pointer

In a sorted array duplicates are adjacent, so a value is the first of its
kind exactly when it differs from the last value we kept. Keep the kept
values compacted at the front of the array:

- `write` is the length of the kept prefix (initially 1: the first element
  is always kept);
- `read` scans the rest. When `nums[read]` differs from `nums[write - 1]`,
  it is new, so copy it to `nums[write]` and advance `write`.

The kept prefix `nums[:write]` is the answer.

```python
def remove_duplicates(nums):
    write = 1
    for read in range(1, len(nums)):
        if nums[read] != nums[write - 1]:
            nums[write] = nums[read]
            write += 1
    return nums[:write]
```

## Complexity

- Time: O(n), one pass.
- Space: O(1) extra for the in-place compaction (the returned copy aside).

## Pitfalls

- Comparing `nums[read]` with `nums[read - 1]` is also correct here, but
  comparing with `nums[write - 1]` is the form that generalizes (e.g. to
  "keep at most two copies").
- Returning the whole array instead of the prefix: the tail still holds
  stale values.
- Using a set, which loses the order guarantee in some languages and uses
  O(n) memory.
