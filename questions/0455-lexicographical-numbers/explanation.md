# Approach: preorder walk of the implicit digit tree

Lay the numbers out as a tree where the children of `x` are `10x .. 10x + 9`
(and the roots are `1 .. 9`). Lexicographic order is exactly the preorder
traversal of this tree. The walk needs no explicit stack because the parent of
`x` is `x // 10`:

- if `cur * 10 <= n`, the next number is the first child, `cur * 10`;
- otherwise move to the next sibling `cur + 1`; when `cur` is a last child
  (ends in 9) or its sibling exceeds `n`, first climb to the parent and repeat.

```python
def lexical_order(n):
    result = []
    cur = 1
    for _ in range(n):
        result.append(cur)
        if cur * 10 <= n:
            cur *= 10
        else:
            while cur % 10 == 9 or cur + 1 > n:
                cur //= 10
            cur += 1
    return result
```

## Complexity

- Time: O(n). Each climb step undoes an earlier descent, so the total work
  across the whole walk is linear.
- Space: O(1) besides the output.

## Pitfalls

- Climbing only when `cur` ends in 9: with `n = 13`, after `13` the sibling
  `14` is too large, so you must climb to `1` and then step to `2`.
- Overflow of `cur * 10` in languages with 32-bit ints if `n` were near the
  limit; casting to 64 bits keeps it safe.
- Sorting by string is correct but O(n log n) and allocates n strings.
