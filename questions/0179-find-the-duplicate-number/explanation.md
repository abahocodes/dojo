# Approach: Floyd's cycle detection on `i -> nums[i]`

View each index `i` as a node with one outgoing edge to `nums[i]`. Starting
from node 0 and following edges gives a sequence `0, nums[0],
nums[nums[0]], ...`. All values are in `[1, n]`, so the walk stays among
`n + 1` nodes and must eventually repeat: it has the shape of the letter rho,
a tail followed by a cycle.

The node where the tail joins the cycle has two incoming edges: one from the
tail and one from inside the cycle. Two different indices pointing to the
same node means two equal values in `nums`, so the cycle entrance **is** the
duplicate. (Node 0 has no incoming edge, so the tail is never empty.)

Floyd's algorithm finds that entrance with O(1) memory:

1. Move `slow` one step and `fast` two steps until they meet inside the cycle.
2. Put `slow` back at the start and move both one step at a time. The point
   where they meet is the cycle entrance (the distance from the start to the
   entrance equals the distance from the meeting point to the entrance, modulo
   the cycle length).

```python
def find_duplicate(nums):
    slow = fast = nums[0]
    while True:
        slow = nums[slow]
        fast = nums[nums[fast]]
        if slow == fast:
            break
    slow = nums[0]
    while slow != fast:
        slow = nums[slow]
        fast = nums[fast]
    return slow
```

Both pointers start at `nums[0]`, i.e. one step past node 0; starting both
phases from the same node keeps the distances in step.

An alternative O(n log n) method with O(1) memory binary-searches the value:
count how many elements are `<= mid`; if the count exceeds `mid`, the
duplicate is at most `mid`.

## Complexity

- Time: O(n): both phases take at most a few laps of a walk of length n + 1.
- Space: O(1).

## Pitfalls

- Starting phase 2 from a different node than phase 1 (for example phase 1
  from index 0, phase 2 from `nums[0]`); the two pointers then fall out of
  step and the answer is wrong.
- Returning the meeting point of phase 1: it is somewhere on the cycle, not
  necessarily the entrance.
- Sorting or negating entries to mark them: both violate the read-only rule.
