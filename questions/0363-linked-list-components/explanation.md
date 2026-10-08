# Approach: count component ends

Mark which values belong to `nums`. A component ends at a node that is marked
while its successor is missing or unmarked; every component has exactly one
such node, so counting them in a single pass gives the answer. (Counting
component *starts* — marked nodes whose predecessor is unmarked — works
equally well.)

```python
def num_components(head, nums):
    wanted = set(nums)
    count = 0
    cur = head
    while cur:
        if cur.val in wanted and (cur.next is None or cur.next.val not in wanted):
            count += 1
        cur = cur.next
    return count
```

Since values are in `[0, n - 1]`, a boolean array of size `n` is a fast
alternative to a hash set.

## Complexity

- Time: O(n + m) where `m = len(nums)`.
- Space: O(m) for the set.

## Pitfalls

- Counting the members of `nums` instead of the runs they form.
- Forgetting the component that ends at the last node of the list.
- Scanning `nums` linearly for each node, which is O(n * m).
