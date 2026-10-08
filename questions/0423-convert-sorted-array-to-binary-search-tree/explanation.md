# Approach: divide and conquer on the middle element

Taking the middle element as the root splits the remaining elements into two
halves whose sizes differ by at most one. Building each half the same way
gives subtrees whose heights also differ by at most one, so the whole tree is
height-balanced. The list is sorted, so everything before the middle is
smaller and everything after it is larger: the BST property holds as well.

The required tie-break, `mid = (lo + hi) // 2`, picks the left middle when a
range has even length. That makes the answer unique.

```python
def sorted_array_to_bst(nums: list[int]) -> "TreeNode | None":
    def build(lo, hi):
        if lo > hi:
            return None
        mid = (lo + hi) // 2
        return TreeNode(nums[mid], build(lo, mid - 1), build(mid + 1, hi))

    return build(0, len(nums) - 1)
```

## Complexity

- Time: `O(n)`: each element becomes exactly one node.
- Space: `O(log n)` for the recursion (the tree is balanced), plus the `O(n)`
  output.

## Pitfalls

- Using `(lo + hi + 1) // 2`, or rounding the other way in a language whose
  integer division truncates negatives, builds a different (still balanced)
  tree. That tree does not match the expected output.
- Slicing (`nums[:mid]`) copies elements at every level, which costs
  `O(n log n)`. Pass index bounds instead.
- In languages with fixed-width integers, `lo + (hi - lo) / 2` avoids overflow
  in general. With `n <= 10^4` the plain sum is also safe.
