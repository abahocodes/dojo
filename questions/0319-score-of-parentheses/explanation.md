# Approach: count the innermost pairs by depth

The stack version works directly from the rules: keep a stack of partial
sums, push `0` on `(`, and on `)` pop `v` and add `max(2 * v, 1)` to the new
top. The final value on the stack is the score.

Distributing the multiplications gives a shorter solution. Doubling
distributes over addition, so `(A B)` is `2A + 2B`. Unfolding all of them, the
score is a sum over the innermost pairs `"()"` only: a core at depth `d`
(enclosed by `d` other pairs) is doubled `d` times and contributes `2^d`.

```python
def score_of_parentheses(s):
    score = depth = 0
    for i, c in enumerate(s):
        if c == "(":
            depth += 1
        else:
            depth -= 1
            if s[i - 1] == "(":
                score += 1 << depth
    return score
```

## Complexity

- Time: O(n), one pass.
- Space: O(1) (the stack version uses O(n)).

## Pitfalls

- Adding `1 << depth` for every `)`: only a `)` that directly closes a `(`
  forms a core pair.
- Decrementing the depth after computing the contribution: the core `"()"` at
  the outer level has depth 0 and scores 1, so decrement first.
- Doubling an empty level in the stack version: `"()"` is 1, not 0.
- The largest answer for length 50 is `2^24`, which still fits in a 32-bit
  integer.
