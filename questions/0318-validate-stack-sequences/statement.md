Two arrays `pushed` and `popped` of the same length contain the same
**distinct** values, each in some order.

Start with an empty stack. The values of `pushed` must be pushed in exactly
the order given, and between pushes you may pop the top of the stack any
number of times (while it is non-empty). Return `true` if some interleaving
of pushes and pops makes the values come off the stack in exactly the order
of `popped`, and `false` otherwise.

## Example 1

```
pushed = [4, 7, 1, 9]
popped = [7, 9, 1, 4]
output = true
# push 4, push 7, pop 7, push 1, push 9, pop 9, pop 1, pop 4
```

## Example 2

```
pushed = [4, 7, 1, 9]
popped = [1, 4, 9, 7]
output = false
# after 1 is popped, 7 sits above 4, so 4 cannot come off next
```

## Constraints

- `1 <= len(pushed) == len(popped) <= 1000`
- `0 <= pushed[i] <= 1000`, all values of `pushed` are distinct.
- `popped` is a permutation of `pushed`.
