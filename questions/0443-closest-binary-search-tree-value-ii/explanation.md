# Approach: two in-order iterators meeting at the target

Laid out in sorted order, the k closest values are a contiguous window that
contains the insertion point of `target`. Grow that window from the middle:
keep the next smaller candidate and the next larger candidate, take whichever
is closer, and advance that side.

Both sides are lazy in-order iterators backed by stacks:

- `pred` iterates values `<= target` in descending order (top = largest).
- `succ` iterates values `> target` in ascending order (top = smallest).

They are seeded by a single walk from the root toward `target`: a node with
`val <= target` goes onto `pred` (then move right, toward larger values),
otherwise onto `succ` (then move left). Popping from `pred` yields the next
value and pushes the popped node's left child plus that child's right spine;
`succ` mirrors this.

```python
def closest_k_values(root, target, k):
    pred, succ = [], []
    node = root
    while node:
        if node.val <= target:
            pred.append(node)
            node = node.right
        else:
            succ.append(node)
            node = node.left

    def next_smaller():
        node = pred.pop()
        cur = node.left
        while cur:
            pred.append(cur)
            cur = cur.right
        return node.val

    def next_larger():
        node = succ.pop()
        cur = node.right
        while cur:
            succ.append(cur)
            cur = cur.left
        return node.val

    result = []
    for _ in range(k):
        if not succ or (pred and target - pred[-1].val <= succ[-1].val - target):
            result.append(next_smaller())
        else:
            result.append(next_larger())
    return result
```

Simpler alternatives: collect the full in-order list and slide a window of
size k (O(n)), or push every value into a max-heap of size k keyed by distance
(O(n log k)).

## Complexity

- Time: O(h + k) amortized, where h is the tree height: O(log n + k) on a
  balanced tree. Each node is pushed and popped at most once.
- Space: O(h) for the two stacks, plus the output.

## Pitfalls

- When one iterator runs out, keep taking from the other; check emptiness
  before peeking.
- Seed the stacks with a consistent rule (`<=` to the predecessor side, `>` to
  the successor side) so that no value is produced twice.
- Compare distances as floating-point values; converting the target to an
  integer changes which value is closer.
- A recursive in-order traversal on a 10^4-deep chain can overflow the call
  stack; the stack-based iterators avoid that.
