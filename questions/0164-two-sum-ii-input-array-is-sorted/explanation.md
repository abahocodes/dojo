# Approach: two pointers from both ends

Keep `lo` at the start and `hi` at the end. Look at `numbers[lo] + numbers[hi]`:

- equal to `target`: done.
- smaller: `numbers[lo]` paired with the largest remaining value is still too
  small, so `numbers[lo]` is in no valid pair. Move `lo` right.
- larger: by the mirror argument `numbers[hi]` is in no valid pair. Move `hi`
  left.

Each step discards one element that cannot be part of the answer, so the
pointers meet the answer pair before they meet each other.

```python
def two_sum_sorted(numbers: list[int], target: int) -> list[int]:
    lo, hi = 0, len(numbers) - 1
    while lo < hi:
        total = numbers[lo] + numbers[hi]
        if total == target:
            return [lo + 1, hi + 1]
        if total < target:
            lo += 1
        else:
            hi -= 1
    return []
```

## Complexity

- Time: O(n): each step moves one pointer inward.
- Space: O(1) besides the output.

## Pitfalls

- Returning 0-based indices. The answer is 1-based.
- Using the same element twice: the loop condition is `lo < hi`, not `lo <= hi`.
- Reaching for a hash map: correct, but O(n) memory and ignores the sorting.
