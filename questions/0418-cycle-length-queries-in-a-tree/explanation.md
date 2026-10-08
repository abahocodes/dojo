# Approach: climb to the lowest common ancestor

Adding an edge between `a` and `b` closes a cycle consisting of the unique
tree path between them plus the new edge, so the answer is `dist(a, b) + 1`.

The distance is the number of steps from `a` and `b` up to their lowest
common ancestor. Moving up is just `v // 2` (or `v >> 1`). Of two different
labels, the larger one is at least as deep (its binary form is at least as
long), so lifting the larger label each time never overshoots the ancestor.

```python
def cycle_length_queries(n, queries):
    answer = []
    for a, b in queries:
        steps = 0
        while a != b:
            if a > b:
                a >>= 1
            else:
                b >>= 1
            steps += 1
        answer.append(steps + 1)
    return answer
```

A bit-level variant: lift the deeper label by the difference in bit lengths,
then the remaining distance is twice the bit length of `a ^ b`.

## Complexity

- Time: O(n) per query, so O(n * q) total, with n at most 30.
- Space: O(1) besides the output.

## Pitfalls

- The answer counts the new edge too: return `dist + 1`, not `dist`.
- Lifting `a` and `b` in lockstep is wrong when they are at different depths;
  always lift the larger label.
- `n` is only an upper bound on the labels; the algorithm doesn't need it.
