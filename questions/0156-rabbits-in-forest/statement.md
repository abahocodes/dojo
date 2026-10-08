A forest is home to an unknown number of rabbits, each of a single color. You
interview some of them, asking each one: "How many *other* rabbits in the
forest have exactly your color?" Every rabbit answers truthfully, and
`answers[i]` is the reply of the `i`-th rabbit you asked.

Not every rabbit was interviewed. Return the **smallest** total number of
rabbits the forest could contain that is consistent with all the replies.

## Example 1

```
answers = [2, 2, 0]
output  = 4
# The two rabbits that said 2 can share a color: a group of 3, one of
# which was not asked. The rabbit that said 0 is alone in its color.
```

## Example 2

```
answers = [1, 1, 1]
output  = 4
# A rabbit that says 1 lives in a color group of exactly 2. Three such
# rabbits need two groups, so 4 rabbits in total.
```

## Constraints

- `1 <= len(answers) <= 1000`
- `0 <= answers[i] < 1000`
