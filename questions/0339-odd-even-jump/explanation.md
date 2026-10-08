# Approach: sorted order + monotonic stack, then DP from the right

**Jump targets.** Sort the indices by `(arr[i], i)`. For index `i`, the odd
target is the smallest value `>= arr[i]` to its right, with ties broken by
the smallest index; in the sorted list, that is the first index after `i`'s
position whose array index is larger than `i`. Walking the sorted list with a
stack of indices still waiting for a target: when the current index `j` is
larger than the index on top, `j` is that index's target. Pop and repeat, then
push `j`. Indices still on the stack at the end have no target. The even
targets come from the same procedure with indices sorted by `(-arr[i], i)`
(largest value `<= arr[i]`, smallest index among ties).

**Reachability.** Let `odd[i]` mean "starting at `i` with an odd jump next, I
reach the end" and `even[i]` the same with an even jump next. Both are true at
the last index. Going right to left:
`odd[i] = even[oddNext[i]]` and `even[i] = odd[evenNext[i]]` (false when the
target does not exist). Every start begins with jump 1, so the answer counts
the indices with `odd[i]` true.

```python
def odd_even_jumps(arr):
    n = len(arr)

    def targets(order):
        nxt = [-1] * n
        stack = []
        for j in order:
            while stack and stack[-1] < j:
                nxt[stack.pop()] = j
            stack.append(j)
        return nxt

    odd_next = targets(sorted(range(n), key=lambda i: (arr[i], i)))
    even_next = targets(sorted(range(n), key=lambda i: (-arr[i], i)))
    odd = [False] * n
    even = [False] * n
    odd[-1] = even[-1] = True
    for i in range(n - 2, -1, -1):
        if odd_next[i] != -1:
            odd[i] = even[odd_next[i]]
        if even_next[i] != -1:
            even[i] = odd[even_next[i]]
    return sum(odd)
```

## Complexity

- Time: O(n log n) for the two sorts; the stack passes and the DP are O(n).
- Space: O(n).

## Pitfalls

- Simulating every start jump by jump with a linear scan per jump: O(n^2) or
  worse.
- Breaking ties wrongly. Equal values must go to the smallest index, which
  is why both sorts use the index as the secondary key in **ascending** order
  (even for the descending-value sort).
- Counting `even[i]` starts: the first jump is always odd.
- Forgetting that the last index counts as good by itself.
