# Approach: two pointers (read and write)

Scan with `read`. The write pointer `w` marks where the next non-zero belongs.
Everything before `w` is the non-zeros seen so far, in order; everything
between `w` and `read` is zeros. Swapping a non-zero at `read` with the slot at
`w` keeps both facts true.

```python
def move_zeroes(nums):
    w = 0
    for read in range(len(nums)):
        if nums[read] != 0:
            nums[w], nums[read] = nums[read], nums[w]
            w += 1
    return nums
```

A two-pass variant copies the non-zeros forward (`nums[w] = nums[read]`) and
then fills `nums[w:]` with zeros; it does the same O(n) work.

## Complexity

- Time: O(n): one pass over the list.
- Space: O(1) extra.

## Pitfalls

- Removing zeros inside a loop (`nums.remove(0)` / `splice`) is O(n) per
  removal, O(n²) overall, and shifts the indices you are iterating over.
- Sorting zeros to the end (e.g. with a key) is O(n log n) and not in place.
- The relative order of the **non-zero** values must not change, so moving
  values from the back into the zeros' slots is wrong.
- Return the list: the tests check the returned value.
