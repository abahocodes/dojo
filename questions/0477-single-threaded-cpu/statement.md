A single-threaded CPU must work through `n` tasks, numbered `0` to `n - 1`.
`tasks[i] = [enqueue_time, processing_time]`: task `i` becomes available at
time `enqueue_time` and needs `processing_time` units of uninterrupted work.

The CPU follows these rules:

- When it is idle and at least one available task has not been run yet, it
  starts the one with the **shortest processing time**; ties go to the
  **smallest index**.
- Once started, a task runs to completion without interruption.
- When it is idle and nothing is available, it waits until the next task
  becomes available.
- The CPU can finish one task and start the next at the same instant. A task
  whose enqueue time equals that instant counts as available.

Return the indices of the tasks in the order the CPU processes them.

## Example 1

```
tasks  = [[1, 2], [2, 4], [3, 2], [4, 1]]
output = [0, 2, 3, 1]
```

At time 1 only task 0 is available; it runs until 3. Tasks 1 and 2 are now
available and task 2 is shorter, so it runs until 5. Task 3 (length 1) beats
task 1 (length 4).

## Example 2

```
tasks  = [[7, 10], [7, 12], [7, 5], [7, 4], [7, 2]]
output = [4, 3, 2, 0, 1]
```

## Constraints

- `1 <= len(tasks) <= 10^5`
- `1 <= enqueue_time, processing_time <= 10^9`
- The running clock can exceed the 32-bit range.
