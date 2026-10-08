# Approach: stack of survivors

Scan left to right and keep a stack of asteroids that are safe so far. A
right-mover is always pushed: anything it might hit is still to come. A
left-mover collides with right-movers at the top of the stack, one after
another:

- while the top is a smaller right-mover, it is destroyed (pop) and the
  left-mover continues;
- if the top is a right-mover of the same size, both are destroyed;
- if the top is a larger right-mover, the left-mover is destroyed;
- if the stack is empty or its top is a left-mover, the new asteroid survives
  (for now and forever) and is pushed.

The stack at the end, read bottom to top, is the answer.

```python
def asteroid_collision(asteroids):
    stack = []
    for a in asteroids:
        alive = True
        while alive and a < 0 and stack and stack[-1] > 0:
            if stack[-1] < -a:
                stack.pop()
            elif stack[-1] == -a:
                stack.pop()
                alive = False
            else:
                alive = False
        if alive:
            stack.append(a)
    return stack
```

## Complexity

- Time: O(n): each asteroid is pushed and popped at most once.
- Space: O(n) for the stack.

## Pitfalls

- Colliding a right-mover with a left-mover that is *before* it (`[-2, 3]`):
  they move apart and both survive.
- Stopping after one collision. A large left-mover can destroy several
  right-movers in a row.
- Forgetting the equal-size case, where both asteroids disappear.
