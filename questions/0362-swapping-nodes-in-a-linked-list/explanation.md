# Approach: two pointers a fixed gap apart

After stepping `first` to the `k`-th node, there are exactly `n - k` nodes
after it. Starting a runner at `first` and a second pointer at `head`, and
moving both until the runner reaches the last node, advances the second pointer
by `n - k` steps — onto node `n - k + 1`, which is the `k`-th from the end.
Only values are swapped, so no relinking is needed.

```python
def swap_nodes(head, k):
    first = head
    for _ in range(k - 1):
        first = first.next
    runner, second = first, head
    while runner.next:
        runner = runner.next
        second = second.next
    first.val, second.val = second.val, first.val
    return head
```

## Complexity

- Time: O(n) — a single pass.
- Space: O(1).

## Pitfalls

- Off-by-one errors: both positions are 1-indexed, so `first` needs `k - 1`
  steps, not `k`.
- Assuming `first` comes before `second`; when `k > n / 2` they are reversed,
  which the value swap handles without special cases.
