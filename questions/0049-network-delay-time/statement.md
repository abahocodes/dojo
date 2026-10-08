A network has `n` servers, numbered `1` to `n`. Each entry `[u, v, w]` in
`times` is a one-way link: a message sent from server `u` arrives at server
`v` after `w` milliseconds.

Server `k` broadcasts an alert at time `0`, and every server forwards it along
all of its outgoing links the moment it first receives it. Return the time at
which the **last** server receives the alert. If some server never receives
it, return `-1`.

## Example 1

```
times  = [[1, 2, 4], [1, 3, 1], [3, 2, 2], [2, 4, 1]]
n      = 4
k      = 1
output = 4      # 3 at t=1, 2 at t=3 (via 3, not the direct 4 ms link), 4 at t=4
```

## Example 2

```
times  = [[1, 2, 5]]
n      = 3
k      = 1
output = -1     # nothing links to server 3
```

## Constraints

- `1 <= n <= 1000`
- `1 <= k <= n`
- `0 <= len(times) <= 6000`
- `1 <= u, v <= n` and `u != v`
- `0 <= w <= 100`
- No pair `(u, v)` appears twice.
