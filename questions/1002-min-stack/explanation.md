# Approach: remember the minimum at every height

A stack only changes at its top, so the elements below any given element stay
put until that element is popped. That means "the minimum of the stack while
this element is on top" can be computed once, at push time, and stored next
to the element.

Each entry is a pair `(value, running_min)`:

- `push(v)`: `running_min = v` if the stack is empty, otherwise
  `min(v, top.running_min)`;
- `pop()`: drop the top pair; the pair below already holds the right minimum;
- `top()`: the top pair's `value`;
- `getMin()`: the top pair's `running_min`.

```python
class MinStack:
    def __init__(self):
        self.items = []  # (value, minimum of this entry and everything below)

    def push(self, val):
        low = min(val, self.items[-1][1]) if self.items else val
        self.items.append((val, low))

    def pop(self):
        self.items.pop()

    def top(self):
        return self.items[-1][0]

    def get_min(self):
        return self.items[-1][1]
```

A common space-saving variant keeps a second stack that only receives a value
when it is `<=` the current minimum, and pops from it when the popped value
equals its top.

## Complexity

- Time: O(1) for every operation.
- Space: O(n) for `n` elements on the stack.

## Pitfalls

- A single `min` variable can't recover the previous minimum after a pop.
- In the two-stack variant, push onto the min stack on `<=`, not `<`: with
  duplicates of the minimum, popping one copy must leave the other.
- Values span the whole 32-bit range, so don't use a sentinel like
  `2^31 - 1` as "no minimum" in a way that could collide with real data, and
  avoid tricks that store `val - min` (it can overflow 32 bits).
