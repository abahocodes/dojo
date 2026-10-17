Build a stack of integers that, besides the usual operations, can report its
smallest element at any moment. Implement a class `MinStack` with:

- `MinStack()` creates an empty stack;
- `push(val)` puts `val` on top;
- `pop()` removes the top element;
- `top()` returns the top element without removing it;
- `getMin()` (`get_min()` in Python) returns the smallest element currently in
  the stack.

**Every one of these must run in O(1) time.** `pop`, `top` and `getMin` are
only called when the stack is not empty.

## How the tests call your code

Below your class the starter code has a small driver,
`min_stack(ops, vals)`, that dojo calls. It creates one `MinStack` and walks
the two lists together: `ops[i]` is `"push"`, `"pop"`, `"top"` or `"getMin"`,
and `vals[i]` is the value to push (it is `0` and ignored for the other
operations). The driver returns the answers of the `top` and `getMin` calls,
in order; `push` and `pop` add nothing to the output.

## Example 1

```
ops    = ["push", "push", "push", "getMin", "pop", "top", "getMin"]
vals   = [5,      2,      8,      0,        0,     0,     0]
output = [2, 2, 2]
```

After the three pushes the stack is `[5, 2, 8]` (top on the right), so the
minimum is `2`. Popping removes `8`; now the top is `2` and the minimum is
still `2`.

## Example 2

```
ops    = ["push", "push", "getMin", "pop", "getMin", "top"]
vals   = [3,      -1,     0,        0,     0,        0]
output = [-1, 3, 3]
```

Once `-1` is popped, the minimum goes back to `3`.

## Constraints

- `1 <= len(ops) == len(vals) <= 3 * 10^4`
- `-2^31 <= vals[i] <= 2^31 - 1`
- `pop`, `top` and `getMin` are only called on a non-empty stack.
