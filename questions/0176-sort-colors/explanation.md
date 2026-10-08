# Approach: Dutch national flag partition

Maintain the invariant

- `nums[0:low]` are all 0,
- `nums[low:mid]` are all 1,
- `nums[mid:high+1]` are unexamined,
- `nums[high+1:]` are all 2.

Initially everything is unexamined. Look at `nums[mid]`:

- `0`: swap it to position `low`; the value that comes back is a 1 (or `mid`
  itself), so advance both `low` and `mid`.
- `1`: it is already in the middle region; advance `mid`.
- `2`: swap it to position `high` and shrink `high`. The value brought to
  `mid` is unexamined, so `mid` stays put.

The unexamined region shrinks by one on every step, so the loop runs at most
`n` times.

```python
def sort_colors(nums):
    a = list(nums)
    low, mid, high = 0, 0, len(a) - 1
    while mid <= high:
        if a[mid] == 0:
            a[low], a[mid] = a[mid], a[low]
            low += 1
            mid += 1
        elif a[mid] == 1:
            mid += 1
        else:
            a[mid], a[high] = a[high], a[mid]
            high -= 1
    return a
```

## Complexity

- Time: O(n), one pass.
- Space: O(1) beyond the array being returned.

## Pitfalls

- Advancing `mid` after swapping with `high`: the incoming value might be a 0
  or a 2 and would be skipped.
- Using `mid < high` as the loop condition, which leaves the last element
  unexamined.
- In languages with unsigned indices (C++ `size_t`), `high` can drop below 0
  when the array is all 2s; use signed indices.
