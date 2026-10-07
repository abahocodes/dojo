A degree program has `num_courses` courses, numbered `0` to `num_courses - 1`.
Each entry `[a, b]` in `prerequisites` means course `b` must be completed
before you may start course `a`.

You take one course at a time. Return `true` if some order lets you complete
every course, and `false` otherwise.

## Example 1

```
num_courses   = 4
prerequisites = [[1, 0], [2, 0], [3, 1], [3, 2]]
output        = true      # one valid order: 0, 1, 2, 3
```

## Example 2

```
num_courses   = 3
prerequisites = [[0, 1], [1, 2], [2, 0]]
output        = false     # 0 needs 1, 1 needs 2, 2 needs 0: nobody can start
```

## Constraints

- `1 <= num_courses <= 2000`
- `0 <= len(prerequisites) <= 5000`
- `0 <= a, b < num_courses`
- No pair `[a, b]` appears twice. An entry may have `a == b`.
