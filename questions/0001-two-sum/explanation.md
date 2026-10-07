# Approach: one pass with a hash map

Keep a map from each value seen so far to its index. For each position `i` with
value `x`, the partner we need is `target - x`. If the partner is already in the
map, we have the answer. Otherwise record `x → i` and continue.

```python
def two_sum(nums, target):
    seen = {}
    for i, x in enumerate(nums):
        if target - x in seen:
            return [seen[target - x], i]
        seen[x] = i
```

## Complexity

- Time: O(n) — each element is visited once, map operations are O(1) average.
- Space: O(n) for the map.

## Pitfalls

- Inserting before checking can pair an element with itself when `x * 2 == target`.
- Duplicates are fine: with `[5, 5]`, the first 5 is stored, the second finds it.
- Sorting + two pointers also works in O(n log n), but you must keep the original
  indices, which makes it clumsier than the map.
