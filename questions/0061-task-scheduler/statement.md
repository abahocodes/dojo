A single processor has to run a batch of jobs. Each job is labelled with one
uppercase letter, and every job takes exactly one time unit. In each time unit
the processor either runs one job or sits idle.

Jobs with the **same** label need a cool-down: if a job labelled `X` runs at
time `t`, the next `X` job may run no earlier than time `t + n + 1`. Jobs may
run in any order you like.

Given the list of job labels `tasks` and the cool-down `n`, return the fewest
time units (idle units included) needed to finish every job.

## Example 1

```
tasks  = ["A", "A", "A", "B", "B", "C"]
n      = 2
output = 7      # A B C A B _ A   (_ = idle)
```

## Example 2

```
tasks  = ["X", "Y", "X", "Y"]
n      = 0
output = 4      # no cool-down, so no idling
```

## Constraints

- `1 <= len(tasks) <= 10^4`
- Every label is a single uppercase English letter.
- `0 <= n <= 100`
