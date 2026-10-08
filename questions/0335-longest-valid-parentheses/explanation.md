# Approach: stack of indices with a wall at the bottom

Scan left to right, keeping a stack of indices. The bottom of the stack is
always the latest "wall": the index just before the current run of characters
that might still form a balanced piece (initially `-1`). Above it sit the
indices of `'('` characters that are still open.

- On `'('`, push its index.
- On `')'`, pop. If the stack becomes empty, the popped value was the wall,
  so this `')'` has no partner; nothing ending here is balanced. Push `i` as
  the new wall.
- Otherwise the characters strictly after the new top up to `i` are balanced:
  every `'('` in that range has been matched, and they all follow the last
  open `'('` or wall. Update the best with `i - stack[-1]`.

```python
def longest_valid_parentheses(s):
    stack = [-1]
    best = 0
    for i, ch in enumerate(s):
        if ch == "(":
            stack.append(i)
        else:
            stack.pop()
            if not stack:
                stack.append(i)
            else:
                best = max(best, i - stack[-1])
    return best
```

An alternative uses O(1) space: scan left to right counting opens and
closes, recording `2 * close` whenever they are equal and resetting when
closes exceed opens, then repeat right to left with the roles swapped.

## Complexity

- Time: O(n), one pass.
- Space: O(n) for the stack in the worst case (all `'('`).

## Pitfalls

- Counting matched pairs overall: `"()(()"` has two pairs, but the longest
  balanced piece is only `2` because the unmatched `'('` splits them.
- Forgetting to join adjacent balanced pieces: in `"()()"` the answer is `4`,
  which the wall handles automatically since it does not move.
- A single left-to-right counter pass misses pieces like `"(()"`; it needs the
  right-to-left pass too.
- The empty string must return `0`.
