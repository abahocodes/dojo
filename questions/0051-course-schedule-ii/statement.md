A degree program has `num_courses` courses, numbered `0` to
`num_courses - 1`. Each entry `[a, b]` in `prerequisites` means course `b`
must be completed before you may start course `a`.

You take one course at a time. Return an order in which you can complete
every course. Many orders may be valid, so to make the answer unique, return
the **lexicographically smallest** one: among all valid orders, the one with
the smallest first course, then the smallest second course, and so on.

If no order lets you complete every course, return an empty list `[]`.

## Example 1

```
num_courses   = 4
prerequisites = [[1, 0], [2, 0], [3, 1], [3, 2]]
output        = [0, 1, 2, 3]    # [0, 2, 1, 3] is also valid, but larger
```

## Example 2

```
num_courses   = 3
prerequisites = [[0, 1], [1, 2], [2, 0]]
output        = []              # the prerequisites form a cycle
```

## Constraints

- `1 <= num_courses <= 2000`
- `0 <= len(prerequisites) <= 5000`
- `0 <= a, b < num_courses`
- No pair `[a, b]` appears twice. An entry may have `a == b`.
