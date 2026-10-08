A data center has a row of servers. `servers[i]` is the **weight** of server
`i`. A queue of tasks arrives over time: task `j` shows up at second `j` and
keeps a server busy for `tasks[j]` seconds.

Tasks are assigned strictly in index order. Whenever a task is waiting (it has
arrived and every earlier task has already been assigned) and at least one
server is free, it is given to the free server with the **smallest weight**;
ties go to the server with the **smallest index**. If no server is free, the
waiting tasks queue up until the earliest moment some server frees up, and then
they are handed out in order, one per free server, using the same rule.

A server that starts a task at second `t` with duration `d` becomes free again
at second `t + d` and can take a new task at that very second. Several tasks
may be assigned during the same second.

Return a list `ans` where `ans[j]` is the index of the server that runs task
`j`.

## Example 1

```
servers = [3, 3, 2]
tasks   = [1, 2, 3, 2, 1, 2]
output  = [2, 2, 0, 2, 1, 2]
```

Second 0: task 0 goes to server 2 (weight 2), free at 1. Second 1: task 1 goes
to server 2 again (free at 3). Second 2: task 2 goes to server 0 (weight 3,
smaller index than server 1), free at 5. Second 3: server 2 is back; task 3
takes it (free at 5). Second 4: task 4 goes to server 1, free at 5. Second 5:
all three are free and task 5 takes server 2.

## Example 2

```
servers = [5, 1, 4, 3, 2]
tasks   = [2, 1, 2, 4, 5, 2, 1]
output  = [1, 4, 1, 4, 1, 3, 2]
```

## Constraints

- `1 <= len(servers), len(tasks) <= 10^5`
- `1 <= servers[i], tasks[j] <= 2 * 10^5`
- Moments in time can exceed the 32-bit range; use 64-bit arithmetic for them.
