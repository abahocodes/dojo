# Approach: a set of values seen so far

Walk the list once and remember every value in a hash set. The first time a
value is already present, a duplicate exists and we can stop early. If we reach
the end, every value was unique.

```python
def contains_duplicate(nums):
    seen = set()
    for x in nums:
        if x in seen:
            return True
        seen.add(x)
    return False
```

A one-liner with the same complexity is `len(set(nums)) < len(nums)`, but it
always builds the full set instead of stopping at the first repeat.

## Complexity

- Time: O(n): each value is checked and inserted once, O(1) on average.
- Space: O(n) for the set in the worst case (all values unique).

## Pitfalls

- Sorting and comparing neighbours is O(n log n) and, if done in place,
  modifies the caller's list.
- Check membership **before** inserting, or every value will "find" itself.
- In JavaScript, use a `Set` rather than a plain object: object keys are
  strings, which is fine for integers but a habit that breaks for other types.
