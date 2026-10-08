# Approach: greedy simulation

Simulate the stack. After each push, pop as long as the top equals the next
value expected in `popped`. This greedy choice is safe: the values are
distinct, so the needed value is on top right now, and every later push would
cover it until those pushes are popped again, which would break the
order.

If all values have been pushed and the stack is still not empty, the next
expected value is buried under something else and can never be popped in
time, so the answer is `false`.

```python
def validate_stack_sequences(pushed, popped):
    stack = []
    j = 0
    for x in pushed:
        stack.append(x)
        while stack and stack[-1] == popped[j]:
            stack.pop()
            j += 1
    return not stack
```

The stack can also live in the prefix of `pushed` itself (overwrite
`pushed[top]`), giving O(1) extra space. The Java and Go solutions use a
separate array for clarity.

## Complexity

- Time: O(n): every value is pushed once and popped at most once.
- Space: O(n) for the stack.

## Pitfalls

- Popping only once after each push. Several pops may be due in a row.
- Checking `j == n` while `j` can run past the end: guard the inner loop with
  "stack non-empty" (once the stack is empty after the final pop, `j == n`
  and the loop stops).
- Trying to backtrack over all interleavings: exponential, and unnecessary
  because the greedy pop is always correct.
